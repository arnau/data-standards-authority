use serde::Deserialize;

use super::Date;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EndorsementStatus {
    Identified,
    Proposed,
    Endorsed,
    Retired,
    Disavowed,
    Superseded,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EndorsementState {
    status: EndorsementStatus,
    start_date: Date,
    review_date: Date,
    #[serde(default)]
    end_date: Option<Date>,
}
