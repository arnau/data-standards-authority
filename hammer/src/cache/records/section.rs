use anyhow::Result;
use rusqlite::{params, Transaction};

#[derive(Debug, Clone)]
pub struct SectionRecord {
    pub id: String,
    pub checksum: String,
    pub resource_type: String,
    pub content: String,
}

impl SectionRecord {
    pub(crate) fn select_all(tx: &Transaction) -> Result<Vec<SectionRecord>> {
        let mut stmt = tx.prepare(
            r#"
            SELECT
                *
            FROM
                section;
        "#,
        )?;
        let mut rows = stmt.query(params![])?;
        let mut result = Vec::new();

        while let Some(row) = rows.next()? {
            let record = SectionRecord {
                id: row.get(0)?,
                checksum: row.get(1)?,
                resource_type: row.get(2)?,
                content: row.get(3)?,
            };
            result.push(record);
        }

        Ok(result)
    }

    pub(crate) fn select(tx: &Transaction, id: &str) -> Result<Option<SectionRecord>> {
        let mut stmt = tx.prepare(
            r#"
            SELECT
                *
            FROM
                section
            WHERE
                id = ?;
        "#,
        )?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            let result = SectionRecord {
                id: row.get(0)?,
                checksum: row.get(1)?,
                resource_type: row.get(2)?,
                content: row.get(3)?,
            };
            return Ok(Some(result));
        }

        Ok(None)
    }

    pub(crate) fn delete(tx: &Transaction, id: &str) -> Result<()> {
        let mut stmt = tx.prepare(
            r#"
            DELETE FROM
                section
            WHERE
                id = ?;
        "#,
        )?;

        stmt.execute(params![id])?;

        Ok(())
    }

    pub(crate) fn insert(tx: &Transaction, record: &SectionRecord) -> Result<()> {
        let values = params![
            &record.id,
            &record.checksum,
            &record.resource_type,
            &record.content,
        ];
        let mut stmt = tx.prepare(
            r#"
            INSERT INTO section (
                id,
                checksum,
                resource_type,
                content
            )
            VALUES (?, ?, ?, ?);
        "#,
        )?;

        stmt.execute(values)?;

        Ok(())
    }
}
