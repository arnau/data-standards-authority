//! This module covers the guidance piece and collection from an input point of view.
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::{split_content, Date, OrganisationId, StandardId, Url};
use crate::cache::{Cache, GuidanceRecord, GuidanceStandardRecord, Transaction};
use crate::checksum::{Checksum, Digest, Hasher};
use crate::report;
use crate::resource::Resource;
use crate::Status;

type GuidanceId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guidance {
    pub metadata: Metadata,
    pub content: String,
}

impl Guidance {
    pub fn id(&self) -> &GuidanceId {
        &self.metadata.id
    }

    pub fn checksum(&self) -> Checksum {
        self.into()
    }
}

impl Digest for Guidance {
    fn digest(&self, hasher: &mut Hasher) {
        self.metadata.digest(hasher);
        self.content.digest(hasher);
    }
}

impl From<&Guidance> for Checksum {
    fn from(guidance: &Guidance) -> Checksum {
        let mut hasher = Hasher::new();
        guidance.digest(&mut hasher);

        hasher.finalize()
    }
}

impl FromStr for Guidance {
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

impl Resource<Guidance> for Cache {
    fn get(&mut self, id: &str) -> Result<Option<Guidance>> {
        let tx = self.conn.transaction()?;
        let mut result = None;

        if let Some(cached) = GuidanceRecord::select(&tx, id)? {
            let related_records = GuidanceStandardRecord::select(&tx, id)?;

            let related = related_records
                .iter()
                .map(|record| record.standard_id.clone())
                .collect::<Vec<_>>();

            let metadata = Metadata {
                id: cached.id,
                description: cached.description,
                maintainer: cached.maintainer_id,
                status: cached.status,
                creation_date: cached.creation_date,
                update_date: cached.update_date,
                publication_date: cached.publication_date,
                standards: related,
                canonical_url: cached.canonical_url,
            };

            let guidance = Guidance {
                metadata,
                content: cached.content,
            };

            result = Some(guidance);
        }

        &self
            .report
            .log(report::Action::Get, report::Entity::Guidance, id, "");

        tx.commit()?;

        Ok(result)
    }

    fn add(&mut self, item: &Guidance) -> Result<()> {
        let tx = self.conn.transaction()?;
        let checksum = item.checksum().to_string();

        if let Some(cached) = GuidanceRecord::select(&tx, item.id())? {
            if cached.checksum != checksum {
                GuidanceRecord::delete(&tx, &item.id())?;
                create(&tx, &item)?;
            }
        } else {
            create(&tx, &item)?;
        }

        Cache::insert_trailmark(&tx, &checksum, "guidance", &self.timestamp)?;

        &self
            .report
            .log(report::Action::Add, report::Entity::Standard, item.id(), "");

        tx.commit()?;

        Ok(())
    }

    fn drop(&mut self, id: &str) -> Result<Option<Guidance>> {
        let item = self.get(&id)?;
        let tx = self.conn.transaction()?;

        if item.is_some() {
            GuidanceRecord::delete(&tx, id)?;
        }

        &self
            .report
            .log(report::Action::Prune, report::Entity::Guidance, id, "");

        tx.commit()?;

        Ok(item)
    }
}

fn create(tx: &Transaction, item: &Guidance) -> Result<()> {
    GuidanceRecord::insert(&tx, &item.into())?;

    for standard_id in &item.metadata.standards {
        GuidanceStandardRecord::insert(
            &tx,
            &GuidanceStandardRecord {
                guidance_id: item.id().clone(),
                standard_id: standard_id.clone(),
            },
        )?;
    }

    Ok(())
}

impl From<&Guidance> for GuidanceRecord {
    fn from(guidance: &Guidance) -> Self {
        GuidanceRecord {
            id: guidance.id().clone(),
            checksum: guidance.checksum().to_string(),
            description: guidance.metadata.description.clone(),
            maintainer_id: guidance.metadata.maintainer.clone(),
            status: guidance.metadata.status.clone(),
            creation_date: guidance.metadata.creation_date.to_string(),
            update_date: guidance.metadata.update_date.to_string(),
            publication_date: guidance.metadata.publication_date.clone(),
            canonical_url: guidance.metadata.canonical_url.clone(),
            content: guidance.content.clone(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metadata {
    #[serde(rename = "identifier")]
    id: GuidanceId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    maintainer: OrganisationId,
    status: Status,
    creation_date: Date,
    update_date: Date,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    publication_date: Option<Date>,
    standards: Vec<StandardId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    canonical_url: Option<Url>,
}

impl Digest for Metadata {
    fn digest(&self, hasher: &mut Hasher) {
        self.id.digest(hasher);
        self.description.digest(hasher);
        self.maintainer.digest(hasher);
        self.status.digest(hasher);
        self.creation_date.digest(hasher);
        self.update_date.digest(hasher);
        self.publication_date.digest(hasher);
        self.standards.digest(hasher);
        self.canonical_url.digest(hasher);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::str::FromStr;

    static GUIDANCE: &'static str = r#"---
type: guidance
identifier: when-to-use-a-graphql-api
maintainer: data-standards-authority
status: draft
creation_date: 2021-04-01
update_date: 2021-05-14
standards:
  - graphql
---
# When to use a GraphQL API

[GraphQL] is an API specification originally developed by Facebook as an alternative to REST for compiling complex data structures in real time. It was open-sourced in 2015 and is used by many organisations with similarly complex requirements.
"#;

    #[test]
    fn single() -> Result<()> {
        let guidance = Guidance::from_str(GUIDANCE)?;
        let mut cache = Cache::connect(":memory:")?;

        cache.add(&guidance)?;

        assert_eq!(&guidance.id().to_string(), "when-to-use-a-graphql-api");

        Ok(())
    }

    #[test]
    fn gad() -> Result<()> {
        let original = Guidance::from_str(GUIDANCE)?;
        let mut cache = Cache::connect(":memory:")?;

        cache.add(&original)?;
        let cached: Guidance = cache.get(&original.id())?.expect("guidance doesn't exist");

        assert_eq!(&original.checksum(), &cached.checksum());

        let _: Option<Guidance> = cache.drop(&original.id())?;
        let void: Option<Guidance> = cache.get(&original.id())?;

        assert!(void.is_none());

        Ok(())
    }
}
