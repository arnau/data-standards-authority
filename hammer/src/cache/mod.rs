use anyhow::{self, Result};
use chrono::{DateTime, Utc};
pub use rusqlite::Transaction;
use rusqlite::{self, params, Connection};
use std::str::FromStr;

pub(crate) mod records;
pub use records::*;
mod strategy;
pub use strategy::Strategy;

use crate::report::{Action, Entity, Report};

/// A Cache storage.
#[derive(Debug)]
pub struct Cache {
    pub timestamp: DateTime<Utc>,
    pub conn: Connection,
    pub strategy: Strategy,
    pub report: Report,
}

impl Cache {
    pub fn connect_with_strategy(strategy: Strategy) -> Result<Cache> {
        let mut report = Report::new();
        let timestamp = Utc::now();
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
            &strategy.to_string(),
            "Cache bootstrap.",
        );

        Ok(Cache {
            timestamp,
            conn,
            strategy,
            report,
        })
    }
    pub fn connect(path: &str) -> Result<Cache> {
        let strategy = Strategy::from_str(path)?;
        Self::connect_with_strategy(strategy)
    }

    pub fn disconnect(&self) -> Result<()> {
        if let Strategy::Disk(_) = self.strategy {
            self.conn
                .pragma_update(None, "wal_checkpoint", &"restart")?;
            self.conn.pragma_update(None, "journal_mode", &"delete")?;
        }

        Ok(())
    }

    pub fn report(&self) -> &Report {
        &self.report
    }

    pub fn transaction(&mut self) -> Result<Transaction> {
        let tx = self.conn.transaction()?;

        Ok(tx)
    }

    /// Remove all stale records for the given session.
    pub fn prune(&mut self) -> Result<()> {
        let tx = self.conn.transaction()?;

        Cache::delete_stale_standards(&tx, &timestamp_string(&self.timestamp))?;

        &self.report.log(
            Action::Prune,
            Entity::Cache,
            &self.strategy.to_string(),
            "Remove all stale records from the cache.",
        );

        tx.commit()?;

        Ok(())
    }

    /// Deletes all trails from past sessions but the latesT.
    pub fn drain_trail(&mut self) -> Result<()> {
        let tx = self.conn.transaction()?;

        Cache::delete_old_trailmarks(&tx, &timestamp_string(&self.timestamp))?;

        &self.report.log(
            Action::Prune,
            Entity::Cache,
            &self.strategy.to_string(),
            "Remove all stale records from the session trail.",
        );

        tx.commit()?;

        Ok(())
    }

    pub(crate) fn delete_old_trailmarks(tx: &Transaction, timestamp: &str) -> Result<()> {
        let values = params![timestamp];
        let mut stmt = tx.prepare(
            r#"
            DELETE FROM
                session_trail
            WHERE
                timestamp <> ?
        "#,
        )?;

        stmt.execute(values)?;

        Ok(())
    }

    pub(crate) fn insert_trailmark(
        tx: &Transaction,
        checksum: &str,
        resource_type: &str,
        timestamp: &DateTime<Utc>,
    ) -> Result<()> {
        let values = params![checksum, resource_type, &timestamp_string(timestamp)];
        let mut stmt = tx.prepare(
            r#"
            INSERT OR IGNORE INTO
                session_trail (
                    checksum,
                    resource_type,
                    timestamp
                )
            VALUES
                (?, ?, ?)
        "#,
        )?;

        stmt.execute(values)?;

        Ok(())
    }

    /// Selects all standard checksum that are not present in the given session trail.
    #[allow(dead_code)]
    pub(crate) fn select_stale_standards(tx: &Transaction, timestamp: &str) -> Result<Vec<String>> {
        let values = params![timestamp];
        let mut stmt = tx.prepare(
            r#"
            SELECT
                checksum
            FROM
                standard
            WHERE
                checksum NOT IN (
                    SELECT
                        checksum
                    FROM
                        session_trail
                    WHERE
                        resource_type = "standard"
                    AND
                        timestamp = ?
                )
        "#,
        )?;

        let mut rows = stmt.query(values)?;
        let mut list = Vec::new();

        while let Some(row) = rows.next()? {
            let s: String = row.get(0)?;
            list.push(s);
        }

        Ok(list)
    }

    /// Deletes all standard records that are not present in the given session trail.
    ///
    /// Use [`Cache.prune`] for a full cleanup.
    pub(crate) fn delete_stale_standards(tx: &Transaction, timestamp: &str) -> Result<()> {
        let values = params![timestamp];
        let mut stmt = tx.prepare(
            r#"
            DELETE FROM
                standard
            WHERE
                checksum IN (
                    SELECT
                        checksum
                    FROM
                        standard
                    WHERE
                        checksum NOT IN (
                            SELECT
                                checksum
                            FROM
                                session_trail
                            WHERE
                                resource_type = "standard"
                            AND
                                timestamp = ?
                        )
                )
        "#,
        )?;

        stmt.execute(values)?;

        Ok(())
    }
}

pub fn timestamp_string(timestamp: &DateTime<Utc>) -> String {
    timestamp.to_rfc3339()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
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
        let cache = Cache::connect(&file_path.display().to_string());

        assert!(
            cache.is_ok(),
            "Failed whilst connecting to a disk-based cache."
        );
    }
}
