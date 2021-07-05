use anyhow::Result;
use rusqlite::{params, Row, Transaction};

#[derive(Debug, Clone)]
pub struct ThemeRecord {
    pub id: String,
    pub checksum: String,
    pub name: String,
    pub description: String,
    pub ordinal: u32,
}

fn into_record(row: &Row) -> Result<ThemeRecord> {
    let record = ThemeRecord {
        id: row.get(0)?,
        checksum: row.get(1)?,
        name: row.get(2)?,
        description: row.get(3)?,
        ordinal: row.get(4)?,
    };

    Ok(record)
}

impl ThemeRecord {
    pub(crate) fn select_all(tx: &Transaction) -> Result<Vec<ThemeRecord>> {
        let mut stmt = tx.prepare(
            r#"
            SELECT
                *
            FROM
                theme;
        "#,
        )?;
        let mut rows = stmt.query(params![])?;
        let mut result = Vec::new();

        while let Some(row) = rows.next()? {
            let record = into_record(&row)?;
            result.push(record);
        }

        Ok(result)
    }

    pub(crate) fn select(tx: &Transaction, id: &str) -> Result<Option<ThemeRecord>> {
        let mut stmt = tx.prepare(
            r#"
            SELECT
                *
            FROM
                theme
            WHERE
                id = ?;
        "#,
        )?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            let result = into_record(&row)?;
            return Ok(Some(result));
        }

        Ok(None)
    }

    pub(crate) fn delete(tx: &Transaction, id: &str) -> Result<()> {
        let mut stmt = tx.prepare(
            r#"
            DELETE FROM
                theme
            WHERE
                id = ?;
        "#,
        )?;

        stmt.execute(params![id])?;

        Ok(())
    }

    pub(crate) fn insert(tx: &Transaction, record: &ThemeRecord) -> Result<()> {
        let values = params![
            &record.id,
            &record.checksum,
            &record.name,
            &record.description,
            &record.ordinal,
        ];
        let mut stmt = tx.prepare(
            r#"
            INSERT INTO theme (
                id,
                checksum,
                name,
                description,
                ordinal
            )
            VALUES (?, ?, ?, ?, ?);
        "#,
        )?;

        stmt.execute(values)?;

        Ok(())
    }
}
