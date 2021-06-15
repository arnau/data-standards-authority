use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Report {
    log: Vec<Entry>,
}

impl Report {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn log(&mut self, action: Action, entity: Entity, entity_id: &str, message: &str) {
        self.log.push(Entry {
            timestamp: Utc::now(),
            action,
            entity,
            entity_id: entity_id.into(),
            message: message.into(),
        });
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    Add,
    Prune,
    Get,
    Fail,
    /// Actions to set up and tear down the system.
    Chore,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Entity {
    Cache,
    Standard,
    Licence,
    Organisation,
    Guidance,
    Usecase,
    Casestudy,
    Topic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    timestamp: DateTime<Utc>,
    action: Action,
    entity: Entity,
    entity_id: String,
    message: String,
}
