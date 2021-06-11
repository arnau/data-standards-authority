use anyhow::Result;
use rusqlite::{params, Transaction};

#[derive(Debug, Clone)]
pub struct OrganisationRecord {
    pub id: String,
    pub checksum: String,
    pub name: String,
    pub url: String,
}

impl OrganisationRecord {
    pub(crate) fn select(tx: &Transaction, id: &str) -> Result<Option<OrganisationRecord>> {
        let mut stmt = tx.prepare(
            r#"
            SELECT
                *
            FROM
                organisation
            WHERE
                id = ?;
        "#,
        )?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            let result = OrganisationRecord {
                id: row.get(0)?,
                checksum: row.get(1)?,
                name: row.get(2)?,
                url: row.get(3)?,
            };
            return Ok(Some(result));
        }

        Ok(None)
    }

    pub(crate) fn delete(tx: &Transaction, id: &str) -> Result<()> {
        let mut stmt = tx.prepare(
            r#"
            DELETE FROM
                organisation
            WHERE
                id = ?;
        "#,
        )?;

        stmt.execute(params![id])?;

        Ok(())
    }

    pub(crate) fn insert(tx: &Transaction, record: &OrganisationRecord) -> Result<()> {
        let values = params![&record.id, &record.checksum, &record.name, &record.url,];
        let mut stmt = tx.prepare(
            r#"
            INSERT INTO organisation
            VALUES (?, ?, ?, ?);
        "#,
        )?;

        stmt.execute(values)?;

        Ok(())
    }
}
