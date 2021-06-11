//! This module contains the Status for a piece of writing.
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::str::FromStr;

use crate::checksum::{Digest, Hasher};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    Draft,
    Published,
    Obsolete,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Status::*;
        let s = match self {
            Draft => "draft",
            Published => "published",
            Obsolete => "obsolete",
        };

        write!(f, "{}", s)
    }
}

impl FromStr for Status {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use Status::*;
        match s {
            "draft" => Ok(Draft),
            "published" => Ok(Published),
            "obsolete" => Ok(Obsolete),
            _ => Err(anyhow::anyhow!("{} is not a valid guidance status", s)),
        }
    }
}

impl Digest for Status {
    fn digest(&self, hasher: &mut Hasher) {
        self.to_string().digest(hasher);
    }
}

#[derive(Debug)]
pub struct StatusError;

impl fmt::Display for StatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Couldn't parse a string into a Status")
    }
}

impl Error for StatusError {}
