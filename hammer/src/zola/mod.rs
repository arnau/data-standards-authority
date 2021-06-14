//! This module implements the specifics for operating on data shaped for [Zola].
//!
//! [Zola]: https://www.getzola.org/

mod endorsement;
mod licence;
mod organisation;
mod standard;
mod taxonomy;
pub use endorsement::EndorsementState;
pub use licence::Licence;
pub use organisation::Organisation;
pub use standard::Standard;
pub use taxonomy::Topic;

type StandardId = String;
type LicenceId = String;
type GuidanceId = String;
type TopicId = String;
type Url = String;
type Date = String;
