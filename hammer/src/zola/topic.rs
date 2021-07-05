//! This module covers the topic from a Zola point of view.
use anyhow::Result;
use rusqlite::Transaction;
use serde::{Deserialize, Serialize};
use std::fmt;

use super::{StandardId, TopicId};
use crate::cache::records::*;
use crate::cache::Cache;
use crate::checksum::{Checksum, Digest, Hasher};
use crate::report;
use crate::resource::Resource;

#[derive(Debug, Clone)]
pub struct Topic {
    pub metadata: Metadata,
    pub content: String,
}

impl Topic {
    pub fn id(&self) -> &TopicId {
        &self.metadata.extra.id
    }

    pub fn checksum(&self) -> Checksum {
        self.into()
    }

    pub fn path(&self) -> String {
        format!("{}.md", self.id())
    }
}

impl fmt::Display for Topic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let metadata = toml::to_string(&self.metadata).expect("metadata to serialize as TOML");

        writeln!(f, "+++")?;
        write!(f, "{}", &metadata)?;
        writeln!(f, "+++")?;
        write!(f, "{}", &self.content)
    }
}

impl Digest for Topic {
    fn digest(&self, hasher: &mut Hasher) {
        self.metadata.digest(hasher);
        self.content.digest(hasher);
    }
}

impl From<&Topic> for Checksum {
    fn from(resource: &Topic) -> Checksum {
        let mut hasher = Hasher::new();
        resource.digest(&mut hasher);

        hasher.finalize()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metadata {
    title: String,
    weight: u32,
    slug: String,
    template: String,
    extra: MetadataExtra,
}

impl Digest for Metadata {
    fn digest(&self, hasher: &mut Hasher) {
        self.extra.id.digest(hasher);
        self.title.digest(hasher);
        self.extra.ordinal.digest(hasher);
        self.extra
            .standards
            .iter()
            .map(|x| x.id.clone())
            .collect::<Vec<_>>()
            .digest(hasher);
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MetadataExtra {
    /// A local unique identifier for the standard.
    #[serde(rename = "identifier")]
    pub id: StandardId,
    pub ordinal: u32,
    pub standards: Vec<RelatedStandard>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedStandard {
    id: String,
    name: String,
    url: String,
    status: String,
    review_date: String,
}

impl From<TopicStandardRecord> for RelatedStandard {
    fn from(record: TopicStandardRecord) -> RelatedStandard {
        RelatedStandard {
            id: record.id.clone(),
            name: record.name,
            url: format!("/standards/{}", record.id),
            status: record.status,
            review_date: record.review_date,
        }
    }
}

impl Digest for RelatedStandard {
    fn digest(&self, hasher: &mut Hasher) {
        self.id.digest(hasher);
    }
}

impl Resource<Topic> for Cache {
    fn get(&mut self, id: &str) -> Result<Option<Topic>> {
        let tx = self.conn.transaction()?;
        let mut result = None;

        if let Some(record) = TopicRecord::select(&tx, id)? {
            result = Some(into_resource(&tx, record)?);
        }

        &self
            .report
            .log(report::Action::Get, report::Entity::Topic, id, "");

        tx.commit()?;

        Ok(result)
    }

    fn add(&mut self, _resource: &Topic) -> Result<()> {
        unimplemented!()
    }

    fn drop(&mut self, _id: &str) -> Result<Option<Topic>> {
        unimplemented!()
    }
}

pub fn get_all(cache: &mut Cache, theme_id: &str) -> Result<Vec<Topic>> {
    let tx = cache.transaction()?;
    let records = TopicRecord::select_by_theme(&tx, theme_id)?;
    let mut result = Vec::new();

    for record in records {
        let resource = into_resource(&tx, record)?;

        result.push(resource);
    }

    tx.commit()?;

    Ok(result)
}

fn into_resource(tx: &Transaction, record: TopicRecord) -> Result<Topic> {
    let standards = TopicStandardRecord::select(tx, &record.id)?;
    let extra = MetadataExtra {
        id: record.id.clone(),
        ordinal: record.ordinal.clone(),
        standards: standards.into_iter().map(|r| r.into()).collect(),
    };
    let metadata = Metadata {
        title: record.name,
        weight: record.ordinal,
        slug: record.id,
        template: "topic.html".to_string(),
        extra,
    };
    let resource = Topic {
        metadata,
        content: record.description,
    };

    Ok(resource)
}
