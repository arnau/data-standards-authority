use anyhow::{self, Result};
use rusqlite::{params, ToSql};
use rusqlite::{Connection, Error::QueryReturnedNoRows, NO_PARAMS};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Strategy {
    Memory,
    Disk(PathBuf),
}

impl FromStr for Strategy {
    // type Err = &'static str;
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            ":memory:" => Ok(Strategy::Memory),
            s => {
                let path = Path::new(s);
                Ok(Strategy::Disk(path.into()))
            }
        }
    }
}

/// A Cache storage.
#[derive(Debug)]
pub struct Cache {
    conn: Connection,
    strategy: Strategy,
}

impl Cache {
    pub fn connect(path: &str) -> Result<Cache> {
        let strategy = Strategy::from_str(path)?;
        let conn = match &strategy {
            Strategy::Disk(path) => {
                let conn = Connection::open(path)?;
                conn.pragma_update(None, "journal_mode", &"wal")?;
                conn
            }
            Strategy::Memory => Connection::open_in_memory()?,
        };
        let bootstrap = include_str!("../sql/cache.sql");

        conn.execute_batch(&bootstrap)?;

        Ok(Cache { conn, strategy })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use temp_testdir::TempDir;

    #[test]
    fn connect_in_memory() {
        let cache = Cache::connect(":memory:");

        assert!(
            cache.is_ok(),
            "Failed whilst connecting to an in-memory cache."
        );
    }

    #[test]
    fn connect_disk() {
        let temp = TempDir::default();
        let mut file_path = PathBuf::from(temp.as_ref());
        file_path.push("cache.db");
        let cache = Cache::connect(&file_path.into_os_string().into_string().unwrap());

        assert!(
            cache.is_ok(),
            "Failed whilst connecting to an in-memory cache."
        );
    }
}
