//! This module covers the standard card and collection from an input point of view.
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::endorsement::EndorsementState;
use super::{split_content, LicenceId, OrganisationId, StandardId, TopicId, Url};
use crate::cache::records::*;
use crate::cache::{Cache, Transaction};
use crate::checksum::{Checksum, Digest, Hasher};
use crate::report;
use crate::resource::Resource;

#[derive(Debug, Clone)]
pub struct Standard {
    pub metadata: Metadata,
    pub content: String,
}

impl Standard {
    pub fn id(&self) -> &StandardId {
        &self.metadata.id
    }

    pub fn checksum(&self) -> Checksum {
        self.into()
    }
}

impl Digest for Standard {
    fn digest(&self, hasher: &mut Hasher) {
        self.metadata.digest(hasher);
        self.content.digest(hasher);
    }
}

impl From<&Standard> for Checksum {
    fn from(standard: &Standard) -> Checksum {
        let mut hasher = Hasher::new();
        standard.digest(&mut hasher);

        hasher.finalize()
    }
}

impl FromStr for Standard {
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
    pub id: StandardId,
    /// The name of the standard.
    pub name: String,
    /// The well-known acronym of the standard.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acronym: Option<String>,
    /// The topic used to classify the standard.
    pub topic: TopicId,
    // /// The list of subjects that refine the topic classification.
    // subjects: Vec<SubjectId>,
    /// The URL to the technical specification for the standard.
    pub specification: Url,
    /// The licence the standard (or specification) is licensed under.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub licence: Option<LicenceId>,
    /// The organisation maintaining the specification.
    pub maintainer: OrganisationId,
    /// The list of related standards.
    #[serde(default)]
    pub related: Vec<StandardId>,
    pub endorsement_state: EndorsementState,
}

impl Digest for Metadata {
    fn digest(&self, hasher: &mut Hasher) {
        self.id.digest(hasher);
        self.name.digest(hasher);
        self.acronym.digest(hasher);
        self.topic.digest(hasher);
        // self.subjects.digest(hasher);
        self.specification.digest(hasher);
        self.licence.digest(hasher);
        self.maintainer.digest(hasher);
        self.related.digest(hasher);
        self.endorsement_state.digest(hasher);
    }
}

impl Resource<Standard> for Cache {
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
                topic: standard_record.topic_id,
                specification: standard_record.specification,
                licence: standard_record.licence_id,
                maintainer: standard_record.maintainer_id,
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

    fn drop(&mut self, standard_id: &str) -> Result<Option<Standard>> {
        let standard = self.get(&standard_id)?;
        let tx = self.conn.transaction()?;

        if standard.is_some() {
            Cache::delete_standard(&tx, standard_id)?;
        }

        &self.report.log(
            report::Action::Prune,
            report::Entity::Standard,
            standard_id,
            "",
        );

        tx.commit()?;

        Ok(standard)
    }
}

impl From<&Standard> for StandardRecord {
    fn from(standard: &Standard) -> Self {
        StandardRecord {
            id: standard.metadata.id.clone(),
            checksum: standard.checksum().to_string(),
            name: standard.metadata.name.clone(),
            acronym: standard.metadata.acronym.clone(),
            topic_id: standard.metadata.topic.clone(),
            specification: standard.metadata.specification.clone(),
            licence_id: standard.metadata.licence.clone(),
            maintainer_id: standard.metadata.maintainer.clone(),
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
    fn baseline_blob() -> Result<()> {
        let standard = Standard::from_str(VAPOUR_STANDARD)?;

        assert_eq!(standard.id(), "vapour");
        assert_eq!(
            &standard.checksum().to_string(),
            "feb2a425f367add826789547e59390d05a9c8aade19a3d619760d57294629faf"
        );
        assert_eq!(
            &standard.content,
            "# Vapour\n\nThis standard will give you no overhead."
        );

        Ok(())
    }

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

        let cached_vapour: Standard = cache.get("vapour")?.unwrap();

        assert_eq!(cached_vapour.metadata.related.len(), 0);
        assert_eq!(cached_vapour.checksum(), vapour_modified.checksum());

        Ok(())
    }

    #[test]
    fn gad_standard() -> Result<()> {
        let original = Standard::from_str(VAPOUR_STANDARD)?;
        let mut cache = Cache::connect(":memory:")?;

        cache.add(&original)?;
        let cached: Standard = cache.get(&original.id())?.expect("standard doesn't exist");

        assert_eq!(&original.checksum(), &cached.checksum());

        let _: Option<Standard> = cache.drop(&original.id())?;
        let void: Option<Standard> = cache.get(&original.id())?;

        assert!(void.is_none());

        Ok(())
    }
}
