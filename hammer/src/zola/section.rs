//! This module covers the section from a Zola point of view.
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::cache::records::*;
use crate::cache::Cache;
use crate::checksum::{Checksum, Digest, Hasher};
use crate::markdown;
use crate::report;
use crate::resource::Resource;

#[derive(Debug, Clone)]
pub struct Section {
    pub metadata: Metadata,
    pub content: String,
}

impl Section {
    pub fn id(&self) -> &str {
        &self.metadata.extra.id
    }

    pub fn checksum(&self) -> Checksum {
        self.into()
    }

    pub fn path(&self) -> String {
        format!("{}/_index.md", self.id())
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

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let metadata = toml::to_string(&self.metadata).expect("metadata to serialize as TOML");

        writeln!(f, "+++")?;
        write!(f, "{}", &metadata)?;
        writeln!(f, "+++")?;
        write!(f, "{}", &self.content)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metadata {
    title: String,
    slug: String,
    template: String,
    extra: MetadataExtra,
}

impl Digest for Metadata {
    fn digest(&self, hasher: &mut Hasher) {
        self.title.digest(hasher);
        self.slug.digest(hasher);
        self.template.digest(hasher);
        self.extra.digest(hasher);
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MetadataExtra {
    id: String,
    resource_type: String,
}

impl Digest for MetadataExtra {
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
            let extra = MetadataExtra {
                id: record.id.clone(),
                resource_type: record.resource_type.clone(),
            };
            let (title, content) = markdown::split_title(&record.content)?;

            let metadata = Metadata {
                title,
                slug: format!("{}", record.id),
                template: "standard-set.html".to_string(),
                extra,
            };
            let resource = Section { metadata, content };

            result = Some(resource);
        }

        &self
            .report
            .log(report::Action::Get, report::Entity::Section, id, "");

        tx.commit()?;

        Ok(result)
    }

    fn add(&mut self, _resource: &Section) -> Result<()> {
        unimplemented!()
    }

    fn drop(&mut self, _id: &str) -> Result<Option<Section>> {
        unimplemented!()
    }
}
