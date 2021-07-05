use anyhow::Result;
use rusqlite::{params, Row, Transaction};

#[derive(Debug, Clone)]
pub struct TopicRecord {
    pub id: String,
    pub checksum: String,
    pub name: String,
    pub description: String,
    pub theme_id: String,
    pub ordinal: u32,
}

fn into_record(row: &Row) -> Result<TopicRecord> {
    let record = TopicRecord {
        id: row.get(0)?,
        checksum: row.get(1)?,
        name: row.get(2)?,
        description: row.get(3)?,
        theme_id: row.get(4)?,
        ordinal: row.get(5)?,
    };

    Ok(record)
}

impl TopicRecord {
    pub(crate) fn select_by_theme(tx: &Transaction, theme_id: &str) -> Result<Vec<TopicRecord>> {
        let mut stmt = tx.prepare(
            r#"
            SELECT
                *
            FROM
                topic
            WHERE
                theme_id = ?;
        "#,
        )?;
        let mut rows = stmt.query(params![theme_id])?;
        let mut result = Vec::new();

        while let Some(row) = rows.next()? {
            let record = into_record(&row)?;
            result.push(record);
        }

        Ok(result)
    }

    pub(crate) fn select_all(tx: &Transaction) -> Result<Vec<TopicRecord>> {
        let mut stmt = tx.prepare(
            r#"
            SELECT
                *
            FROM
                topic;
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
            let result = into_record(&row)?;
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

#[derive(Debug, Clone)]
pub struct TopicStandardRecord {
    pub id: String,
    pub name: String,
    pub status: String,
    pub review_date: String,
}

impl TopicStandardRecord {
    pub fn select(tx: &Transaction, topic_id: &str) -> Result<Vec<TopicStandardRecord>> {
        let mut stmt = tx.prepare(
            r#"
            SELECT
                standard.id,
                standard.name,
                endorsement_state.status,
                endorsement_state.review_date
            FROM
                standard
            INNER JOIN endorsement_state ON
                standard.id = endorsement_state.standard_id
            WHERE
                topic_id = ?;
        "#,
        )?;
        let mut rows = stmt.query(params![topic_id])?;
        let mut result = Vec::new();

        while let Some(row) = rows.next()? {
            let record = TopicStandardRecord {
                id: row.get(0)?,
                name: row.get(1)?,
                status: row.get(2)?,
                review_date: row.get(3)?,
            };
            result.push(record);
        }

        Ok(result)
    }
}
