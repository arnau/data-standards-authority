//! This module covers the standard card and collection from an input point of view.
use serde::Deserialize;

use super::endorsement::EndorsementState;
use super::{LicenceId, OrganisationId, TopicId, Url};

pub type StandardId = String;

#[derive(Debug, Clone, Deserialize)]
struct Standard {
    /// A local unique identifier for the standard.
    #[serde(rename(serialize = "identifier"))]
    id: StandardId,
    /// The name of the standard.
    name: String,
    /// The well-known acronym of the standard.
    #[serde(default)]
    acronym: Option<String>,
    /// The topic used to classify the standard.
    topic: TopicId,
    /// The list of subjects that refine the topic classification.
    // subjects: Vec<SubjectId>,
    /// The URL to the technical specification for the standard.
    specification: Url,
    /// The licence the standard (or specification) is licensed under.
    #[serde(default)]
    licence: Option<LicenceId>,
    /// The organisation maintaining the specification.
    maintainer: OrganisationId,
    /// The list of related standards.
    #[serde(default)]
    related: Vec<StandardId>,
    endorsement_state: EndorsementState,
}
