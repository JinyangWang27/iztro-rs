use crate::{
    domains::{Domain, domain_for_palace},
    extractor::{ChartFeatures, FeatureExtractionError, FeatureExtractor},
    mutagen_flows::MutagenFlow,
    palace_features::PalaceFeature,
    relations::{PalaceRelation, all_palace_relations},
    star_features::StarFeature,
};
use iztro_core::Chart;

/// Deterministic extractor turning chart facts into structured feature facts.
///
/// This is the first feature-extraction slice. It records placement facts only:
/// supported palace domains, a factual star feature for every placed star (with
/// the palace domain as optional metadata), natal mutagen flows for every placed
/// star with a mutagen, and the deterministic cyclic palace relations. It
/// performs no interpretation, emits no claims, and produces no narrative.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct BasicFeatureExtractor;

impl FeatureExtractor for BasicFeatureExtractor {
    fn extract(&self, chart: &Chart) -> Result<ChartFeatures, FeatureExtractionError> {
        let mut palace_features = Vec::new();
        let mut star_features = Vec::new();
        let mut mutagen_flows = Vec::new();

        for palace in chart.palaces() {
            let domain = domain_for_palace(palace.name());

            if let Some(domain) = domain {
                palace_features.push(PalaceFeature::new(palace.name(), domain));
            }

            for placement in palace.stars() {
                // Star features preserve every placed star fact. The palace
                // domain is optional metadata, so stars in unsupported-domain
                // palaces are still recorded with a `None` domain.
                star_features.push(StarFeature::new(
                    palace.name(),
                    placement.name(),
                    placement.kind(),
                    placement.brightness(),
                    placement.mutagen(),
                    placement.scope(),
                    domain,
                ));

                // Mutagen flows have no domain, so every placed star with a
                // mutagen emits one regardless of its palace domain.
                if let Some(mutagen) = placement.mutagen() {
                    mutagen_flows.push(MutagenFlow::new(
                        palace.name(),
                        placement.name(),
                        mutagen,
                        placement.scope(),
                    ));
                }
            }
        }

        let domains = distinct_domains(&palace_features);
        let relations = all_relations();

        Ok(ChartFeatures::new(
            chart.method_profile().id(),
            domains,
            palace_features,
            star_features,
            mutagen_flows,
            relations,
        ))
    }
}

/// Collects the distinct domains represented by the palace features in
/// first-seen order.
fn distinct_domains(palace_features: &[PalaceFeature]) -> Vec<Domain> {
    let mut domains = Vec::new();
    for feature in palace_features {
        if !domains.contains(&feature.domain()) {
            domains.push(feature.domain());
        }
    }
    domains
}

/// Flattens the deterministic cyclic relations of all twelve palaces into
/// edge-level relation features.
fn all_relations() -> Vec<PalaceRelation> {
    all_palace_relations()
        .iter()
        .flat_map(|relations| relations.to_relations())
        .collect()
}
