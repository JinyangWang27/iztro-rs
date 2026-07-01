//! Effective selected-view chart state.
//!
//! This is a read-only model over one active chart slice: branches remain the
//! stable palace coordinates, palace names come from one selected frame, and
//! fact provenance is retained per star or mutagen activation.

use crate::core::error::ChartError;
use crate::core::model::chart::{
    Chart, HoroscopeChart, MutagenActivation, PalaceName, StarPlacement, TemporalLayer,
    TemporalPalaceLayout,
};
use crate::core::model::star::mutagen::Scope;
use lunar_lite::EarthlyBranch;

const SCOPE_ORDER: [Scope; 7] = [
    Scope::Natal,
    Scope::Decadal,
    Scope::Age,
    Scope::Yearly,
    Scope::Monthly,
    Scope::Daily,
    Scope::Hourly,
];

/// One effective typed star fact with retained source provenance.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectiveStarRef<'a> {
    source_scope: Scope,
    branch: EarthlyBranch,
    placement: &'a StarPlacement,
}

impl<'a> EffectiveStarRef<'a> {
    /// Creates a sourced star reference.
    pub const fn new(
        source_scope: Scope,
        branch: EarthlyBranch,
        placement: &'a StarPlacement,
    ) -> Self {
        Self {
            source_scope,
            branch,
            placement,
        }
    }

    /// Scope that supplied this star fact.
    pub const fn source_scope(self) -> Scope {
        self.source_scope
    }

    /// Stable branch coordinate occupied by this star.
    pub const fn branch(self) -> EarthlyBranch {
        self.branch
    }

    /// Borrowed typed star placement.
    pub const fn placement(self) -> &'a StarPlacement {
        self.placement
    }
}

/// One effective mutagen activation with retained source provenance.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectiveMutagenRef<'a> {
    source_scope: Scope,
    activation: &'a MutagenActivation,
}

impl<'a> EffectiveMutagenRef<'a> {
    /// Creates a sourced mutagen activation reference.
    pub const fn new(source_scope: Scope, activation: &'a MutagenActivation) -> Self {
        Self {
            source_scope,
            activation,
        }
    }

    /// Scope that supplied this mutagen activation.
    pub const fn source_scope(self) -> Scope {
        self.source_scope
    }

    /// Borrowed activation fact.
    pub const fn activation(self) -> &'a MutagenActivation {
        self.activation
    }
}

/// Read-only selected chart state for pattern/rule analysis.
#[derive(Clone, Debug, PartialEq)]
pub struct EffectiveChartState<'a> {
    chart: &'a Chart,
    active_scopes: Vec<Scope>,
    layers: Vec<&'a TemporalLayer>,
    palace_frame_scope: Scope,
    palace_frame_layout: Option<&'a TemporalPalaceLayout>,
}

impl<'a> EffectiveChartState<'a> {
    /// Builds a natal-only effective state.
    pub fn from_chart(
        chart: &'a Chart,
        palace_frame_scope: Scope,
        active_scopes: Vec<Scope>,
    ) -> Result<Self, ChartError> {
        Self::from_parts(chart, &[], palace_frame_scope, active_scopes)
    }

    /// Builds an effective state over a horoscope chart.
    pub fn from_horoscope(
        horoscope: &'a HoroscopeChart,
        palace_frame_scope: Scope,
        active_scopes: Vec<Scope>,
    ) -> Result<Self, ChartError> {
        Self::from_parts(
            horoscope.natal(),
            horoscope.layers(),
            palace_frame_scope,
            active_scopes,
        )
    }

    fn from_parts(
        chart: &'a Chart,
        available_layers: &'a [TemporalLayer],
        palace_frame_scope: Scope,
        active_scopes: Vec<Scope>,
    ) -> Result<Self, ChartError> {
        validate_active_scopes(&active_scopes)?;
        if !active_scopes.contains(&palace_frame_scope) {
            return Err(ChartError::ActiveFrameScopeNotVisible {
                scope: palace_frame_scope,
            });
        }

        let mut layers = Vec::new();
        for scope in canonical_scopes(&active_scopes) {
            if scope == Scope::Natal {
                continue;
            }
            let mut matching = available_layers
                .iter()
                .filter(|layer| layer.scope() == scope);
            let layer = matching
                .next()
                .ok_or(ChartError::MissingHoroscopeLayer { scope })?;
            if matching.next().is_some() {
                return Err(ChartError::DuplicateHoroscopeLayer { scope });
            }
            layers.push(layer);
        }

        let palace_frame_layout = if palace_frame_scope == Scope::Natal {
            None
        } else {
            let layer = layers
                .iter()
                .find(|layer| layer.scope() == palace_frame_scope)
                .expect("frame scope was validated as active");
            Some(
                layer
                    .palace_layout()
                    .ok_or(ChartError::MissingHoroscopePalaceLayout {
                        scope: palace_frame_scope,
                    })?,
            )
        };

        Ok(Self {
            chart,
            active_scopes: canonical_scopes(&active_scopes),
            layers,
            palace_frame_scope,
            palace_frame_layout,
        })
    }

