//! This module deals with data shaped as source, a mix of Markdown, Toml, CSV and YAML.
//!
//! Source Markdown files are prepended with a YAML frontmatter.
use crate::cache::records::*;
use crate::cache::Transaction;
use crate::report;
use crate::Cache;
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

pub mod endorsement;
pub mod licence;
pub mod standard;

use endorsement::EndorsementState;
pub use licence::Licence;
use standard::Metadata;
pub use standard::Standard;

// TODO: Consider promoting to Chrono
pub type Date = String;

// pub type SubjectId = String;
pub type LicenceId = String;
pub type OrganisationId = String;
pub type TopicId = String;
pub type Url = String;

fn split_content(blob: &str) -> Result<(&str, &str)> {
    lazy_static! {
        static ref FRONTMATTER_RE: Regex =
            Regex::new(r"^\s*---(\r?\n(?s).*?(?-s))---\r?\n?((?s).*(?-s))$").unwrap();
    }

    let groups = FRONTMATTER_RE
        .captures(blob)
        .expect("frontmatter split failure");
    let frontmatter = groups.get(1).expect("group frontmatter missing").as_str();
    let content = groups.get(2).expect("group content missing").as_str();

    Ok((frontmatter, content))
}

/// Cache operations from a source perspective.
pub trait Source {
    /// Given a standard id, retrives and shapes a Standard according to the Source spec.
    fn get_standard(&mut self, standard_id: &str) -> Result<Option<Standard>>;

    /// Given a Source Standard, attempts to store it in the cache.
    ///
    /// If the standard already exists in the cache it will either update or skip depending on whether it has changed.
    fn add_standard(&mut self, standard: &Standard) -> Result<()>;

    /// Given a standard id it attempts to remove it from the cache (this includes all dependent information such as
    /// related and endorsement state).
    fn prune_standard(&mut self, standard_id: &str) -> Result<()>;

    fn get_licence(&mut self, licence_id: &str) -> Result<Option<Licence>>;

    fn add_licence(&mut self, licence: &Licence) -> Result<()>;

    fn prune_licence(&mut self, licence_id: &str) -> Result<()>;
}

impl Source for Cache {
    fn get_standard(&mut self, standard_id: &str) -> Result<Option<Standard>> {
        let tx = self.conn.transaction()?;
        let mut result = None;

        if let Some(standard_record) = Cache::select_standard(&tx, standard_id)? {
            let related_records = Cache::select_related_standards(&tx, standard_id)?;
            let endorsement_record = Cache::select_endorsement_state(&tx, standard_id)?
                .expect("missing endorsement state. the cache is corrupted.");

            let related = related_records
                .iter()
                .map(|record| record.related_standard_id.clone())
                .collect::<Vec<_>>();
            let endorsement_state = EndorsementState {
                status: endorsement_record.status.parse()?,
                start_date: endorsement_record.start_date,
                review_date: endorsement_record.review_date,
                end_date: endorsement_record.end_date,
            };
            let metadata = Metadata {
                id: standard_record.id,
                name: standard_record.name,
                acronym: standard_record.acronym,
                topic: standard_record.topic,
                specification: standard_record.specification,
                licence: standard_record.licence,
                maintainer: standard_record.maintainer,
                related,
                endorsement_state,
            };
            let standard = Standard {
                metadata,
                content: standard_record.content,
            };

            result = Some(standard);
        }

        &self.report.log(
            report::Action::Get,
            report::Entity::Standard,
            standard_id,
            "",
        );

        tx.commit()?;

        Ok(result)
    }

    fn add_standard(&mut self, standard: &Standard) -> Result<()> {
        let tx = self.conn.transaction()?;
        let checksum = standard.checksum().to_string();

        if let Some(cached_standard) = Cache::select_standard(&tx, standard.id())? {
            if cached_standard.checksum != checksum {
                update_standard(&tx, standard)?;
            }
        } else {
            create_standard(&tx, standard)?;
        }

        Cache::insert_trailmark(&tx, &checksum, "standard", &self.timestamp)?;

        &self.report.log(
            report::Action::Add,
            report::Entity::Standard,
            standard.id(),
            "",
        );

        tx.commit()?;

        Ok(())
    }

    fn prune_standard(&mut self, standard_id: &str) -> Result<()> {
        let tx = self.conn.transaction()?;

        Cache::delete_standard(&tx, standard_id)?;

        &self.report.log(
            report::Action::Prune,
            report::Entity::Standard,
            standard_id,
            "",
        );

        tx.commit()?;

        Ok(())
    }

