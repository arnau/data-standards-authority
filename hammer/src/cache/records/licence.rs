use anyhow::Result;
use rusqlite::{params, Transaction};

#[derive(Debug, Clone)]
pub struct LicenceRecord {
    pub id: String,
    pub checksum: String,
    pub name: String,
    pub acronym: Option<String>,
    pub url: String,
}

impl LicenceRecord {
    pub(crate) fn select(tx: &Transaction, licence_id: &str) -> Result<Option<LicenceRecord>> {
        let mut stmt = tx.prepare(
            r#"
            SELECT
                *
            FROM
                licence
            WHERE
                id = ?;
        "#,
        )?;
        let mut rows = stmt.query(params![licence_id])?;

        if let Some(row) = rows.next()? {
            let result = LicenceRecord {
                id: row.get(0)?,
                checksum: row.get(1)?,
                name: row.get(2)?,
                acronym: row.get(3)?,
                url: row.get(4)?,
            };
            return Ok(Some(result));
        }

        Ok(None)
    }

    pub(crate) fn delete(tx: &Transaction, licence_id: &str) -> Result<()> {
        let mut stmt = tx.prepare(
            r#"
            DELETE FROM
                licence
            WHERE
                id = ?;
        "#,
        )?;

        stmt.execute(params![licence_id])?;

        Ok(())
    }

    pub(crate) fn insert(tx: &Transaction, record: &LicenceRecord) -> Result<()> {
        let values = params![
            &record.id,
            &record.checksum,
            &record.name,
            &record.acronym,
            &record.url,
        ];
        let mut stmt = tx.prepare(
            r#"
            INSERT INTO licence (
                id,
                checksum,
                name,
                acronym,
                url
            )
            VALUES (?, ?, ?, ?, ?);
        "#,
        )?;

        stmt.execute(values)?;

        Ok(())
    }
}
