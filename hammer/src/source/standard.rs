//! This module covers the standard card and collection from an input point of view.
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::endorsement::EndorsementState;
use super::{split_content, LicenceId, OrganisationId, TopicId, Url};
use crate::checksum::{Checksum, Digest, Hasher};

pub type StandardId = String;

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
    #[serde(default)]
    pub acronym: Option<String>,
    /// The topic used to classify the standard.
    pub topic: TopicId,
    // /// The list of subjects that refine the topic classification.
    // subjects: Vec<SubjectId>,
    /// The URL to the technical specification for the standard.
    pub specification: Url,
    /// The licence the standard (or specification) is licensed under.
    #[serde(default)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::str::FromStr;

    #[test]
    fn baseline_blob() -> Result<()> {
        let blob = r#"---
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
        let standard = Standard::from_str(blob)?;

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
}
