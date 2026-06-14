use crate::core::Chart;
use crate::features::{
    domains::Domain, mutagen_flows::MutagenFlow, palace_features::PalaceFeature,
    relations::PalaceRelation, star_features::StarFeature,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Aggregated features extracted from chart facts.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ChartFeatures {
    source_profile_id: String,
    domains: Vec<Domain>,
    palace_features: Vec<PalaceFeature>,
    star_features: Vec<StarFeature>,
    mutagen_flows: Vec<MutagenFlow>,
    relations: Vec<PalaceRelation>,
}

impl ChartFeatures {
    /// Creates a feature aggregate.
    pub fn new(
        source_profile_id: impl Into<String>,
        domains: Vec<Domain>,
        palace_features: Vec<PalaceFeature>,
        star_features: Vec<StarFeature>,
        mutagen_flows: Vec<MutagenFlow>,
        relations: Vec<PalaceRelation>,
    ) -> Self {
        Self {
            source_profile_id: source_profile_id.into(),
            domains,
            palace_features,
            star_features,
            mutagen_flows,
            relations,
        }
    }

    /// Returns an empty feature aggregate for a source profile.
    pub fn empty(source_profile_id: impl Into<String>) -> Self {
        Self::new(
            source_profile_id,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
    }

    /// Returns the method profile identifier associated with these features.
    pub fn source_profile_id(&self) -> &str {
        &self.source_profile_id
    }

    /// Returns domains represented by this feature set.
    pub fn domains(&self) -> &[Domain] {
        &self.domains
    }

    /// Returns palace features.
    pub fn palace_features(&self) -> &[PalaceFeature] {
        &self.palace_features
    }

    /// Returns star features.
    pub fn star_features(&self) -> &[StarFeature] {
        &self.star_features
    }

    /// Returns mutagen-flow features.
    pub fn mutagen_flows(&self) -> &[MutagenFlow] {
        &self.mutagen_flows
    }

    /// Returns palace relations.
    pub fn relations(&self) -> &[PalaceRelation] {
        &self.relations
    }
}

/// Extracts semantic features from deterministic chart facts.
pub trait FeatureExtractor {
    /// Extracts a structured feature aggregate from a chart.
    fn extract(&self, chart: &Chart) -> Result<ChartFeatures, FeatureExtractionError>;
}

/// Errors produced by feature extractors.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum FeatureExtractionError {
    /// Feature extraction has not been implemented for this extractor.
    #[error("feature extraction is not implemented")]
    NotImplemented,
}
