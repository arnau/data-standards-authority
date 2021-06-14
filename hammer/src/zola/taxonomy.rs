use serde::{Deserialize, Serialize};

use super::TopicId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub id: TopicId,
    pub name: String,
    pub description: String,
}
