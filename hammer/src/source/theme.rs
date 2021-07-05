//! This module covers the theme from a source point of view.
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::{split_content, ThemeId};
use crate::cache::{Cache, ThemeRecord};
use crate::checksum::{Checksum, Digest, Hasher};
use crate::report;
use crate::resource::Resource;

#[derive(Debug, Clone)]
pub struct Theme {
    pub metadata: Metadata,
    pub content: String,
}

impl Theme {
    pub fn id(&self) -> &ThemeId {
        &self.metadata.id
    }

    pub fn checksum(&self) -> Checksum {
        self.into()
    }
}

impl Digest for Theme {
    fn digest(&self, hasher: &mut Hasher) {
        self.metadata.digest(hasher);
        self.content.digest(hasher);
    }
}

impl From<&Theme> for Checksum {
    fn from(resource: &Theme) -> Checksum {
        let mut hasher = Hasher::new();
        resource.digest(&mut hasher);

        hasher.finalize()
    }
}

impl FromStr for Theme {
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
    pub id: ThemeId,
    pub name: String,
    pub ordinal: u32,
}

impl Digest for Metadata {
    fn digest(&self, hasher: &mut Hasher) {
        self.id.digest(hasher);
        self.name.digest(hasher);
        self.ordinal.digest(hasher);
    }
}

impl Resource<Theme> for Cache {
    fn get(&mut self, id: &str) -> Result<Option<Theme>> {
        let tx = self.conn.transaction()?;
        let mut result = None;

        if let Some(record) = ThemeRecord::select(&tx, id)? {
            let metadata = Metadata {
                id: record.id,
                name: record.name,
                ordinal: record.ordinal,
            };
            let content = record.description;
            let resource = Theme { metadata, content };

            result = Some(resource)
        }

        &self
            .report
            .log(report::Action::Get, report::Entity::Theme, id, "");

        tx.commit()?;

        Ok(result)
    }

    fn add(&mut self, resource: &Theme) -> Result<()> {
        let tx = self.conn.transaction()?;
        let checksum = resource.checksum().to_string();

        if let Some(record) = ThemeRecord::select(&tx, resource.id())? {
            if record.checksum != checksum {
                ThemeRecord::delete(&tx, resource.id())?;
                ThemeRecord::insert(&tx, &resource.into())?;
            }
        } else {
            ThemeRecord::insert(&tx, &resource.into())?;
        }

        Cache::insert_trailmark(&tx, &checksum, "theme", &self.timestamp)?;

        &self.report.log(
            report::Action::Add,
            report::Entity::Theme,
            resource.id(),
            "",
        );

        tx.commit()?;

        Ok(())
    }

    fn drop(&mut self, id: &str) -> Result<Option<Theme>> {
        let resource = self.get(id)?;
        let tx = self.conn.transaction()?;

        if resource.is_some() {
            ThemeRecord::delete(&tx, id)?;
        }

        &self
            .report
            .log(report::Action::Prune, report::Entity::Theme, id, "");

        tx.commit()?;

        Ok(resource)
    }
}

impl From<&Theme> for ThemeRecord {
    fn from(resource: &Theme) -> Self {
        ThemeRecord {
            id: resource.id().clone(),
            checksum: resource.checksum().to_string(),
            name: resource.metadata.name.clone(),
            description: resource.content.clone(),
            ordinal: resource.metadata.ordinal,
        }
    }
}