    fn get_licence(&mut self, licence_id: &str) -> Result<Option<Licence>> {
        let tx = self.conn.transaction()?;
        let mut result = None;

        if let Some(licence_record) = Cache::select_licence(&tx, licence_id)? {
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

    fn add_licence(&mut self, licence: &Licence) -> Result<()> {
        let tx = self.conn.transaction()?;
        let checksum = licence.checksum().to_string();

        if let Some(cached) = Cache::select_licence(&tx, &licence.id)? {
            if cached.checksum != checksum {
                Cache::delete_licence(&tx, &licence.id)?;
            }
        }

        Cache::insert_licence(&tx, &licence.into())?;

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

    fn prune_licence(&mut self, licence_id: &str) -> Result<()> {
        let tx = self.conn.transaction()?;

        Cache::delete_licence(&tx, licence_id)?;

        &self.report.log(
            report::Action::Prune,
            report::Entity::Licence,
            licence_id,
            "",
        );

        tx.commit()?;

        Ok(())
    }
}

use crate::resource::Resource;

impl Resource<dyn Source, Standard> for Cache {
    fn get(&mut self, standard_id: &str) -> Result<Option<Standard>> {
        let tx = self.conn.transaction()?;
        let mut result = None;

        if let Some(standard_record) = Cache::select_standard(&tx, standard_id)? {
            let related_records = Cache::select_related_standards(&tx, standard_id)?;
            let endorsement_record = Cache::select_endorsement_state(&tx, standard_id)?
                .expect("missing endorsement state. the cache is corrupted.");

            let related = related_records
                .iter()
                .map(|record| record.related_standard_id.clone())
                .collect::<Vec<_>>();
            let endorsement_state = EndorsementState {
                status: endorsement_record.status.parse()?,
                start_date: endorsement_record.start_date,
                review_date: endorsement_record.review_date,
                end_date: endorsement_record.end_date,
            };
            let metadata = Metadata {
                id: standard_record.id,
                name: standard_record.name,
                acronym: standard_record.acronym,
                topic: standard_record.topic,
                specification: standard_record.specification,
                licence: standard_record.licence,
                maintainer: standard_record.maintainer,
                related,
                endorsement_state,
            };
            let standard = Standard {
                metadata,
                content: standard_record.content,
            };

            result = Some(standard);
        }

        &self.report.log(
            report::Action::Get,
            report::Entity::Standard,
            standard_id,
            "",
        );

        tx.commit()?;

        Ok(result)
    }

    fn add(&mut self, standard: &Standard) -> Result<()> {
        let tx = self.conn.transaction()?;
        let checksum = standard.checksum().to_string();

        if let Some(cached_standard) = Cache::select_standard(&tx, standard.id())? {
            if cached_standard.checksum != checksum {
                update_standard(&tx, standard)?;
            }
        } else {
            create_standard(&tx, standard)?;
        }

        Cache::insert_trailmark(&tx, &checksum, "standard", &self.timestamp)?;

        &self.report.log(
            report::Action::Add,
            report::Entity::Standard,
            standard.id(),
            "",
        );

        tx.commit()?;

        Ok(())
    }

    fn prune(&mut self, standard_id: &str) -> Result<()> {
        let tx = self.conn.transaction()?;

        Cache::delete_standard(&tx, standard_id)?;

        &self.report.log(
            report::Action::Prune,
            report::Entity::Standard,
            standard_id,
            "",
        );

        tx.commit()?;

        Ok(())
    }
}

impl Resource<Source, Licence> for Cache {
    fn get(&mut self, licence_id: &str) -> Result<Option<Licence>> {
        let tx = self.conn.transaction()?;
        let mut result = None;

        if let Some(licence_record) = Cache::select_licence(&tx, licence_id)? {
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

        if let Some(cached) = Cache::select_licence(&tx, &licence.id)? {
            if cached.checksum != checksum {
                Cache::delete_licence(&tx, &licence.id)?;
            }
        }

        Cache::insert_licence(&tx, &licence.into())?;

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

    fn prune(&mut self, licence_id: &str) -> Result<()> {
        let tx = self.conn.transaction()?;

        Cache::delete_licence(&tx, licence_id)?;

        &self.report.log(
            report::Action::Prune,
            report::Entity::Licence,
            licence_id,
            "",
        );

        tx.commit()?;

        Ok(())
    }
}

impl From<&Standard> for StandardRecord {
    fn from(standard: &Standard) -> Self {
        StandardRecord {
            id: standard.metadata.id.clone(),
            checksum: standard.checksum().to_string(),
            name: standard.metadata.name.clone(),
            acronym: standard.metadata.acronym.clone(),
            topic: standard.metadata.topic.clone(),
            specification: standard.metadata.specification.clone(),
            licence: standard.metadata.licence.clone(),
            maintainer: standard.metadata.maintainer.clone(),
            content: standard.content.clone(),
        }
    }
}

impl From<&Standard> for EndorsementStateRecord {
    fn from(standard: &Standard) -> Self {
        EndorsementStateRecord {
            standard_id: standard.metadata.id.clone(),
            status: standard.metadata.endorsement_state.status.to_string(),
            start_date: standard.metadata.endorsement_state.start_date.to_string(),
            review_date: standard.metadata.endorsement_state.review_date.to_string(),
            end_date: standard.metadata.endorsement_state.end_date.clone(),
        }
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

/// Helper to perform a strict create. Will fail if the standard exists.
fn create_standard(tx: &Transaction, standard: &Standard) -> Result<()> {
    Cache::insert_standard(&tx, &standard.into())?;

    for related in &standard.metadata.related {
        Cache::insert_related_standard(
            &tx,
            &RelatedStandardRecord {
                standard_id: standard.id().clone(),
                related_standard_id: related.clone(),
            },
        )?;
    }

    Cache::insert_endorsement_state(&tx, &standard.into())?;

    Ok(())
}

/// Helper to perform replace an existing standard. This relies on the `ON DELETE CASCADE`.
fn update_standard(tx: &Transaction, standard: &Standard) -> Result<()> {
    Cache::delete_standard(&tx, &standard.id())?;
    create_standard(&tx, standard)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use serde_json;
    use std::str::FromStr;

    static VAPOUR_STANDARD: &'static str = r#"---
type: standard
identifier: vapour
name: Vapour
topic: exchange
subjects:
    - api_access
specification: https://spec.vapour.org/
licence: ogl
maintainer: data-standards-authority
endorsement_state:
    status: identified
    start_date: 2021-06-01
    review_date: 2021-06-01
related:
    - steam
---
# Vapour

This standard will give you no overhead."#;
    static STEAM_STANDARD: &'static str = r#"---
type: standard
identifier: steam
name: Steam
topic: exchange
subjects:
    - api_access
specification: https://spec.steam.org/
licence: ogl
maintainer: data-standards-authority
endorsement_state:
    status: identified
    start_date: 2021-06-01
    review_date: 2021-06-01
related:
    - vapour
---
# Steam

This standard will give you warmth."#;
    #[test]
    fn single_standard() -> Result<()> {
        let standard = Standard::from_str(VAPOUR_STANDARD)?;
        let mut cache = Cache::connect(":memory:")?;

        cache.add(&standard)?;

        assert_eq!(
            &standard.checksum().to_string(),
            "feb2a425f367add826789547e59390d05a9c8aade19a3d619760d57294629faf"
        );

        Ok(())
    }

    #[test]
    fn two_standard() -> Result<()> {
        let vapour = Standard::from_str(VAPOUR_STANDARD)?;
        let steam = Standard::from_str(STEAM_STANDARD)?;
        let mut cache = Cache::connect(":memory:")?;

        cache.add(&vapour)?;
        cache.add(&steam)?;

        assert_eq!(
            &vapour.checksum().to_string(),
            "feb2a425f367add826789547e59390d05a9c8aade19a3d619760d57294629faf"
        );

        Ok(())
    }

    #[test]
    fn same_standard_twice() -> Result<()> {
        let vapour = Standard::from_str(VAPOUR_STANDARD)?;
        let mut cache = Cache::connect(":memory:")?;

        cache.add(&vapour)?;
        cache.add(&vapour)?;

        assert_eq!(
            &vapour.checksum().to_string(),
            "feb2a425f367add826789547e59390d05a9c8aade19a3d619760d57294629faf"
        );

        Ok(())
    }

    #[test]
    fn update_standard() -> Result<()> {
        let mut cache = Cache::connect(":memory:")?;
        let vapour = Standard::from_str(VAPOUR_STANDARD)?;
        let vapour2 = r#"---
type: standard
identifier: vapour
name: Vapour
topic: exchange
subjects:
    - api_access
specification: https://spec.vapour.org/
licence: ogl
maintainer: data-standards-authority
endorsement_state:
    status: identified
    start_date: 2021-06-01
    review_date: 2021-06-01
---
# Vapour

This standard will give you no overhead."#;

        let vapour_modified = Standard::from_str(vapour2)?;

        cache.add(&vapour)?;
        cache.add(&vapour_modified)?;

        let cached_vapour = cache.get_standard("vapour")?.unwrap();

        assert_eq!(cached_vapour.metadata.related.len(), 0);
        assert_eq!(cached_vapour.checksum(), vapour_modified.checksum());

        Ok(())
    }

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

        assert_eq!(licences.len(), 3);

        Ok(())
    }
}
