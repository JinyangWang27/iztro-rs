//! Model-level horoscope runtime helper projections and queries.
//!
//! These helpers mirror the upstream `iztro@2.5.8` `FunctionalHoroscope`
//! runtime behavior without implementing the full upstream horoscope JSON
//! payload. They project existing natal and temporal facts by branch and never
//! mutate natal chart data or create new placements.

use lunar_lite::{EarthlyBranch, HeavenlyStem};
use crate::core::{
    error::ChartError,
    model::{
        chart::{HoroscopeChart, Palace, PalaceName, TemporalLayer},
        star::{
            StarName,
            mutagen::{Mutagen, Scope},
        },
    },
};

const REQUIRED_RUNTIME_SCOPES: [Scope; 6] = [
    Scope::Age,
    Scope::Decadal,
    Scope::Yearly,
    Scope::Monthly,
    Scope::Daily,
    Scope::Hourly,
];

/// Branch-based projection of one horoscope palace.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HoroscopePalaceProjection {
    scope: Scope,
    requested_palace_name: PalaceName,
    branch: EarthlyBranch,
    natal_palace_name: PalaceName,
    temporal_palace_name: Option<PalaceName>,
    natal_palace_stem: HeavenlyStem,
    natal_typed_stars: Vec<StarName>,
    natal_decorative_stars: Vec<StarName>,
    temporal_stars: Vec<StarName>,
    temporal_decorative_stars: Vec<StarName>,
    temporal_mutagen_activations: Vec<HoroscopeProjectionMutagenActivation>,
}

impl HoroscopePalaceProjection {
    /// Returns the scope used to select the projection branch.
    pub const fn scope(&self) -> Scope {
        self.scope
    }

    /// Returns the palace name requested by the caller.
    pub const fn requested_palace_name(&self) -> PalaceName {
        self.requested_palace_name
    }

    /// Returns the stable branch identity of this projection.
    pub const fn branch(&self) -> EarthlyBranch {
        self.branch
    }

    /// Returns the natal palace name at this branch.
    pub const fn natal_palace_name(&self) -> PalaceName {
        self.natal_palace_name
    }

    /// Returns the temporal palace name assigned to this branch, if any.
    pub const fn temporal_palace_name(&self) -> Option<PalaceName> {
        self.temporal_palace_name
    }

    /// Returns the natal palace stem at this branch.
    pub const fn natal_palace_stem(&self) -> HeavenlyStem {
        self.natal_palace_stem
    }

    /// Returns natal typed star names at this branch.
    pub fn natal_typed_stars(&self) -> &[StarName] {
        &self.natal_typed_stars
    }

    /// Returns natal decorative star names at this branch.
    pub fn natal_decorative_stars(&self) -> &[StarName] {
        &self.natal_decorative_stars
    }

    /// Returns temporal scoped star names at this branch for the projection scope.
    pub fn temporal_stars(&self) -> &[StarName] {
        &self.temporal_stars
    }

    /// Returns temporal decorative star names at this branch for the projection scope.
    pub fn temporal_decorative_stars(&self) -> &[StarName] {
        &self.temporal_decorative_stars
    }

    /// Returns temporal mutagen activations at this branch for the projection scope.
    pub fn temporal_mutagen_activations(&self) -> &[HoroscopeProjectionMutagenActivation] {
        &self.temporal_mutagen_activations
    }
}

/// Mutagen activation visible in a palace projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct HoroscopeProjectionMutagenActivation {
    target_star: StarName,
    mutagen: Mutagen,
}

impl HoroscopeProjectionMutagenActivation {
    /// Creates a projected mutagen activation DTO.
    pub const fn new(target_star: StarName, mutagen: Mutagen) -> Self {
        Self {
            target_star,
            mutagen,
        }
    }

    /// Returns the activated natal star.
    pub const fn target_star(&self) -> StarName {
        self.target_star
    }

    /// Returns the transform applied to the star.
    pub const fn mutagen(&self) -> Mutagen {
        self.mutagen
    }
}

/// 三方四正 projection for a target palace.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HoroscopeSurroundPalaces {
    target: HoroscopePalaceProjection,
    opposite: HoroscopePalaceProjection,
    wealth: HoroscopePalaceProjection,
    career: HoroscopePalaceProjection,
}

impl HoroscopeSurroundPalaces {
    /// Returns the target palace projection.
    pub const fn target(&self) -> &HoroscopePalaceProjection {
        &self.target
    }

