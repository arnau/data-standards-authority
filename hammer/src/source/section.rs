//! This module covers the section from an input point of view.
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::split_content;
use crate::cache::records::*;
use crate::cache::Cache;
use crate::checksum::{Checksum, Digest, Hasher};
use crate::report;
use crate::resource::Resource;

#[derive(Debug, Clone)]
pub struct Section {
    pub metadata: Metadata,
    pub content: String,
}

impl Section {
    pub fn id(&self) -> &str {
        &self.metadata.id
    }

    pub fn checksum(&self) -> Checksum {
        self.into()
    }
}

impl Digest for Section {
    fn digest(&self, hasher: &mut Hasher) {
        self.metadata.digest(hasher);
        self.content.digest(hasher);
    }
}

impl From<&Section> for Checksum {
    fn from(resource: &Section) -> Checksum {
        let mut hasher = Hasher::new();
        resource.digest(&mut hasher);

        hasher.finalize()
    }
}

impl FromStr for Section {
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metadata {
    /// A local unique identifier for the standard.
    #[serde(rename = "identifier")]
    id: String,
    resource_type: String,
}

impl Digest for Metadata {
    fn digest(&self, hasher: &mut Hasher) {
        self.id.digest(hasher);
        self.resource_type.digest(hasher);
    }
}

impl Resource<Section> for Cache {
    fn get(&mut self, id: &str) -> Result<Option<Section>> {
        let tx = self.conn.transaction()?;
        let mut result = None;

        if let Some(record) = SectionRecord::select(&tx, id)? {
            let metadata = Metadata {
                id: record.id,
                resource_type: record.resource_type,
            };
            let resource = Section {
                metadata,
                content: record.content,
            };

            result = Some(resource);
        }

        &self
            .report
            .log(report::Action::Get, report::Entity::Section, id, "");

        tx.commit()?;

        Ok(result)
    }

    fn add(&mut self, resource: &Section) -> Result<()> {
        let tx = self.conn.transaction()?;
        let checksum = resource.checksum().to_string();

        if let Some(record) = SectionRecord::select(&tx, resource.id())? {
            if record.checksum != checksum {
                SectionRecord::delete(&tx, resource.id())?;
                SectionRecord::insert(&tx, &resource.into())?;
            }
        } else {
            SectionRecord::insert(&tx, &resource.into())?;
        }

        Cache::insert_trailmark(&tx, &checksum, "section", &self.timestamp)?;

        &self.report.log(
            report::Action::Add,
            report::Entity::Section,
            resource.id(),
            "",
        );

        tx.commit()?;

        Ok(())
    }

    fn drop(&mut self, id: &str) -> Result<Option<Section>> {
        let resource = self.get(&id)?;
        let tx = self.conn.transaction()?;

        if resource.is_some() {
            SectionRecord::delete(&tx, id)?;
        }

        &self
            .report
            .log(report::Action::Prune, report::Entity::Section, id, "");

        tx.commit()?;

        Ok(resource)
    }
}

impl From<&Section> for SectionRecord {
    fn from(resource: &Section) -> Self {
        SectionRecord {
            id: resource.metadata.id.clone(),
            checksum: resource.checksum().to_string(),
            resource_type: resource.metadata.resource_type.clone(),
            content: resource.content.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::str::FromStr;

    #[test]
    fn gad_section() -> Result<()> {
        let raw = r#"---
type: section
identifier: standard
resource_type: standard
---
# Standards catalogue

Lorem ipsum"#;
        let original = Section::from_str(raw)?;
        let mut cache = Cache::connect(":memory:")?;
        // let mut cache = Cache::connect("./caca")?;

        cache.add(&original)?;

        let cached: Section = cache.get(&original.id())?.expect("section doesn't exist");

        assert_eq!(&original.checksum(), &cached.checksum());

        let _: Option<Section> = cache.drop(original.id())?;
        let void: Option<Section> = cache.get(original.id())?;

        assert!(void.is_none());

        Ok(())
    }
}
