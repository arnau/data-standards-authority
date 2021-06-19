use anyhow::Result;
use hammer::resource::Resource;
use hammer::source::{Licence, Standard};
use hammer::Cache;
use std::fs::read_dir;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

/// Markdown based resoruce types.
#[derive(Debug, Clone)]
pub enum ResourceType {
    Standard,
    Topic,
    Section,
    Guidance,
    Unknown,
}

impl ResourceType {
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

use std::fmt;

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ResourceType::Standard => "standard",
            ResourceType::Section => "section",
            ResourceType::Topic => "topic",
            ResourceType::Guidance => "guidance",
            ResourceType::Unknown => "unknown",
        };

        write!(f, "{}", s)
    }
}

fn main() -> Result<()> {
    let source_dir = "../corpus";
    let mut cache = Cache::connect("./cache.db")?;
    let walker = WalkDir::new(source_dir).into_iter();

    for result in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = result?;

        if let Some(ext) = entry.path().extension() {
            if ext == "md" {
                let mut file = File::open(&entry.path())?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                match ResourceType::from_hint(&contents) {
                    ResourceType::Unknown => {
                        println!("unknown resource type: {}", &entry.path().display());
                    }
                    typ => {
                        println!("{}, {}", &entry.path().display(), typ);
                    }
                }
            }
        }
    }

    cache.prune()?;
    // cache.drain_trail()?;

    dbg!(cache.report());

    Ok(())
}
