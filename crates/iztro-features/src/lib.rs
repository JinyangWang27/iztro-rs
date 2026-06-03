//! Feature extraction contracts for iztro-rs charts.

pub mod basic;
pub mod domains;
pub mod extractor;
pub mod mutagen_flows;
pub mod palace_features;
pub mod relations;
pub mod star_features;

pub use basic::BasicFeatureExtractor;
pub use domains::{Domain, domain_for_palace};
pub use extractor::{ChartFeatures, FeatureExtractionError, FeatureExtractor};
pub use mutagen_flows::MutagenFlow;
pub use palace_features::PalaceFeature;
pub use relations::{PalaceRelation, PalaceRelationKind, PalaceRelations, all_palace_relations};
pub use star_features::StarFeature;