    /// Returns the opposite palace projection.
    pub const fn opposite(&self) -> &HoroscopePalaceProjection {
        &self.opposite
    }

    /// Returns the 财帛位 projection in the upstream 三方四正 convention.
    pub const fn wealth(&self) -> &HoroscopePalaceProjection {
        &self.wealth
    }

    /// Returns the 官禄位 projection in the upstream 三方四正 convention.
    pub const fn career(&self) -> &HoroscopePalaceProjection {
        &self.career
    }
}

/// Typed facade for upstream-compatible horoscope runtime helper behavior.
pub struct HoroscopeRuntime<'a> {
    chart: &'a HoroscopeChart,
}

impl<'a> HoroscopeRuntime<'a> {
    /// Creates a runtime helper facade after validating required temporal layers.
    pub fn new(chart: &'a HoroscopeChart) -> Result<Self, ChartError> {
        for scope in REQUIRED_RUNTIME_SCOPES {
            let layer = required_layer(chart, scope)?;
            if layer.palace_layout().is_none() {
                return Err(ChartError::MissingHoroscopePalaceLayout { scope });
            }
        }

        Ok(Self { chart })
    }

    /// Projects the nominal-age Life Palace (`agePalace` upstream helper).
    pub fn age_palace(&self) -> Result<HoroscopePalaceProjection, ChartError> {
        self.project_by_palace_name(Scope::Age, PalaceName::Life)
    }

    /// Projects one palace in the selected natal or temporal scope.
    pub fn palace(
        &self,
        scope: Scope,
        palace_name: PalaceName,
    ) -> Result<HoroscopePalaceProjection, ChartError> {
        self.project_by_palace_name(scope, palace_name)
    }

    /// Projects target, opposite, wealth, and career palaces for 三方四正.
    pub fn surround_palaces(
        &self,
        scope: Scope,
        palace_name: PalaceName,
    ) -> Result<HoroscopeSurroundPalaces, ChartError> {
        let branch = self.branch_for_palace(scope, palace_name)?;
        Ok(HoroscopeSurroundPalaces {
            target: self.project_by_branch(scope, palace_name, branch)?,
            opposite: self.project_by_branch(scope, palace_name, branch.offset(6))?,
            wealth: self.project_by_branch(scope, palace_name, branch.offset(8))?,
            career: self.project_by_branch(scope, palace_name, branch.offset(4))?,
        })
    }

    /// Returns true when all queried horoscope stars are present.
    pub fn has_horoscope_stars(
        &self,
        scope: Scope,
        palace: PalaceName,
        stars: &[StarName],
    ) -> Result<bool, ChartError> {
        let present = self.decadal_yearly_stars_for_projected_palace(scope, palace)?;
        Ok(stars.iter().all(|star| present.contains(star)))
    }

    /// Returns true when all queried horoscope stars are absent.
    pub fn not_have_horoscope_stars(
        &self,
        scope: Scope,
        palace: PalaceName,
        stars: &[StarName],
    ) -> Result<bool, ChartError> {
        let present = self.decadal_yearly_stars_for_projected_palace(scope, palace)?;
        Ok(stars.iter().all(|star| !present.contains(star)))
    }

    /// Returns true when at least one queried horoscope star is present.
    pub fn has_one_of_horoscope_stars(
        &self,
        scope: Scope,
        palace: PalaceName,
        stars: &[StarName],
    ) -> Result<bool, ChartError> {
        let present = self.decadal_yearly_stars_for_projected_palace(scope, palace)?;
        Ok(stars.iter().any(|star| present.contains(star)))
    }

    /// Returns true when a scope's mutagen target lands in the projected palace.
    pub fn has_horoscope_mutagen(
        &self,
        scope: Scope,
        palace: PalaceName,
        mutagen: Mutagen,
    ) -> Result<bool, ChartError> {
        if scope == Scope::Natal {
            return Ok(false);
        }
        let branch = self.branch_for_palace(scope, palace)?;
        let layer = self.layer(scope)?;

        Ok(layer.activations().iter().any(|activation| {
            activation.mutagen() == mutagen && activation.target_branch() == branch
        }))
    }

    fn decadal_yearly_stars_for_projected_palace(
        &self,
        scope: Scope,
        palace: PalaceName,
    ) -> Result<Vec<StarName>, ChartError> {
        let branch = self.branch_for_palace(scope, palace)?;
        let mut out = self.temporal_stars_for_branch(Scope::Decadal, branch)?;
        out.extend(self.temporal_stars_for_branch(Scope::Yearly, branch)?);
        Ok(out)
    }

