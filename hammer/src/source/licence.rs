//! This module covers the standard card and collection from an input point of view.
use serde::{Deserialize, Serialize};

use crate::checksum::{Checksum, Digest, Hasher};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Licence {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub acronym: Option<String>,
    pub url: String,
}

impl Licence {
    pub fn checksum(&self) -> Checksum {
        self.into()
    }
}

impl From<&Licence> for Checksum {
    fn from(licence: &Licence) -> Checksum {
        let mut hasher = Hasher::new();

        licence.id.digest(&mut hasher);
        licence.name.digest(&mut hasher);
        licence.acronym.digest(&mut hasher);
        licence.url.digest(&mut hasher);

        hasher.finalize()
    }
}
