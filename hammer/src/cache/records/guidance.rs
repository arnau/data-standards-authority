use anyhow::Result;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use rusqlite::{params, Transaction};
use std::str::FromStr;

use crate::{Status, StatusError};

#[derive(Debug, Clone)]
pub struct GuidanceRecord {
    pub(crate) id: String,
    pub(crate) checksum: String,
    pub(crate) description: Option<String>,
    pub(crate) maintainer_id: String,
    pub(crate) status: Status,
    pub(crate) creation_date: String,
    pub(crate) update_date: String,
    pub(crate) publication_date: Option<String>,
    pub(crate) canonical_url: Option<String>,
    pub(crate) content: String,
}

impl GuidanceRecord {
    pub(crate) fn select(tx: &Transaction, id: &str) -> Result<Option<GuidanceRecord>> {
        let mut stmt = tx.prepare(
            r#"
            SELECT
                *
            FROM
                guidance
            WHERE
                id = ?;
        "#,
        )?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            let result = GuidanceRecord {
                id: row.get(0)?,
                checksum: row.get(1)?,
                description: row.get(2)?,
                maintainer_id: row.get(3)?,
                status: row.get(4)?,
                creation_date: row.get(5)?,
                update_date: row.get(6)?,
                publication_date: row.get(7)?,
                canonical_url: row.get(8)?,
                content: row.get(9)?,
            };
            return Ok(Some(result));
        }

        Ok(None)
    }

    pub(crate) fn delete(tx: &Transaction, id: &str) -> Result<()> {
        let mut stmt = tx.prepare(
            r#"
            DELETE FROM
                guidance
            WHERE
                id = ?;
        "#,
        )?;

        stmt.execute(params![id])?;

        Ok(())
    }

    pub(crate) fn insert(tx: &Transaction, record: &GuidanceRecord) -> Result<()> {
        let values = params![
            &record.id,
            &record.checksum,
            &record.description,
            &record.maintainer_id,
            &record.status,
            &record.creation_date,
            &record.update_date,
            &record.publication_date,
            &record.canonical_url,
            &record.content,
        ];
        let mut stmt = tx.prepare(
            r#"
            INSERT INTO guidance
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?);
        "#,
        )?;

        stmt.execute(values)?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct GuidanceStandardRecord {
    pub(crate) guidance_id: String,
    pub(crate) standard_id: String,
}

impl GuidanceStandardRecord {
    pub(crate) fn select(tx: &Transaction, id: &str) -> Result<Vec<GuidanceStandardRecord>> {
        let mut stmt = tx.prepare(
            r#"
            SELECT
                *
            FROM
                guidance_standard
            WHERE
                guidance_id = ?;
        "#,
        )?;
        let mut rows = stmt.query(params![id])?;
        let mut list = Vec::new();

        while let Some(row) = rows.next()? {
            let result = GuidanceStandardRecord {
                guidance_id: row.get(0)?,
                standard_id: row.get(1)?,
            };

            list.push(result);
        }

        Ok(list)
    }

    pub(crate) fn insert(tx: &Transaction, record: &GuidanceStandardRecord) -> Result<()> {
        let values = params![&record.guidance_id, &record.standard_id];
        let mut stmt = tx.prepare(
            r#"
            INSERT INTO guidance_standard
            VALUES (?, ?);
        "#,
        )?;

        stmt.execute(values)?;

        Ok(())
    }
}

impl FromSql for Status {
    #[inline]
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value.as_str().and_then(|s| match Status::from_str(s) {
            Ok(s) => Ok(s),
            // TODO: make StatusError more expressive.
            Err(_err) => {
                let e = StatusError;
                Err(FromSqlError::Other(Box::new(e)))
            }
        })
    }
}

impl ToSql for Status {
    #[inline]
    fn to_sql(&self) -> std::result::Result<ToSqlOutput<'_>, rusqlite::Error> {
        let s = self.to_string();
        Ok(ToSqlOutput::from(s))
    }
}
