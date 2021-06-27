//! This module covers the guidance page from a Zola point of view.
use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::Transaction;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use super::{GuidanceId, Organisation, Url};
use crate::cache::records::*;
use crate::cache::Cache;
use crate::checksum::{Checksum, Digest, Hasher};
use crate::markdown;
use crate::report;
use crate::resource::Resource;
use crate::Status;

#[derive(Debug, Clone)]
pub struct Guidance {
    pub metadata: Metadata,
    pub content: String,
}

impl Guidance {
    pub fn id(&self) -> &GuidanceId {
        &self.metadata.extra.id
    }

    pub fn checksum(&self) -> Checksum {
        self.into()
    }

    pub fn path(&self) -> String {
        format!("{}.md", self.id())
    }
}

impl fmt::Display for Guidance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let metadata = toml::to_string(&self.metadata).expect("metadata to serialize as TOML");

        writeln!(f, "+++")?;
        write!(f, "{}", &metadata)?;
        writeln!(f, "+++")?;
        write!(f, "{}", &self.content)
    }
}

impl Digest for Guidance {
    fn digest(&self, hasher: &mut Hasher) {
        self.metadata.digest(hasher);
        self.content.digest(hasher);
    }
}

impl From<&Guidance> for Checksum {
    fn from(resource: &Guidance) -> Checksum {
        let mut hasher = Hasher::new();
        resource.digest(&mut hasher);

        hasher.finalize()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metadata {
    title: String,
    date: DateTime<Utc>,
    slug: String,
    template: String,
    extra: MetadataExtra,
}

impl Digest for Metadata {
    fn digest(&self, hasher: &mut Hasher) {
        self.extra.id.digest(hasher);
        self.title.digest(hasher);
        self.extra.status.digest(hasher);
        self.extra.creation_date.digest(hasher);
        self.extra.update_date.digest(hasher);
        self.extra.publication_date.digest(hasher);
        self.extra.maintainer.id().digest(hasher);
        self.extra.canonical_url.digest(hasher);
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MetadataExtra {
    /// A local unique identifier for the standard.
    #[serde(rename = "identifier")]
    pub id: GuidanceId,
    pub status: Status,
    pub creation_date: String,
    pub update_date: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publication_date: Option<String>,
    /// The URL to the publication in GOV.UK.
    pub canonical_url: Option<Url>,
    /// The organisation maintaining the specification.
    pub maintainer: Organisation,
}

impl Resource<Guidance> for Cache {
    fn get(&mut self, id: &str) -> Result<Option<Guidance>> {
        let tx = self.conn.transaction()?;
        let mut result = None;

        if let Some(record) = GuidanceRecord::select(&tx, id)? {
            result = Some(into_resource(&tx, record)?);
        }

        &self
            .report
            .log(report::Action::Get, report::Entity::Guidance, id, "");

        tx.commit()?;

        Ok(result)
    }

    fn add(&mut self, _resource: &Guidance) -> Result<()> {
        unimplemented!()
    }

    fn drop(&mut self, _id: &str) -> Result<Option<Guidance>> {
        unimplemented!()
    }
}

pub fn get_all(cache: &mut Cache) -> Result<Vec<Guidance>> {
    let tx = cache.transaction()?;
    let records = GuidanceRecord::select_all(&tx)?;
    let mut result = Vec::new();

    for record in records {
        let resource = into_resource(&tx, record)?;

        result.push(resource);
    }

    tx.commit()?;

    Ok(result)
}

fn into_resource(tx: &Transaction, record: GuidanceRecord) -> Result<Guidance> {
    let id = &record.id;
    let maintainer =
        OrganisationRecord::select(&tx, &record.maintainer_id)?.expect("maintainer to exist");
    let extra = MetadataExtra {
        id: record.id.clone(),
        status: record.status,
        creation_date: record.creation_date.clone(),
        update_date: record.update_date,
        publication_date: record.publication_date,
        canonical_url: record.canonical_url,
        maintainer: maintainer.into(),
    };
    let date = FromStr::from_str(&format!("{}T00:00:00Z", &record.creation_date))?;
    let (title, content) = markdown::split_title(&record.content)?;
    let metadata = Metadata {
        title,
        date,
        slug: record.id,
        template: "guidance.html".to_string(),
        extra,
    };
    let resource = Guidance { metadata, content };

    Ok(resource)
}
