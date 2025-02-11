//! This module covers the standard card from a Zola point of view.
use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::Transaction;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use super::{EndorsementState, Licence, Organisation, StandardId, TopicReference, Url};
use crate::cache::records::*;
use crate::cache::Cache;
use crate::checksum::{Checksum, Digest, Hasher};
use crate::report;
use crate::resource::Resource;

#[derive(Debug, Clone)]
pub struct Standard {
    pub metadata: Metadata,
    pub content: String,
}

impl Standard {
    pub fn id(&self) -> &StandardId {
        &self.metadata.extra.id
    }

    pub fn checksum(&self) -> Checksum {
        self.into()
    }

    pub fn path(&self) -> String {
        format!("{}.md", self.id())
    }
}

impl fmt::Display for Standard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let metadata = toml::to_string(&self.metadata).expect("metadata to serialize as TOML");

        writeln!(f, "+++")?;
        write!(f, "{}", &metadata)?;
        writeln!(f, "+++")?;
        write!(f, "{}", &self.content)
    }
}

impl Digest for Standard {
    fn digest(&self, hasher: &mut Hasher) {
        self.metadata.digest(hasher);
        self.content.digest(hasher);
    }
}

impl From<&Standard> for Checksum {
    fn from(resource: &Standard) -> Checksum {
        let mut hasher = Hasher::new();
        resource.digest(&mut hasher);

        hasher.finalize()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metadata {
    title: String,
    date: DateTime<Utc>,
    slug: String,
    template: String,
    extra: MetadataExtra,
}

impl Digest for Metadata {
    fn digest(&self, hasher: &mut Hasher) {
        self.extra.id.digest(hasher);
        self.extra.name.digest(hasher);
        self.extra.acronym.digest(hasher);
        self.extra.topic.digest(hasher);
        self.extra.specification.digest(hasher);
        self.extra
            .licence
            .clone()
            .map(|x| x.id().clone())
            .digest(hasher);
        self.extra.maintainer.id().digest(hasher);
        self.extra
            .related
            .iter()
            .map(|x| x.id.clone())
            .collect::<Vec<_>>()
            .digest(hasher);
        self.extra.endorsement_state.digest(hasher);
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MetadataExtra {
    /// A local unique identifier for the standard.
    #[serde(rename = "identifier")]
    pub id: StandardId,
    /// The name of the standard.
    pub name: String,
    /// The well-known acronym of the standard.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acronym: Option<String>,
    /// The URL to the technical specification for the standard.
    pub specification: Url,
    /// The list of related standards.
    #[serde(default)]
    pub related: Vec<RelatedStandard>,
    /// The topic used to classify the standard.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<TopicReference>,
    /// The licence the standard (or specification) is licensed under.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub licence: Option<Licence>,
    /// The organisation maintaining the specification.
    pub maintainer: Organisation,
    pub endorsement_state: EndorsementState,
}

/// A reference to a related standard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedStandard {
    id: String,
    name: String,
}

impl Digest for RelatedStandard {
    fn digest(&self, hasher: &mut Hasher) {
        self.id.digest(hasher);
    }
}

impl Resource<Standard> for Cache {
    fn get(&mut self, id: &str) -> Result<Option<Standard>> {
        let tx = self.conn.transaction()?;
        let mut result = None;

        if let Some(record) = StandardRecord::select(&tx, id)? {
            result = Some(into_resource(&tx, record)?);
        }

        &self
            .report
            .log(report::Action::Get, report::Entity::Standard, id, "");

        tx.commit()?;

        Ok(result)
    }

    fn add(&mut self, _resource: &Standard) -> Result<()> {
        unimplemented!()
    }

    fn drop(&mut self, _id: &str) -> Result<Option<Standard>> {
        unimplemented!()
    }
}

pub fn get_all(cache: &mut Cache) -> Result<Vec<Standard>> {
    let tx = cache.transaction()?;
    let records = StandardRecord::select_all(&tx)?;
    let mut result = Vec::new();

    for record in records {
        let resource = into_resource(&tx, record)?;

        result.push(resource);
    }

    tx.commit()?;

    Ok(result)
}

fn into_resource(tx: &Transaction, record: StandardRecord) -> Result<Standard> {
    let standard_id = &record.id;
    let related_records = RelatedStandardRecord::select(tx, &standard_id)?;
    let endorsement_record = EndorsementStateRecord::select(tx, &standard_id)?
        .expect("missing endorsement state. the cache is corrupted.");
    let mut related: Vec<RelatedStandard> = Vec::new();

    for related_record in related_records {
        if let Some(std_record) = StandardRecord::select(&tx, &related_record.related_standard_id)?
        {
            related.push(RelatedStandard {
                id: std_record.id,
                name: std_record.name,
            });
        }
    }

    let licence = if let Some(licence_id) = record.licence_id {
        LicenceRecord::select(&tx, &licence_id)?
    } else {
        None
    };
    let maintainer =
        OrganisationRecord::select(&tx, &record.maintainer_id)?.expect("maintainer to exist");
    let topic = TopicRecord::select(&tx, &record.topic_id)?.map(|record| TopicReference {
        id: record.id,
        name: record.name,
        theme: record.theme_id,
    });

    let endorsement_state = EndorsementState {
        status: endorsement_record.status.parse()?,
        start_date: endorsement_record.start_date,
        review_date: endorsement_record.review_date,
        end_date: endorsement_record.end_date,
    };
    let extra = MetadataExtra {
        id: record.id.clone(),
        name: record.name.clone(),
        acronym: record.acronym,
        specification: record.specification,
        topic,
        licence: licence.map(Into::into),
        maintainer: maintainer.into(),
        related,
        endorsement_state: endorsement_state.clone(),
    };
    let date = FromStr::from_str(&format!("{}T00:00:00Z", &endorsement_state.start_date))?;
    let metadata = Metadata {
        title: record.name,
        date,
        slug: record.id,
        template: "standard.html".to_string(),
        extra,
    };
    let standard = Standard {
        metadata,
        content: record.content,
    };

    Ok(standard)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::Cache;
    use crate::source;

    #[test]
    fn from_source() -> Result<()> {
        let vapour_raw = r#"---
type: standard
identifier: vapour
name: Vapour
topic: exchange
subjects:
    - api_access
specification: https://spec.vapour.org/
licence: ogl
maintainer: data-standards-authority
endorsement_state:
    status: identified
    start_date: 2021-06-01
    review_date: 2021-06-01
related:
    - steam
---
This standard will give you no overhead."#;
        let steam_raw = r#"---
type: standard
identifier: steam
name: Steam
topic: exchange
subjects:
    - api_access
specification: https://spec.steam.org/
licence: ogl
maintainer: data-standards-authority
endorsement_state:
    status: identified
    start_date: 2021-06-01
    review_date: 2021-06-01
related:
    - vapour
---
This standard will give you warmth."#;
        let licence_raw = r#"{
            "id": "ogl",
            "name": "Open Government Licence",
            "url": "https://ogl.gov.uk"
        }"#;
        let org_raw = r#"{
            "id": "data-standards-authority",
            "name": "Data Standards Authority",
            "url": "https://dsa.gov.uk"
        }"#;
        let zola_page = r#"+++
title = "Vapour"
date = "2021-06-01T00:00:00Z"
slug = "vapour"
template = "standard.html"

[extra]
identifier = "vapour"
name = "Vapour"
specification = "https://spec.vapour.org/"

[[extra.related]]
id = "steam"
name = "Steam"

[extra.topic]
identifier = "exchange"
name = "Exchange"
theme = "other"

[extra.licence]
id = "ogl"
name = "Open Government Licence"
url = "https://ogl.gov.uk"

[extra.maintainer]
id = "data-standards-authority"
name = "Data Standards Authority"
url = "https://dsa.gov.uk"

[extra.endorsement_state]
status = "identified"
start_date = "2021-06-01"
review_date = "2021-06-01"
+++
This standard will give you no overhead."#;
        let topic_raw = r#"---
type: topic
identifier: exchange
name: Exchange
theme: other
ordinal: 1
---"#;
        let mut cache = Cache::connect(":memory:")?;
        let vapour = source::Standard::from_str(vapour_raw)?;
        let steam = source::Standard::from_str(steam_raw)?;
        let licence = source::Licence::from_str(licence_raw)?;
        let org = source::Organisation::from_str(org_raw)?;
        let topic = source::Topic::from_str(topic_raw)?;

        cache.add(&org)?;
        cache.add(&licence)?;
        cache.add(&steam)?;
        cache.add(&vapour)?;
        cache.add(&topic)?;

        let actual: Standard = cache.get(&vapour.id())?.unwrap();

        assert_eq!(&actual.to_string(), zola_page);

        Ok(())
    }
}
