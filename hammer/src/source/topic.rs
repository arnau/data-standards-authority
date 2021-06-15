//! This module covers the taxonomical topic from an input point of view.
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::{split_content, TopicId};
use crate::cache::{Cache, TopicRecord};
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
        &self.metadata.id
    }

    pub fn checksum(&self) -> Checksum {
        self.into()
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

impl FromStr for Topic {
    type Err = anyhow::Error;

    fn from_str(blob: &str) -> Result<Self, Self::Err> {
        let (frontmatter, content) = split_content(blob)?;
        let metadata = serde_yaml::from_str(frontmatter)?;

        Ok(Self {
            metadata,
            content: content.into(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "identifier")]
    pub id: TopicId,
    pub name: String,
    pub theme: String,
    pub ordinal: u32,
}

impl Digest for Metadata {
    fn digest(&self, hasher: &mut Hasher) {
        self.id.digest(hasher);
        self.name.digest(hasher);
        self.theme.digest(hasher);
        self.ordinal.digest(hasher);
    }
}

impl Resource<Topic> for Cache {
    fn get(&mut self, id: &str) -> Result<Option<Topic>> {
        let tx = self.conn.transaction()?;
        let mut result = None;

        if let Some(record) = TopicRecord::select(&tx, id)? {
            let metadata = Metadata {
                id: record.id,
                name: record.name,
                theme: record.theme_id,
                ordinal: record.ordinal,
            };
            let content = record.description;
            let topic = Topic { metadata, content };

            result = Some(topic)
        }

        &self
            .report
            .log(report::Action::Get, report::Entity::Topic, id, "");

        tx.commit()?;

        Ok(result)
    }

    fn add(&mut self, resource: &Topic) -> Result<()> {
        let tx = self.conn.transaction()?;
        let checksum = resource.checksum().to_string();

        if let Some(record) = TopicRecord::select(&tx, resource.id())? {
            if record.checksum != checksum {
                TopicRecord::delete(&tx, resource.id())?;
                TopicRecord::insert(&tx, &resource.into())?;
            }
        } else {
            TopicRecord::insert(&tx, &resource.into())?;
        }

        Cache::insert_trailmark(&tx, &checksum, "topic", &self.timestamp)?;

        &self.report.log(
            report::Action::Add,
            report::Entity::Topic,
            resource.id(),
            "",
        );

        tx.commit()?;

        Ok(())
    }

    fn drop(&mut self, id: &str) -> Result<Option<Topic>> {
        let resource = self.get(id)?;
        let tx = self.conn.transaction()?;

        if resource.is_some() {
            TopicRecord::delete(&tx, id)?;
        }

        &self
            .report
            .log(report::Action::Prune, report::Entity::Topic, id, "");

        tx.commit()?;

        Ok(resource)
    }
}

impl From<&Topic> for TopicRecord {
    fn from(resource: &Topic) -> Self {
        TopicRecord {
            id: resource.id().clone(),
            checksum: resource.checksum().to_string(),
            name: resource.metadata.name.clone(),
            description: resource.content.clone(),
            theme_id: resource.metadata.theme.clone(),
            ordinal: resource.metadata.ordinal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::str::FromStr;

    #[test]
    fn gad() -> Result<()> {
        let raw = r#"---
type: topic
identifier: geospatial
name: Geospatial
theme: reference-data
ordinal: 1
---
Standards that focus on geospatial information. For example, Local Authority boundaries, addresses or land descriptors."#;
        let resource = Topic::from_str(raw)?;
        let mut cache = Cache::connect(":memory:")?;

        cache.add(&resource)?;

        let cached: Topic = cache.get(&resource.id())?.expect("topic doesn't exist");

        assert_eq!(&resource.checksum(), &cached.checksum());

        let _: Option<Topic> = cache.drop(&resource.id())?;
        let void: Option<Topic> = cache.get(&resource.id())?;

        assert!(void.is_none());

        Ok(())
    }
}
