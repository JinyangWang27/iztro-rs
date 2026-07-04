//! Feature extraction contracts for iztro-rs charts.

pub mod basic;
pub mod domains;
pub mod extractor;
pub mod mutagen_flows;
pub mod palace_features;
pub mod palace_stem;
pub mod relations;
pub mod star_features;

pub use basic::BasicFeatureExtractor;
pub use domains::{Domain, domain_for_palace};
pub use extractor::{ChartFeatures, FeatureExtractionError, FeatureExtractor};
pub use mutagen_flows::MutagenFlow;
pub use palace_features::PalaceFeature;
pub use palace_stem::{
    MutagenFlowTarget, PalaceStemMutagenFlow, PalaceStemRole, PalaceStemRoleAssignment,
    PalaceStemSource, birth_year_stem_origin_palaces, mutagen_flows_from_palace,
    mutagen_flows_landing_in_palace, palace_stem_mutagen_flows, palace_stem_role_assignments,
    self_transforming_flows,
};
pub use relations::{
    PalaceNameRelation, PalaceNameRelationKind, PalaceNameRelations, all_palace_relations,
};
// Compatibility aliases for the former feature relation names. The `PalaceName*`
// names above are canonical; these preserve existing `features::PalaceRelation*`
// users.
pub use relations::{PalaceRelation, PalaceRelationKind, PalaceRelations};
pub use star_features::StarFeature;
