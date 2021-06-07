use anyhow::Result;
use hammer::resource::Resource;
use hammer::source::{Licence, Standard};
use hammer::Cache;
use std::str::FromStr;

fn main() -> Result<()> {
    let standards = vec![
        r#"---
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
This standard will give you no overhead."#,
        r#"---
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
This standard will give you warmth."#,
    ];

    let licence_raw = r#"{
      "id": "ogl-3",
      "name": "Open Government License",
      "acronym": "OGL",
      "url": "https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/"
    }"#;

    let mut cache = Cache::connect("./cache.db")?;

    // for text in standards.iter().take(1).collect::<Vec<_>>() {
    for text in standards {
        let standard = Standard::from_str(text)?;
        cache.add(&standard)?;
    }

    let licence = Licence::from_str(licence_raw)?;
    cache.add(&licence)?;

    cache.prune()?;
    // cache.drain_trail()?;

    dbg!(cache.report());

    Ok(())
}