    /// Active scopes in canonical natal-outward order.
    pub fn active_scopes(&self) -> &[Scope] {
        &self.active_scopes
    }

    /// Scope supplying the selected palace-name frame.
    pub const fn palace_frame_scope(&self) -> Scope {
        self.palace_frame_scope
    }

    /// Returns the branch carrying `palace` in the selected palace frame.
    pub fn branch_of_palace(&self, palace: PalaceName) -> Option<EarthlyBranch> {
        match self.palace_frame_layout {
            None => self.chart.branch_of_palace(palace),
            Some(layout) => layout
                .names()
                .iter()
                .find(|name| name.palace_name() == palace)
                .map(|name| name.branch()),
        }
    }

    /// Effective typed stars at `branch`, including natal facts and active
    /// temporal overlays.
    pub fn stars_in_palace(&self, branch: EarthlyBranch) -> Vec<EffectiveStarRef<'a>> {
        let mut stars = self.stars_in_palace_for_source(Scope::Natal, branch);
        for layer in &self.layers {
            stars.extend(self.stars_in_palace_for_source(layer.scope(), branch));
        }
        stars
    }

    /// Typed stars at `branch` from exactly one source scope.
    pub fn stars_in_palace_for_source(
        &self,
        source_scope: Scope,
        branch: EarthlyBranch,
    ) -> Vec<EffectiveStarRef<'a>> {
        if !self.active_scopes.contains(&source_scope) {
            return Vec::new();
        }

        if source_scope == Scope::Natal {
            return self
                .chart
                .palaces()
                .iter()
                .filter(|palace| palace.branch() == branch)
                .flat_map(|palace| {
                    palace.stars().iter().map(move |placement| {
                        EffectiveStarRef::new(Scope::Natal, branch, placement)
                    })
                })
                .collect();
        }

        self.layers
            .iter()
            .filter(|layer| layer.scope() == source_scope)
            .flat_map(|layer| layer.placements())
            .filter(move |placement| placement.branch() == branch)
            .map(|placement| {
                EffectiveStarRef::new(source_scope, placement.branch(), placement.placement())
            })
            .collect()
    }

    /// Effective mutagen activations from all active non-natal scopes.
    pub fn mutagen_activations(&self) -> Vec<EffectiveMutagenRef<'a>> {
        self.layers
            .iter()
            .flat_map(|layer| {
                layer
                    .activations()
                    .iter()
                    .map(|activation| EffectiveMutagenRef::new(layer.scope(), activation))
            })
            .collect()
    }

    /// Effective mutagen activations from exactly one source scope.
    pub fn mutagen_activations_for_source(
        &self,
        source_scope: Scope,
    ) -> Vec<EffectiveMutagenRef<'a>> {
        if source_scope == Scope::Natal || !self.active_scopes.contains(&source_scope) {
            return Vec::new();
        }
        self.layers
            .iter()
            .filter(|layer| layer.scope() == source_scope)
            .flat_map(|layer| {
                layer
                    .activations()
                    .iter()
                    .map(|activation| EffectiveMutagenRef::new(source_scope, activation))
            })
            .collect()
    }
}

fn validate_active_scopes(active_scopes: &[Scope]) -> Result<(), ChartError> {
    if !active_scopes.contains(&Scope::Natal) {
        return Err(ChartError::EffectiveChartStateMissingNatalScope);
    }
    for (index, scope) in active_scopes.iter().enumerate() {
        if active_scopes[..index].contains(scope) {
            return Err(ChartError::DuplicateEffectiveChartStateScope { scope: *scope });
        }
    }
    Ok(())
}

fn canonical_scopes(active_scopes: &[Scope]) -> Vec<Scope> {
    SCOPE_ORDER
        .into_iter()
        .filter(|scope| active_scopes.contains(scope))
        .collect()
}
