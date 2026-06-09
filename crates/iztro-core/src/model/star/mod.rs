//! Star facts: stable identifiers, kind/category, brightness, factual metadata,
//! and the Heavenly Stem mutagen (四化) table.

pub mod brightness;
pub mod flow;
pub mod kind;
pub mod metadata;
pub mod mutagen;
pub mod name;

pub use brightness::Brightness;
pub use flow::{FlowStarBase, FlowStarScope, flow_star_name, try_flow_star_parts};
pub use kind::{StarCategory, StarKind};
pub use metadata::{
    KnownStarFamily, KnownStarMetadata, StarMetadata, adjective_star_metadata,
    adjective_star_metadata_table, known_star_metadata, known_star_metadata_table,
    major_star_metadata, major_star_metadata_table, minor_star_metadata, minor_star_metadata_table,
    represented_star_metadata_table, star_metadata, try_adjective_star_metadata,
    try_known_star_metadata, try_major_star_metadata, try_minor_star_metadata, try_star_metadata,
};
pub use name::StarName;
