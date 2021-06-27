use anyhow::Result;
use rusqlite::{params, Row, Transaction};

#[derive(Debug, Clone)]
pub struct StandardRecord {
    pub(crate) id: String,
    pub(crate) checksum: String,
    pub(crate) name: String,
    pub(crate) acronym: Option<String>,
    pub(crate) topic_id: String,
    pub(crate) specification: String,
    pub(crate) licence_id: Option<String>,
    pub(crate) maintainer_id: String,
    pub(crate) content: String,
}

fn into_record(row: &Row) -> Result<StandardRecord> {
    let record = StandardRecord {
        id: row.get(0)?,
        checksum: row.get(1)?,
        name: row.get(2)?,
        acronym: row.get(3)?,
        topic_id: row.get(4)?,
        specification: row.get(5)?,
        licence_id: row.get(6)?,
        maintainer_id: row.get(7)?,
        content: row.get(8)?,
    };

    Ok(record)
}

impl StandardRecord {
    pub(crate) fn select_all(tx: &Transaction) -> Result<Vec<StandardRecord>> {
        let mut stmt = tx.prepare(
            r#"
            SELECT
                *
            FROM
                standard;
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

    pub(crate) fn select(tx: &Transaction, standard_id: &str) -> Result<Option<StandardRecord>> {
        let mut stmt = tx.prepare(
            r#"
            SELECT
                *
            FROM
                standard
            WHERE
                id = ?;
        "#,
        )?;
        let mut rows = stmt.query(params![standard_id])?;

        if let Some(row) = rows.next()? {
            let result = into_record(&row)?;
            return Ok(Some(result));
        }

        Ok(None)
    }

    pub(crate) fn delete(tx: &Transaction, standard_id: &str) -> Result<()> {
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

    pub(crate) fn insert(tx: &Transaction, record: &StandardRecord) -> Result<()> {
        let values = params![
            &record.id,
            &record.checksum,
            &record.name,
            &record.acronym,
            &record.topic_id,
            &record.specification,
            &record.licence_id,
            &record.maintainer_id,
            &record.content,
        ];
        let mut stmt = tx.prepare(
            r#"
            INSERT INTO standard
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);
        "#,
        )?;

        stmt.execute(values)?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RelatedStandardRecord {
    pub(crate) standard_id: String,
    pub(crate) related_standard_id: String,
}

impl RelatedStandardRecord {
    pub(crate) fn select(
        tx: &Transaction,
        standard_id: &str,
    ) -> Result<Vec<RelatedStandardRecord>> {
        let mut stmt = tx.prepare(
            r#"
            SELECT
                *
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

    pub(crate) fn insert(tx: &Transaction, record: &RelatedStandardRecord) -> Result<()> {
        let values = params![&record.standard_id, &record.related_standard_id];
        let mut stmt = tx.prepare(
            r#"
            INSERT INTO related_standard
            VALUES (?, ?);
        "#,
        )?;

        stmt.execute(values)?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct EndorsementStateRecord {
    pub(crate) standard_id: String,
    pub(crate) status: String,
    pub(crate) start_date: String,
    pub(crate) review_date: String,
    pub(crate) end_date: Option<String>,
}

impl EndorsementStateRecord {
    pub(crate) fn select(
        tx: &Transaction,
        standard_id: &str,
    ) -> Result<Option<EndorsementStateRecord>> {
        let mut stmt = tx.prepare(
            r#"
            SELECT
                *
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

    pub(crate) fn insert(tx: &Transaction, record: &EndorsementStateRecord) -> Result<()> {
        let values = params![
            &record.standard_id,
            &record.status,
            &record.start_date,
            &record.review_date,
            &record.end_date,
        ];
        let mut stmt = tx.prepare(
            r#"
            INSERT INTO endorsement_state
            VALUES (?, ?, ?, ?, ?);
        "#,
        )?;

        stmt.execute(values)?;

        Ok(())
    }
}
