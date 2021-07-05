//! This module deals with data shaped as source, a mix of Markdown, Toml, CSV and YAML.
//!
//! Source Markdown files are prepended with a YAML frontmatter.
use anyhow::Result;
use lazy_static::lazy_static;
use log::{info, warn};
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;
use walkdir::{DirEntry, WalkDir};

pub mod endorsement;
pub mod guidance;
pub mod licence;
pub mod organisation;
pub mod section;
pub mod standard;
pub mod theme;
pub mod topic;

pub use guidance::Guidance;
pub use licence::Licence;
pub use organisation::Organisation;
pub use section::Section;
pub use standard::Standard;
pub use theme::Theme;
pub use topic::Topic;

use crate::cache::Cache;
use crate::resource::{Resource, ResourceType};

// TODO: Consider promoting to Chrono
pub type Date = String;

// pub type SubjectId = String;
pub type StandardId = String;
pub type LicenceId = String;
pub type OrganisationId = String;
pub type TopicId = String;
pub type ThemeId = String;
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

/// Helper function for the CLI to read from the given path and cache the content.
pub fn read(source_dir: &Path, cache: &mut Cache) -> Result<()> {
    let walker = WalkDir::new(source_dir).into_iter();

    for result in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = result?;

        if let Some(ext) = entry.path().extension() {
            if ext == "md" {
                process_markdown_source(cache, &entry.path())?;
            } else if ext == "json" {
                process_json_source(cache, &entry.path())?;
            } else {
                warn!("unprocessed {}", &entry.path().display());
            }
        }
    }

    Ok(())
}

fn process_markdown_source(cache: &mut Cache, entry: &Path) -> Result<()> {
    let path = entry.display().to_string();
    let mut file = File::open(&entry)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let resource_type = ResourceType::from_hint(&contents);

    match resource_type {
        ResourceType::Unknown => {
            warn!("unknown type {}", &path);
        }
        ref typ => {
            info!("{} type {}", typ, &path);
        }
    }

    match resource_type {
        ResourceType::Guidance => {
            let resource = Guidance::from_str(&contents)?;
            cache.add((&resource).into())?;
        }
        ResourceType::Section => {
            let resource = Section::from_str(&contents)?;
            cache.add((&resource).into())?;
        }
        ResourceType::Standard => {
            let resource = Standard::from_str(&contents)?;
            cache.add((&resource).into())?;
        }
        ResourceType::Theme => {
            let resource = Theme::from_str(&contents)?;
            cache.add((&resource).into())?;
        }
        ResourceType::Topic => {
            let resource = Topic::from_str(&contents)?;
            cache.add((&resource).into())?;
        }
        _ => (),
    }
    Ok(())
}

fn process_json_source(cache: &mut Cache, entry: &Path) -> Result<()> {
    let path = &entry.display().to_string();
    let file_stem = entry.file_stem().map(|s| s.to_string_lossy().into_owned());
    if let Some(stem) = file_stem.as_ref() {
        match stem.as_str() {
            "licences" => {
                let file = File::open(path)?;
                let reader = BufReader::new(file);
                let resources: Vec<Licence> = serde_json::from_reader(reader)?;

                for resource in &resources {
                    cache.add(resource)?;
                }

                info!("licence set {}", &path);
            }
            "organisations" => {
                let file = File::open(path)?;
                let reader = BufReader::new(file);
                let resources: Vec<Organisation> = serde_json::from_reader(reader)?;

                for resource in &resources {
                    cache.add(resource)?;
                }

                info!("organisation set {}", &path);
            }
            _ => {
                warn!("unprocessed {}", &path);
            }
        }
    }

    Ok(())
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
