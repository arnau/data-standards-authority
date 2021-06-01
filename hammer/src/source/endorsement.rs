use anyhow;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use super::Date;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EndorsementStatus {
    Identified,
    Proposed,
    Endorsed,
    Retired,
    Disavowed,
    Superseded,
}

impl fmt::Display for EndorsementStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use EndorsementStatus::*;
        let s = match self {
            Identified => "identified",
            Proposed => "proposed",
            Endorsed => "endorsed",
            Retired => "retired",
            Disavowed => "disavowed",
            Superseded => "superseded",
        };

        write!(f, "{}", s)
    }
}

impl FromStr for EndorsementStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use EndorsementStatus::*;
        match s {
            "identified" => Ok(Identified),
            "proposed" => Ok(Proposed),
            "endorsed" => Ok(Endorsed),
            "retired" => Ok(Retired),
            "disavowed" => Ok(Disavowed),
            "superseded" => Ok(Superseded),
            _ => Err(anyhow::anyhow!("{} is not a valid endorsement status", s)),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EndorsementState {
    pub status: EndorsementStatus,
    pub start_date: Date,
    pub review_date: Date,
    #[serde(default)]
    pub end_date: Option<Date>,
}
