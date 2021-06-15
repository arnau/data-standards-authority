use anyhow::Result;
use rusqlite::{params, Transaction};

#[derive(Debug, Clone)]
pub struct TopicRecord {
    pub id: String,
    pub checksum: String,
    pub name: String,
    pub description: String,
    pub theme_id: String,
    pub ordinal: u32,
}

impl TopicRecord {
    pub(crate) fn select(tx: &Transaction, id: &str) -> Result<Option<TopicRecord>> {
        let mut stmt = tx.prepare(
            r#"
            SELECT
                *
            FROM
                topic
            WHERE
                id = ?;
        "#,
        )?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            let result = TopicRecord {
                id: row.get(0)?,
                checksum: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                theme_id: row.get(4)?,
                ordinal: row.get(5)?,
            };
            return Ok(Some(result));
        }

        Ok(None)
    }

    pub(crate) fn delete(tx: &Transaction, id: &str) -> Result<()> {
        let mut stmt = tx.prepare(
            r#"
            DELETE FROM
                topic
            WHERE
                id = ?;
        "#,
        )?;

        stmt.execute(params![id])?;

        Ok(())
    }

    pub(crate) fn insert(tx: &Transaction, record: &TopicRecord) -> Result<()> {
        let values = params![
            &record.id,
            &record.checksum,
            &record.name,
            &record.description,
            &record.theme_id,
            &record.ordinal,
        ];
        let mut stmt = tx.prepare(
            r#"
            INSERT INTO topic (
                id,
                checksum,
                name,
                description,
                theme_id,
                ordinal
            )
            VALUES (?, ?, ?, ?, ?, ?);
        "#,
        )?;

        stmt.execute(values)?;

        Ok(())
    }
}
