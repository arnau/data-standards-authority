use serde::{Deserialize, Serialize};

use super::TopicId;
use crate::checksum::{Digest, Hasher};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicReference {
    #[serde(rename = "identifier")]
    pub id: TopicId,
    pub name: String,
    pub theme: String,
}

impl Digest for TopicReference {
    fn digest(&self, hasher: &mut Hasher) {
        self.id.digest(hasher);
        self.name.digest(hasher);
        self.theme.digest(hasher);
    }
}
