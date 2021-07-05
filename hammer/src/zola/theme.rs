//! This module covers the theme from a Zola point of view.
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt;

use super::{StandardId, ThemeId};
use crate::cache::records::*;
use crate::cache::Cache;
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
        &self.metadata.extra.id
    }

    pub fn checksum(&self) -> Checksum {
        self.into()
    }

    pub fn path(&self) -> String {
        format!("{}/", self.id())
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let metadata = toml::to_string(&self.metadata).expect("metadata to serialize as TOML");

        writeln!(f, "+++")?;
        write!(f, "{}", &metadata)?;
        writeln!(f, "+++")?;
        write!(f, "{}", &self.content)
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
        self.extra.ordinal.digest(hasher);
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MetadataExtra {
    /// A local unique identifier for the standard.
    #[serde(rename = "identifier")]
    pub id: StandardId,
    pub ordinal: u32,
}

impl Resource<Theme> for Cache {
    fn get(&mut self, id: &str) -> Result<Option<Theme>> {
        let tx = self.conn.transaction()?;
        let mut result = None;

        if let Some(record) = ThemeRecord::select(&tx, id)? {
            result = Some(into_resource(record)?);
        }

        &self
            .report
            .log(report::Action::Get, report::Entity::Theme, id, "");

        tx.commit()?;

        Ok(result)
    }

    fn add(&mut self, _resource: &Theme) -> Result<()> {
        unimplemented!()
    }

    fn drop(&mut self, _id: &str) -> Result<Option<Theme>> {
        unimplemented!()
    }
}

pub fn get_all(cache: &mut Cache) -> Result<Vec<Theme>> {
    let tx = cache.transaction()?;
    let records = ThemeRecord::select_all(&tx)?;
    let mut result = Vec::new();

    for record in records {
        let resource = into_resource(record)?;

        result.push(resource);
    }

    tx.commit()?;

    Ok(result)
}

fn into_resource(record: ThemeRecord) -> Result<Theme> {
    let extra = MetadataExtra {
        id: record.id.clone(),
        ordinal: record.ordinal.clone(),
    };
    let metadata = Metadata {
        title: record.name,
        weight: record.ordinal,
        slug: record.id,
        template: "theme.html".to_string(),
        extra,
    };
    let resource = Theme {
        metadata,
        content: record.description,
    };

    Ok(resource)
}
