//! Layer keys and selection-to-layer expansion for cacheable analysis.

use serde::{Deserialize, Serialize};

use crate::core::Scope;
use crate::core::pattern::PatternScope;
use crate::core::view::StaticTemporalNavigationSelection;
use crate::rules::classical::ClaimScope;

/// A key identifying one cacheable analysis layer.
///
/// Each variant pins exactly the temporal indexes needed to address a layer in
/// the existing navigation model. The indexes mirror
/// [`StaticTemporalNavigationSelection`]: a decadal index, then a year, month,
/// day, and hour index as the layer deepens.
///
/// Layers are the unit of caching: an app requests only the layers it is missing
/// from its cache (see [`analysis_layers_for_selection`]) and groups cached
/// results by [`AnalysisLayerKey::scope`].
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AnalysisLayerKey {
    /// 本命 (natal).
    Natal,
    /// 大限 (decadal).
    Decadal {
        /// Selected decadal period index.
        decadal_index: usize,
    },
    /// 小限 (nominal-age).
    Age {
        /// Selected decadal period index.
        decadal_index: usize,
        /// Selected year index within the period.
        year_index: usize,
    },
    /// 流年 (yearly).
    Yearly {
        /// Selected decadal period index.
        decadal_index: usize,
        /// Selected year index within the period.
        year_index: usize,
    },
    /// 流月 (monthly).
    Monthly {
        /// Selected decadal period index.
        decadal_index: usize,
        /// Selected year index within the period.
        year_index: usize,
        /// Selected lunar-month index.
        month_index: usize,
    },
    /// 流日 (daily).
    Daily {
        /// Selected decadal period index.
        decadal_index: usize,
        /// Selected year index within the period.
        year_index: usize,
        /// Selected lunar-month index.
        month_index: usize,
        /// Selected lunar-day index.
        day_index: usize,
    },
    /// 流时 (hourly).
    Hourly {
        /// Selected decadal period index.
        decadal_index: usize,
        /// Selected year index within the period.
        year_index: usize,
        /// Selected lunar-month index.
        month_index: usize,
        /// Selected lunar-day index.
        day_index: usize,
        /// Selected double-hour index.
        hour_index: usize,
    },
}

impl AnalysisLayerKey {
    /// The temporal [`Scope`] this layer is asserted within.
    pub fn scope(&self) -> Scope {
        match self {
            Self::Natal => Scope::Natal,
            Self::Decadal { .. } => Scope::Decadal,
            Self::Age { .. } => Scope::Age,
            Self::Yearly { .. } => Scope::Yearly,
            Self::Monthly { .. } => Scope::Monthly,
            Self::Daily { .. } => Scope::Daily,
            Self::Hourly { .. } => Scope::Hourly,
        }
    }

    /// The [`ClaimScope`] for classical rule hits belonging to this layer.
    pub fn claim_scope(&self) -> ClaimScope {
        ClaimScope::from(self.scope())
    }

    /// The [`PatternScope`] for pattern detections belonging to this layer.
    pub fn pattern_scope(&self) -> PatternScope {
        match self {
            Self::Natal => PatternScope::Natal,
            Self::Decadal { .. } => PatternScope::Decadal,
            Self::Age { .. } => PatternScope::Age,
            Self::Yearly { .. } => PatternScope::Yearly,
            Self::Monthly { .. } => PatternScope::Monthly,
            Self::Daily { .. } => PatternScope::Daily,
            Self::Hourly { .. } => PatternScope::Hourly,
        }
    }
}

