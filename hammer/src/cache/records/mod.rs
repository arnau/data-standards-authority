//! This module contains the table records as per the cache physical data model.
//!
//! They should strictly match [`../cache.sql`].

mod guidance;
mod licence;
mod organisation;
mod standard;

pub use guidance::{GuidanceRecord, GuidanceStandardRecord};
pub use licence::LicenceRecord;
pub use organisation::OrganisationRecord;
pub use standard::{EndorsementStateRecord, RelatedStandardRecord, StandardRecord};
