//! This module deals with data shaped as source, a mix of Markdown, Toml, CSV and YAML.
//!
//! Source Markdown files are prepended with a YAML frontmatter.
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

pub mod endorsement;
pub mod licence;
pub mod standard;

pub use licence::Licence;
pub use standard::Standard;

// TODO: Consider promoting to Chrono
pub type Date = String;

// pub type SubjectId = String;
pub type LicenceId = String;
pub type OrganisationId = String;
pub type TopicId = String;
pub type Url = String;

fn split_content(blob: &str) -> Result<(&str, &str)> {
    lazy_static! {
        static ref FRONTMATTER_RE: Regex =
            Regex::new(r"^\s*---(\r?\n(?s).*?(?-s))---\r?\n?((?s).*(?-s))$").unwrap();
    }

    let groups = FRONTMATTER_RE
        .captures(blob)
        .expect("frontmatter split failure");
    let frontmatter = groups.get(1).expect("group frontmatter missing").as_str();
    let content = groups.get(2).expect("group content missing").as_str();

    Ok((frontmatter, content))
}

/// The source lens.
pub trait Source {}
