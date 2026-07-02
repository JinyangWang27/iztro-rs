//! Layer keys and selection-to-layer expansion for cacheable analysis.
//!
//! [`StaticTemporalNavigationSelection`] belongs to core and describes a
//! concrete navigation path. [`AnalysisLayerKey`] belongs to analysis and
//! describes a cacheable analysis identity. For cache stability, each key maps
//! to its own canonical analysis selection before core builds the temporal
//! overlay stack.

use serde::{Deserialize, Serialize};

use crate::core::StaticTemporalNavigationSelection;
use crate::core::{ChartError, Scope};
use crate::rules::classical::ClaimScope;
use crate::rules::pattern::PatternScope;

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

    /// The canonical core navigation selection used to analyze this cache key.
    ///
    /// `AnalysisLayerKey` owns cache identity, so it also owns the policy that
    /// turns that identity into a stable core navigation anchor. Age has no
    /// separate navigation selection; it reuses the corresponding yearly
    /// selection, while [`analysis_scopes_for_layer_key`] truncates active scopes
    /// to `[Natal, Decadal, Age]`.
    pub(crate) fn selection_for_canonical_analysis(
        &self,
    ) -> Result<StaticTemporalNavigationSelection, ChartError> {
        use StaticTemporalNavigationSelection as Sel;

        Ok(match self {
            Self::Natal => Sel::Natal,
            Self::Decadal { decadal_index } => Sel::Decadal {
                decadal_index: *decadal_index,
            },
            Self::Age {
                decadal_index,
                year_index,
            }
            | Self::Yearly {
                decadal_index,
                year_index,
            } => Sel::Yearly {
                decadal_index: *decadal_index,
                year_index: checked_temporal_index("year_index", *year_index, 9)?,
            },
            Self::Monthly {
                decadal_index,
                year_index,
                month_index,
            } => Sel::Monthly {
                decadal_index: *decadal_index,
                year_index: checked_temporal_index("year_index", *year_index, 9)?,
                month_index: checked_temporal_index("month_index", *month_index, 11)?,
            },
            Self::Daily {
                decadal_index,
                year_index,
                month_index,
                day_index,
            } => Sel::Daily {
                decadal_index: *decadal_index,
                year_index: checked_temporal_index("year_index", *year_index, 9)?,
                month_index: checked_temporal_index("month_index", *month_index, 11)?,
                day_index: checked_temporal_index("day_index", *day_index, 29)?,
            },
            Self::Hourly {
                decadal_index,
                year_index,
                month_index,
                day_index,
                hour_index,
            } => Sel::Hourly {
                decadal_index: *decadal_index,
                year_index: checked_temporal_index("year_index", *year_index, 9)?,
                month_index: checked_temporal_index("month_index", *month_index, 11)?,
                day_index: checked_temporal_index("day_index", *day_index, 29)?,
                hour_index: checked_temporal_index("hour_index", *hour_index, 12)?,
            },
        })
    }
}

fn checked_temporal_index(field: &'static str, value: usize, max: u8) -> Result<u8, ChartError> {
    // ChartError stores temporal child indexes as u8 because core selections use u8.
    // Larger usize values fail closed and are reported as u8::MAX.
    let value = u8::try_from(value).map_err(|_| ChartError::InvalidTemporalSelectionIndex {
        field,
        value: u8::MAX,
        max,
    })?;
    if value > max {
        return Err(ChartError::InvalidTemporalSelectionIndex { field, value, max });
    }
    Ok(value)
}

/// The inclusive natal-outward [`Scope`] chain an analysis layer may inspect.
///
/// A layer detector may read its own scope and every **ancestor** scope, but
/// never a **descendant**. The chain is therefore truncated at the layer's own
/// scope:
///
/// | Layer     | Active scopes                                            |
/// |-----------|----------------------------------------------------------|
/// | Natal     | `[Natal]`                                                |
/// | Decadal   | `[Natal, Decadal]`                                       |
/// | Age       | `[Natal, Decadal, Age]`                                  |
/// | Yearly    | `[Natal, Decadal, Age, Yearly]`                          |
/// | Monthly   | `[Natal, Decadal, Age, Yearly, Monthly]`                 |
/// | Daily     | `[Natal, Decadal, Age, Yearly, Monthly, Daily]`          |
/// | Hourly    | `[Natal, Decadal, Age, Yearly, Monthly, Daily, Hourly]`  |
///
/// Cached layer stability has two parts:
///
/// 1. canonical target-coordinate anchoring from [`AnalysisLayerKey`];
/// 2. active-scope truncation from [`AnalysisLayerKey`].
///
/// Canonical anchoring prevents descendant target coordinates from leaking into
/// ancestor overlay construction. Scope truncation prevents descendant overlay
/// scopes and facts from leaking into ancestor detection. Both are necessary:
/// detecting a 流年 layer inside a selected 流月 view must use the 流年 key's own
/// canonical target coordinates and see only `Natal..=Yearly`.
pub fn analysis_scopes_for_layer_key(key: &AnalysisLayerKey) -> Vec<Scope> {
    const ORDER: [Scope; 7] = [
        Scope::Natal,
        Scope::Decadal,
        Scope::Age,
        Scope::Yearly,
        Scope::Monthly,
        Scope::Daily,
        Scope::Hourly,
    ];
    let target = key.scope();
    let mut scopes = Vec::new();
    for scope in ORDER {
        scopes.push(scope);
        if scope == target {
            break;
        }
    }
    scopes
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn age_key_uses_yearly_selection_as_canonical_anchor() {
        let key = AnalysisLayerKey::Age {
            decadal_index: 1,
            year_index: 2,
        };

        let selection = key
            .selection_for_canonical_analysis()
            .expect("age key should map to yearly selection");

        assert_eq!(
            selection,
            StaticTemporalNavigationSelection::Yearly {
                decadal_index: 1,
                year_index: 2,
            }
        );
    }

    #[test]
    fn canonical_anchor_rejects_out_of_range_child_indexes() {
        let key = AnalysisLayerKey::Hourly {
            decadal_index: 0,
            year_index: 0,
            month_index: 0,
            day_index: 0,
            hour_index: 13,
        };

        let err = key
            .selection_for_canonical_analysis()
            .expect_err("hour index above 12 should be rejected");

        assert!(matches!(
            err,
            ChartError::InvalidTemporalSelectionIndex {
                field: "hour_index",
                value: 13,
                max: 12,
            }
        ));
    }
}