    fn project_by_palace_name(
        &self,
        scope: Scope,
        palace_name: PalaceName,
    ) -> Result<HoroscopePalaceProjection, ChartError> {
        let branch = self.branch_for_palace(scope, palace_name)?;
        self.project_by_branch(scope, palace_name, branch)
    }

    fn project_by_branch(
        &self,
        scope: Scope,
        requested_palace_name: PalaceName,
        branch: EarthlyBranch,
    ) -> Result<HoroscopePalaceProjection, ChartError> {
        let palace = self.natal_palace_by_branch(branch)?;
        let layer = if scope == Scope::Natal {
            None
        } else {
            Some(self.layer(scope)?)
        };

        Ok(HoroscopePalaceProjection {
            scope,
            requested_palace_name,
            branch,
            natal_palace_name: palace.name(),
            temporal_palace_name: layer
                .and_then(TemporalLayer::palace_layout)
                .and_then(|layout| layout.name_for_branch(branch)),
            natal_palace_stem: palace.stem(),
            natal_typed_stars: palace.stars().iter().map(|star| star.name()).collect(),
            natal_decorative_stars: palace
                .decorative_stars()
                .iter()
                .map(|star| star.name())
                .collect(),
            temporal_stars: layer
                .map(|temporal| {
                    temporal
                        .placements()
                        .iter()
                        .filter(|placement| placement.branch() == branch)
                        .map(|placement| placement.placement().name())
                        .collect()
                })
                .unwrap_or_default(),
            temporal_decorative_stars: layer
                .map(|temporal| {
                    temporal
                        .temporal_decorative_stars()
                        .iter()
                        .filter(|placement| placement.branch() == branch)
                        .map(|placement| placement.name())
                        .collect()
                })
                .unwrap_or_default(),
            temporal_mutagen_activations: layer
                .map(|temporal| {
                    temporal
                        .activations()
                        .iter()
                        .filter(|activation| activation.target_branch() == branch)
                        .map(|activation| {
                            HoroscopeProjectionMutagenActivation::new(
                                activation.target_star(),
                                activation.mutagen(),
                            )
                        })
                        .collect()
                })
                .unwrap_or_default(),
        })
    }

    fn branch_for_palace(
        &self,
        scope: Scope,
        palace_name: PalaceName,
    ) -> Result<EarthlyBranch, ChartError> {
        if scope == Scope::Natal {
            return self
                .chart
                .natal()
                .palaces()
                .iter()
                .find(|palace| palace.name() == palace_name)
                .map(Palace::branch)
                .ok_or(ChartError::MissingHoroscopePalaceName { scope, palace_name });
        }

        self.layer(scope)?
            .palace_layout()
            .ok_or(ChartError::MissingHoroscopePalaceLayout { scope })?
            .names()
            .iter()
            .find(|name| name.palace_name() == palace_name)
            .map(|name| name.branch())
            .ok_or(ChartError::MissingHoroscopePalaceName { scope, palace_name })
    }

    fn natal_palace_by_branch(&self, branch: EarthlyBranch) -> Result<&Palace, ChartError> {
        self.chart
            .natal()
            .palaces()
            .iter()
            .find(|palace| palace.branch() == branch)
            .ok_or(ChartError::MissingNatalPalaceForBranch { branch })
    }

    fn temporal_stars_for_branch(
        &self,
        scope: Scope,
        branch: EarthlyBranch,
    ) -> Result<Vec<StarName>, ChartError> {
        Ok(self
            .layer(scope)?
            .placements()
            .iter()
            .filter(|placement| placement.branch() == branch)
            .map(|placement| placement.placement().name())
            .collect())
    }

    fn layer(&self, scope: Scope) -> Result<&TemporalLayer, ChartError> {
        required_layer(self.chart, scope)
    }
}

fn required_layer(chart: &HoroscopeChart, scope: Scope) -> Result<&TemporalLayer, ChartError> {
    let mut layers = chart.layers_in_scope(scope);
    let layer = layers
        .next()
        .ok_or(ChartError::MissingHoroscopeLayer { scope })?;
    if layers.next().is_some() {
        return Err(ChartError::DuplicateHoroscopeLayer { scope });
    }
    Ok(layer)
}
