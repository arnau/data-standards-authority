//! This module contains the table records as per the cache physical data model.
//!
//! They should strictly match [`../cache.sql`].

#[derive(Debug, Clone)]
pub struct StandardRecord {
    pub(crate) id: String,
    pub(crate) checksum: String,
    pub(crate) name: String,
    pub(crate) acronym: Option<String>,
    pub(crate) topic_id: String,
    pub(crate) specification: String,
    pub(crate) licence_id: Option<String>,
    pub(crate) maintainer_id: String,
    pub(crate) content: String,
}

#[derive(Debug, Clone)]
pub struct RelatedStandardRecord {
    pub(crate) standard_id: String,
    pub(crate) related_standard_id: String,
}

#[derive(Debug, Clone)]
pub struct EndorsementStateRecord {
    pub(crate) standard_id: String,
    pub(crate) status: String,
    pub(crate) start_date: String,
    pub(crate) review_date: String,
    pub(crate) end_date: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LicenceRecord {
    pub id: String,
    pub checksum: String,
    pub name: String,
    pub acronym: Option<String>,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct OrganisationRecord {
    pub id: String,
    pub checksum: String,
    pub name: String,
    pub url: String,
}