/// Expands a temporal navigation selection into the analysis layers it requires.
///
/// The returned vector is the full ancestor chain the selection makes visible,
/// ordered from natal outward. The app requests only the layers it is missing
/// from its cache and composes visible groups itself.
///
/// When a year is selected, both [`AnalysisLayerKey::Age`] (小限) and
/// [`AnalysisLayerKey::Yearly`] (流年) are included: they are distinct scopes that
/// both become relevant once the selected view has a year.
///
/// | Selection            | Required layers                                          |
/// |----------------------|----------------------------------------------------------|
/// | Natal / PreDecadal   | `[Natal]`                                                |
/// | Decadal              | `[Natal, Decadal]`                                       |
/// | Yearly               | `[Natal, Decadal, Age, Yearly]`                          |
/// | Monthly              | `[Natal, Decadal, Age, Yearly, Monthly]`                 |
/// | Daily                | `[Natal, Decadal, Age, Yearly, Monthly, Daily]`          |
/// | Hourly               | `[Natal, Decadal, Age, Yearly, Monthly, Daily, Hourly]`  |
pub fn analysis_layers_for_selection(
    selection: StaticTemporalNavigationSelection,
) -> Vec<AnalysisLayerKey> {
    use StaticTemporalNavigationSelection as Sel;

    match selection {
        Sel::Natal | Sel::PreDecadal => vec![AnalysisLayerKey::Natal],
        Sel::Decadal { decadal_index } => vec![
            AnalysisLayerKey::Natal,
            AnalysisLayerKey::Decadal { decadal_index },
        ],
        Sel::Yearly {
            decadal_index,
            year_index,
        } => {
            let year_index = year_index as usize;
            vec![
                AnalysisLayerKey::Natal,
                AnalysisLayerKey::Decadal { decadal_index },
                AnalysisLayerKey::Age {
                    decadal_index,
                    year_index,
                },
                AnalysisLayerKey::Yearly {
                    decadal_index,
                    year_index,
                },
            ]
        }
        Sel::Monthly {
            decadal_index,
            year_index,
            month_index,
        } => {
            let year_index = year_index as usize;
            let month_index = month_index as usize;
            vec![
                AnalysisLayerKey::Natal,
                AnalysisLayerKey::Decadal { decadal_index },
                AnalysisLayerKey::Age {
                    decadal_index,
                    year_index,
                },
                AnalysisLayerKey::Yearly {
                    decadal_index,
                    year_index,
                },
                AnalysisLayerKey::Monthly {
                    decadal_index,
                    year_index,
                    month_index,
                },
            ]
        }
        Sel::Daily {
            decadal_index,
            year_index,
            month_index,
            day_index,
        } => {
            let year_index = year_index as usize;
            let month_index = month_index as usize;
            let day_index = day_index as usize;
            vec![
                AnalysisLayerKey::Natal,
                AnalysisLayerKey::Decadal { decadal_index },
                AnalysisLayerKey::Age {
                    decadal_index,
                    year_index,
                },
                AnalysisLayerKey::Yearly {
                    decadal_index,
                    year_index,
                },
                AnalysisLayerKey::Monthly {
                    decadal_index,
                    year_index,
                    month_index,
                },
                AnalysisLayerKey::Daily {
                    decadal_index,
                    year_index,
                    month_index,
                    day_index,
                },
            ]
        }
        Sel::Hourly {
            decadal_index,
            year_index,
            month_index,
            day_index,
            hour_index,
        } => {
            let year_index = year_index as usize;
            let month_index = month_index as usize;
            let day_index = day_index as usize;
            let hour_index = hour_index as usize;
            vec![
                AnalysisLayerKey::Natal,
                AnalysisLayerKey::Decadal { decadal_index },
                AnalysisLayerKey::Age {
                    decadal_index,
                    year_index,
                },
                AnalysisLayerKey::Yearly {
                    decadal_index,
                    year_index,
                },
                AnalysisLayerKey::Monthly {
                    decadal_index,
                    year_index,
                    month_index,
                },
                AnalysisLayerKey::Daily {
                    decadal_index,
                    year_index,
                    month_index,
                    day_index,
                },
                AnalysisLayerKey::Hourly {
                    decadal_index,
                    year_index,
                    month_index,
                    day_index,
                    hour_index,
                },
            ]
        }
    }
}
