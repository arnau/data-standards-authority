//! This module covers the licence from an input point of view.
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::LicenceId;
use crate::cache::{Cache, LicenceRecord};
use crate::checksum::{Checksum, Digest, Hasher};
use crate::report;
use crate::resource::Resource;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Licence {
    id: LicenceId,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acronym: Option<String>,
    pub url: String,
}

impl Licence {
    pub fn id(&self) -> &LicenceId {
        &self.id
    }

    pub fn checksum(&self) -> Checksum {
        self.into()
    }
}

impl From<&Licence> for Checksum {
    fn from(licence: &Licence) -> Checksum {
        let mut hasher = Hasher::new();
        licence.digest(&mut hasher);

        hasher.finalize()
    }
}

impl Digest for Licence {
    fn digest(&self, hasher: &mut Hasher) {
        self.id.digest(hasher);
        self.name.digest(hasher);
        self.acronym.digest(hasher);
        self.url.digest(hasher);
    }
}

impl FromStr for Licence {
    type Err = anyhow::Error;

    fn from_str(blob: &str) -> Result<Self, Self::Err> {
        let licence = serde_json::from_str(blob)?;

        Ok(licence)
    }
}

impl Resource<Licence> for Cache {
    fn get(&mut self, licence_id: &str) -> Result<Option<Licence>> {
        let tx = self.conn.transaction()?;
        let mut result = None;

        if let Some(licence_record) = LicenceRecord::select(&tx, licence_id)? {
            result = Some(Licence {
                id: licence_record.id.clone(),
                name: licence_record.name.clone(),
                acronym: licence_record.acronym.clone(),
                url: licence_record.url.clone(),
            });
        }

        &self
            .report
            .log(report::Action::Get, report::Entity::Licence, licence_id, "");

        tx.commit()?;

        Ok(result)
    }

    fn add(&mut self, licence: &Licence) -> Result<()> {
        let tx = self.conn.transaction()?;
        let checksum = licence.checksum().to_string();

        if let Some(cached) = LicenceRecord::select(&tx, &licence.id)? {
            if cached.checksum != checksum {
                LicenceRecord::delete(&tx, &licence.id)?;
                LicenceRecord::insert(&tx, &licence.into())?;
            }
        } else {
            LicenceRecord::insert(&tx, &licence.into())?;
        }

        Cache::insert_trailmark(&tx, &checksum, "licence", &self.timestamp)?;

        &self.report.log(
            report::Action::Add,
            report::Entity::Licence,
            &licence.id,
            "",
        );

        tx.commit()?;

        Ok(())
    }

    fn drop(&mut self, licence_id: &str) -> Result<Option<Licence>> {
        let licence = self.get(&licence_id)?;
        let tx = self.conn.transaction()?;

        if licence.is_some() {
            LicenceRecord::delete(&tx, licence_id)?;
        }

        &self.report.log(
            report::Action::Prune,
            report::Entity::Licence,
            licence_id,
            "",
        );

        tx.commit()?;

        Ok(licence)
    }
}

impl From<&Licence> for LicenceRecord {
    fn from(licence: &Licence) -> Self {
        LicenceRecord {
            id: licence.id.clone(),
            checksum: licence.checksum().to_string(),
            name: licence.name.clone(),
            acronym: licence.acronym.clone(),
            url: licence.url.clone(),
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
    fn add_licences() -> Result<()> {
        let mut cache = Cache::connect(":memory:")?;
        let raw = r#"[
          {
            "id": "ogl-3",
            "name": "Open Government License",
            "acronym": "OGL",
            "url": "https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/"
          },
          {
            "id": "mit",
            "name": "MIT License",
            "acronym": "MIT",
            "url": "https://choosealicense.com/licenses/mit/"
          },
          {
            "id": "owfa-1-0",
            "name": "Open Web Foundation Agreement 1.0",
            "url": "http://www.openwebfoundation.org/legal/the-owf-1-0-agreements/owfa-1-0"
          }
        ]"#;
        let licences: Vec<Licence> = serde_json::from_str(raw)?;

        for licence in &licences {
            cache.add(licence)?;
        }

        assert_eq!(licences.len(), 3);

        Ok(())
    }

    #[test]
    fn add_licence_twice() -> Result<()> {
        let mut cache = Cache::connect(":memory:")?;
        let raw = r#"
          {
            "id": "ogl-3",
            "name": "Open Government License",
            "acronym": "OGL",
            "url": "https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/"
          }
        "#;
        let licence = Licence::from_str(raw)?;

        cache.add(&licence)?;
        cache.add(&licence)?;

        assert_eq!(licence.id, "ogl-3");

        Ok(())
    }

    #[test]
    fn gad_licence() -> Result<()> {
        let mut cache = Cache::connect(":memory:")?;
        let raw = r#"
          {
            "id": "ogl-3",
            "name": "Open Government License",
            "acronym": "OGL",
            "url": "https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/"
          }
        "#;
        let licence = Licence::from_str(raw)?;

        cache.add(&licence)?;
        let cached: Licence = cache.get(&licence.id)?.expect("licence doesn't exist");

        assert_eq!(&licence.checksum(), &cached.checksum());

        let _: Option<Licence> = cache.drop(&licence.id)?;

        let void: Option<Licence> = cache.get(&licence.id)?;

        assert!(void.is_none());

        Ok(())
    }
}
