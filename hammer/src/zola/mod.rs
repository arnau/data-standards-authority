//! This module implements the specifics for operating on data shaped for [Zola].
//!
//! [Zola]: https://www.getzola.org/

mod endorsement;
mod guidance;
mod licence;
mod organisation;
pub mod section;
mod standard;
mod taxonomy;
mod theme;
mod topic;

pub use endorsement::EndorsementState;
pub use guidance::Guidance;
pub use licence::Licence;
pub use organisation::Organisation;
pub use section::Section;
pub use standard::Standard;
pub use taxonomy::TopicReference;
pub use theme::Theme;
pub use topic::Topic;

use anyhow::Result;
use log::{info, warn};
use std::fs;
use std::path::Path;

use crate::cache::Cache;
use crate::resource::ResourceType;

type StandardId = String;
type LicenceId = String;
type GuidanceId = String;
type TopicId = String;
type ThemeId = String;
type Url = String;
type Date = String;

pub fn write(sink_dir: &Path, cache: &mut Cache) -> Result<()> {
    let sections = section::get_all(cache)?;

    // Agressively clean previous build.
    if sink_dir.exists() {
        fs::remove_dir_all(sink_dir)?;
    }
    fs::create_dir(sink_dir)?;

    for section in sections {
        let section_path = sink_dir.join(&section.path());
        let resource_type = section.resource_type()?;

        fs::create_dir(&section_path)?;
        fs::write(&section_path.join("_index.md"), &section.to_string())?;

        match resource_type {
            ResourceType::Standard => {
                info!("Write standard set");
                let resources = standard::get_all(cache)?;
                for resource in resources {
                    let resource_path = section_path.join(&resource.path());
                    fs::write(&resource_path, &resource.to_string())?;
                }
            }
            ResourceType::Guidance => {
                info!("Write guidance set");
                let resources = guidance::get_all(cache)?;
                for resource in resources {
                    let resource_path = section_path.join(&resource.path());
                    fs::write(&resource_path, &resource.to_string())?;
                }
            }
            ResourceType::Theme => {
                info!("Write theme set");
                let resources = theme::get_all(cache)?;
                for resource in resources {
                    let resource_path = section_path.join(&resource.path());
                    fs::create_dir(&resource_path)?;
                    fs::write(&resource_path.join("_index.md"), &resource.to_string())?;

                    info!("Write {} topics set", &resource.id());
                    let subresources = topic::get_all(cache, &resource.id())?;
                    for subresource in subresources {
                        let subresource_path = resource_path.join(&subresource.path());
                        fs::write(&subresource_path, &subresource.to_string())?;
                    }
                }
            }

            typ => {
                warn!("'{}' is an unimplemented zola resource", typ);
            }
        }
    }

    Ok(())
}
