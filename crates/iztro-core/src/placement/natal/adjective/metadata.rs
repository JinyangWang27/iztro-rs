//! Factual metadata passthroughs for the supported adjective-star subset.

use crate::model::star::{StarMetadata, StarName};

/// Returns factual metadata for the supported adjective-star subset.
pub const fn adjective_star_metadata_table() -> &'static [StarMetadata] {
    crate::model::star::adjective_star_metadata_table()
}

/// Returns factual metadata for one supported adjective star.
pub fn adjective_star_metadata(star: StarName) -> &'static StarMetadata {
    crate::model::star::adjective_star_metadata(star)
}

/// Returns factual metadata for one adjective star, if it is represented.
pub fn try_adjective_star_metadata(star: StarName) -> Option<&'static StarMetadata> {
    crate::model::star::try_adjective_star_metadata(star)
}
