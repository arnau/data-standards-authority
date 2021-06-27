use anyhow::Result;
use clap::Clap;
use log::info;
use std::path::PathBuf;

use crate::cache::{Cache, Strategy};
use crate::{source, zola};

type Achievement = String;

/// Manages bulletins
#[derive(Debug, Clap)]
pub struct Cmd {
    /// Cache path.
    #[clap(long, value_name = "path", default_value = ":memory:")]
    cache_path: Strategy,
    /// The path to the source to build from.
    #[clap(long, short = 'i', value_name = "path")]
    input_path: PathBuf,
    /// The path to the sink to build into.
    #[clap(long, short = 'o', value_name = "path")]
    output_path: PathBuf,
}

impl Cmd {
    pub fn run(&self) -> Result<Achievement> {
        let mut cache = Cache::connect_with_strategy(self.cache_path.clone())?;

        info!("Reading source");
        source::read(&self.input_path, &mut cache)?;
        info!("Writing zola");
        zola::write(&self.output_path, &mut cache)?;

        Ok("".into())
    }
}
