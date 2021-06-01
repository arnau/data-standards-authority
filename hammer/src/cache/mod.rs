use anyhow::{self, Result};
pub use rusqlite::Transaction;
use rusqlite::{self, params, Connection};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::report::{Action, Entity, Report};

#[derive(Debug, Clone, PartialEq)]
pub enum Strategy {
    Memory,
    Disk(PathBuf),
}

impl FromStr for Strategy {
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
    pub conn: Connection,
    pub strategy: Strategy,
    pub report: Report,
}

impl Cache {
    pub fn connect(path: &str) -> Result<Cache> {
        let strategy = Strategy::from_str(path)?;
        let mut report = Report::new();
        let conn = match &strategy {
            Strategy::Disk(path) => {
                let conn = Connection::open(path)?;
                conn.pragma_update(None, "journal_mode", &"wal")?;
                conn.pragma_update(None, "foreign_keys", &"on")?;
                conn
            }
            Strategy::Memory => Connection::open_in_memory()?,
        };
        let bootstrap = include_str!("../sql/cache.sql");

        conn.execute_batch(&bootstrap)?;
        report.log(
            Action::Chore,
            Entity::Cache,
            "bootstrap",
            "Cache bootstrap.",
        );

        Ok(Cache {
            conn,
            strategy,
            report,
        })
    }

    pub fn disconnect(&self) -> Result<()> {
        if let Strategy::Disk(_) = self.strategy {
            self.conn
                .pragma_update(None, "wal_checkpoint", &"restart")?;
            self.conn.pragma_update(None, "journal_mode", &"delete")?;
        }

        Ok(())
    }

    /// Retrieves the standard by its id.
    pub fn read_standard(tx: &Transaction, standard_id: &str) -> Result<Option<StandardRecord>> {
        let mut stmt = tx.prepare(
            r#"
            SELECT
                id,
                checksum,
                name,
                acronym,
                topic,
                specification,
                licence,
                maintainer,
                content
            FROM
                standard
            WHERE
                id = ?;
        "#,
        )?;
        let mut rows = stmt.query(params![standard_id])?;

        if let Some(row) = rows.next()? {
            let result = StandardRecord {
                id: row.get(0)?,
                checksum: row.get(1)?,
                name: row.get(2)?,
                acronym: row.get(3)?,
                topic: row.get(4)?,
                specification: row.get(5)?,
                licence: row.get(6)?,
                maintainer: row.get(7)?,
                content: row.get(8)?,
            };
            return Ok(Some(result));
        }

        Ok(None)
    }

    pub fn delete_standard(tx: &Transaction, standard_id: &str) -> Result<()> {
        let mut stmt = tx.prepare(
            r#"
            DELETE FROM
                standard
            WHERE
                id = ?;
        "#,
        )?;

        stmt.execute(params![standard_id])?;

        Ok(())
    }

    pub fn insert_standard(tx: &Transaction, record: &StandardRecord) -> Result<()> {
        let values = params![
            &record.id,
            &record.checksum,
            &record.name,
            &record.acronym,
            &record.topic,
            &record.specification,
            &record.licence,
            &record.maintainer,
            &record.content,
        ];
        let mut stmt = tx.prepare(
            r#"
            INSERT INTO standard (
                id,
                checksum,
                name,
                acronym,
                topic,
                specification,
                licence,
                maintainer,
                content
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);
        "#,
        )?;

        stmt.execute(values)?;

        Ok(())
    }

    pub fn read_related_standards(
        tx: &Transaction,
        standard_id: &str,
    ) -> Result<Vec<RelatedStandardRecord>> {
        let mut stmt = tx.prepare(
            r#"
            SELECT
                standard_id,
                related_standard_id
            FROM
                related_standard
            WHERE
                standard_id = ?;
        "#,
        )?;
        let mut rows = stmt.query(params![standard_id])?;
        let mut list = Vec::new();

        while let Some(row) = rows.next()? {
            let result = RelatedStandardRecord {
                standard_id: row.get(0)?,
                related_standard_id: row.get(1)?,
            };

            list.push(result);
        }

        Ok(list)
    }

    pub fn insert_related_standard(tx: &Transaction, record: &RelatedStandardRecord) -> Result<()> {
        let values = params![&record.standard_id, &record.related_standard_id];
        let mut stmt = tx.prepare(
            r#"
            INSERT INTO related_standard (
                standard_id,
                related_standard_id
            )
            VALUES (?, ?);
        "#,
        )?;

        stmt.execute(values)?;

        Ok(())
    }

    pub fn read_endorsement_state(
        tx: &Transaction,
        standard_id: &str,
    ) -> Result<Option<EndorsementStateRecord>> {
        let mut stmt = tx.prepare(
            r#"
            SELECT
                standard_id,
                status,
                start_date,
                review_date,
                end_date
            FROM
                endorsement_state
            WHERE
                standard_id = ?;
        "#,
        )?;
        let mut rows = stmt.query(params![standard_id])?;

        if let Some(row) = rows.next()? {
            let result = EndorsementStateRecord {
                standard_id: row.get(0)?,
                status: row.get(1)?,
                start_date: row.get(2)?,
                review_date: row.get(3)?,
                end_date: row.get(4)?,
            };
            return Ok(Some(result));
        }

        Ok(None)
    }

    pub fn insert_endorsement_state(
        tx: &Transaction,
        record: &EndorsementStateRecord,
    ) -> Result<()> {
        let values = params![
            &record.standard_id,
            &record.status,
            &record.start_date,
            &record.review_date,
            &record.end_date,
        ];
        let mut stmt = tx.prepare(
            r#"
            INSERT INTO endorsement_state (
                standard_id,
                status,
                start_date,
                review_date,
                end_date
            )
            VALUES (?, ?, ?, ?, ?);
        "#,
        )?;

        stmt.execute(values)?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct StandardRecord {
    pub id: String,
    pub checksum: String,
    pub name: String,
    pub acronym: Option<String>,
    pub topic: String,
    pub specification: String,
    pub licence: Option<String>,
    pub maintainer: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct RelatedStandardRecord {
    pub standard_id: String,
    pub related_standard_id: String,
}

#[derive(Debug, Clone)]
pub struct EndorsementStateRecord {
    pub standard_id: String,
    pub status: String,
    pub start_date: String,
    pub review_date: String,
    pub end_date: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
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
            "Failed whilst connecting to a disk-based cache."
        );
    }
}
