//! This module contains the table records as per the cache physical data model.
//!
//! They should strictly match [`../cache.sql`].

mod guidance;
mod licence;
mod organisation;
mod section;
mod standard;
mod topic;

pub use guidance::{GuidanceRecord, GuidanceStandardRecord};
pub use licence::LicenceRecord;
pub use organisation::OrganisationRecord;
pub use section::SectionRecord;
pub use standard::{EndorsementStateRecord, RelatedStandardRecord, StandardRecord};
pub use topic::TopicRecord;
