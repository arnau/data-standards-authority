//! This module defines a generic resource to be implemented to interface with the cache storage.

use anyhow::{bail, Result};
use std::fmt;
use std::str::FromStr;

use crate::checksum::Digest;

/// A trait to be implemented by cachable resources.
pub trait Resource<Item: Digest> {
    /// Composes a single resource given its id.
    fn get(&mut self, id: &str) -> Result<Option<Item>>;

    /// Inserts the given resource to the store.
    fn add(&mut self, item: &Item) -> Result<()>;

    /// Cleans a single resource and potentially any dependency given its id.
    fn drop(&mut self, id: &str) -> Result<Option<Item>>;

    // /// Inserts the given collection of resources to the store.
    // fn bulk(&mut self, collection: &[Item]) -> Result<()>;

    // /// Composes the full collection of resources.
    // fn mass(&mut self) -> Result<Vec<Item>>;
}

/// Markdown based resource types.
///
/// Auxiliary types such as Licence or Organisation are not considered here as they are never represented on their own.
#[derive(Debug, Clone)]
pub enum ResourceType {
    CaseStudy,
    Guidance,
    Section,
    Standard,
    Theme,
    Topic,
    Unknown,
    UseCase,
}

impl ResourceType {
    /// Peeks the first few characters expecting to find a type declaration as the first property of the frontmatter
    /// Yaml.
    pub fn from_hint(text: &str) -> ResourceType {
        if text.starts_with("---\ntype: standard\n") {
            return ResourceType::Standard;
        }

        if text.starts_with("---\ntype: topic\n") {
            return ResourceType::Topic;
        }

        if text.starts_with("---\ntype: guidance\n") {
            return ResourceType::Guidance;
        }

        if text.starts_with("---\ntype: section\n") {
            return ResourceType::Section;
        }

        ResourceType::Unknown
    }
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ResourceType::*;

        let s = match self {
            CaseStudy => "case-study",
            Guidance => "guidance",
            Section => "section",
            Standard => "standard",
            Theme => "theme",
            Topic => "topic",
            Unknown => "unknown",
            UseCase => "use-case",
        };

        write!(f, "{}", s)
    }
}

impl FromStr for ResourceType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ResourceType::*;

        match s {
            "case-study" => Ok(CaseStudy),
            "guidance" => Ok(Guidance),
            "section" => Ok(Section),
            "standard" => Ok(Standard),
            "theme" => Ok(Theme),
            "topic" => Ok(Topic),
            "use-case" => Ok(UseCase),
            _ => bail!(format!("'{}' is not a known resource type", s)),
        }
    }
}
