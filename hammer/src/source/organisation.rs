//! This module covers the organisation from an input point of view.
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::cache::{Cache, OrganisationRecord};
use crate::checksum::{Checksum, Digest, Hasher};
use crate::report;
use crate::resource::Resource;

type OrganisationId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organisation {
    id: OrganisationId,
    pub name: String,
    pub url: String,
}

impl Organisation {
    pub fn id(&self) -> &OrganisationId {
        &self.id
    }

    pub fn checksum(&self) -> Checksum {
        self.into()
    }
}

impl From<&Organisation> for Checksum {
    fn from(organisation: &Organisation) -> Checksum {
        let mut hasher = Hasher::new();
        organisation.digest(&mut hasher);

        hasher.finalize()
    }
}

impl Digest for Organisation {
    fn digest(&self, hasher: &mut Hasher) {
        self.id.digest(hasher);
        self.name.digest(hasher);
        self.url.digest(hasher);
    }
}

impl FromStr for Organisation {
    type Err = anyhow::Error;

    fn from_str(blob: &str) -> Result<Self, Self::Err> {
        let organisation = serde_json::from_str(blob)?;

        Ok(organisation)
    }
}

impl Resource<Organisation> for Cache {
    fn get(&mut self, id: &str) -> Result<Option<Organisation>> {
        let tx = self.conn.transaction()?;
        let mut result = None;

        if let Some(record) = OrganisationRecord::select(&tx, id)? {
            result = Some(Organisation {
                id: record.id.clone(),
                name: record.name.clone(),
                url: record.url.clone(),
            });
        }

        &self
            .report
            .log(report::Action::Get, report::Entity::Organisation, id, "");

        tx.commit()?;

        Ok(result)
    }

    fn add(&mut self, item: &Organisation) -> Result<()> {
        let tx = self.conn.transaction()?;
        let checksum = item.checksum().to_string();

        if let Some(cached) = OrganisationRecord::select(&tx, &item.id)? {
            if cached.checksum != checksum {
                OrganisationRecord::delete(&tx, &item.id)?;
                OrganisationRecord::insert(&tx, &item.into())?;
            }
        } else {
            OrganisationRecord::insert(&tx, &item.into())?;
        }

        Cache::insert_trailmark(&tx, &checksum, "organisation", &self.timestamp)?;

        &self.report.log(
            report::Action::Add,
            report::Entity::Organisation,
            &item.id,
            "",
        );

        tx.commit()?;

        Ok(())
    }

    fn drop(&mut self, id: &str) -> Result<Option<Organisation>> {
        let item = self.get(&id)?;
        let tx = self.conn.transaction()?;

        if item.is_some() {
            OrganisationRecord::delete(&tx, id)?;
        }

        &self
            .report
            .log(report::Action::Prune, report::Entity::Organisation, id, "");

        tx.commit()?;

        Ok(item)
    }
}

impl From<&Organisation> for OrganisationRecord {
    fn from(item: &Organisation) -> Self {
        OrganisationRecord {
            id: item.id().clone(),
            checksum: item.checksum().to_string(),
            name: item.name.clone(),
            url: item.url.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use serde_json;
    use std::str::FromStr;

    #[test]
    fn add_organisations() -> Result<()> {
        let mut cache = Cache::connect(":memory:")?;
        let raw = r#"[
              {
                "id": "data-standards-authority",
                "name": "Data Standards Authority",
                "url": "https://www.gov.uk/government/groups/data-standards-authority/"
              }
            ]"#;
        let collection: Vec<Organisation> = serde_json::from_str(raw)?;

        for item in &collection {
            cache.add(item)?;
        }

        assert_eq!(collection.len(), 1);

        Ok(())
    }

    #[test]
    fn add_twice() -> Result<()> {
        let mut cache = Cache::connect(":memory:")?;
        let raw = r#"
              {
                "id": "data-standards-authority",
                "name": "Data Standards Authority",
                "url": "https://www.gov.uk/government/groups/data-standards-authority/"
              }
            "#;
        let item = Organisation::from_str(raw)?;

        cache.add(&item)?;
        cache.add(&item)?;

        assert_eq!(item.id, "data-standards-authority");

        Ok(())
    }

    #[test]
    fn gad_item() -> Result<()> {
        let mut cache = Cache::connect(":memory:")?;
        let raw = r#"
          {
            "id": "data-standards-authority",
            "name": "Data Standards Authority",
            "url": "https://www.gov.uk/government/groups/data-standards-authority/"
          }
        "#;
        let item = Organisation::from_str(raw)?;

        cache.add(&item)?;
        let cached: Organisation = cache.get(&item.id)?.expect("organisation doesn't exist");

        assert_eq!(&item.checksum(), &cached.checksum());

        let _: Option<Organisation> = cache.drop(&item.id)?;

        let void: Option<Organisation> = cache.get(&item.id)?;

        assert!(void.is_none());

        Ok(())
    }
}
