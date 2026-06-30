//! A hierarchical temporal navigation selection (an index path into the 大限 →
//! 流年 → 流月 → 流日 → 流时 drill-down).
//!
//! This is a domain selection concept, not a presentation read model: it names
//! *which* temporal slice a caller wants by index path, independent of any GUI
//! snapshot. Core, analysis, the projection layer, and the facade all consume it,
//! so it lives in `core` (depended on by everything above it) rather than in the
//! projection layer.

use serde::{Deserialize, Serialize};

/// A renderer-neutral, hierarchical drill-down selection for the bottom panel.
///
/// A caller (TUI/GUI) reports *which* bottom-panel cell the user chose as an
/// **index path** (大限 → 流年 → 流月 → 流日 → 流时); the facade resolves the indices
/// to concrete lunar/solar coordinates and prepares the matching projection. Each
/// deeper variant carries its ancestors' indices. The caller never derives the
/// overlay, the lunar labels, or the month/day validity itself.
///
/// Indices: `year_index` 0..=9 (within the 大限's 10 years); `month_index` 0..=11
/// (lunar month 正月..腊月); `day_index` 0..=29 (lunar day 初一..三十); `hour_index`
/// is upstream iztro `timeIndex` 0..=12 (early 子..亥, late 子).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum StaticTemporalNavigationSelection {
    /// 本命 — the natal slice with no temporal overlay.
    #[default]
    Natal,
    /// 限前 — the span before the first 大限. Carries no overlay; natal base.
    PreDecadal,
    /// 大限 — the selected decadal period; enables the 流年 row.
    Decadal {
        /// Zero-based index into the decadal frame periods.
        decadal_index: usize,
    },
    /// 流年/小限 — a year within the selected 大限; enables the 流月 row.
    Yearly {
        /// Selected decadal period index.
        decadal_index: usize,
        /// Zero-based year within the period (0..=9).
        year_index: u8,
    },
    /// 流月 — a lunar month of the selected 流年; enables the 流日 row.
    Monthly {
        /// Selected decadal period index.
        decadal_index: usize,
        /// Selected year within the period (0..=9).
        year_index: u8,
        /// Zero-based lunar month (0..=11 -> 正月..腊月).
        month_index: u8,
    },
    /// 流日 — a lunar day of the selected 流月; enables the 流时 row.
    Daily {
        /// Selected decadal period index.
        decadal_index: usize,
        /// Selected year within the period (0..=9).
        year_index: u8,
        /// Selected lunar month (0..=11).
        month_index: u8,
        /// Zero-based lunar day (0..=29 -> 初一..三十).
        day_index: u8,
    },
    /// 流时 — a double-hour of the selected 流日.
    Hourly {
        /// Selected decadal period index.
        decadal_index: usize,
        /// Selected year within the period (0..=9).
        year_index: u8,
        /// Selected lunar month (0..=11).
        month_index: u8,
        /// Selected lunar day (0..=29).
        day_index: u8,
        /// Upstream iztro `timeIndex` (0..=12: early 子..亥, late 子).
        hour_index: u8,
    },
}

impl StaticTemporalNavigationSelection {
    /// The selected decadal period index, if the path reaches 大限.
    pub const fn decadal_index(&self) -> Option<usize> {
        match self {
            Self::Natal | Self::PreDecadal => None,
            Self::Decadal { decadal_index }
            | Self::Yearly { decadal_index, .. }
            | Self::Monthly { decadal_index, .. }
            | Self::Daily { decadal_index, .. }
            | Self::Hourly { decadal_index, .. } => Some(*decadal_index),
        }
    }

    /// The selected year index, if the path reaches 流年.
    pub const fn year_index(&self) -> Option<u8> {
        match self {
            Self::Yearly { year_index, .. }
            | Self::Monthly { year_index, .. }
            | Self::Daily { year_index, .. }
            | Self::Hourly { year_index, .. } => Some(*year_index),
            _ => None,
        }
    }

    /// The selected lunar-month index, if the path reaches 流月.
    pub const fn month_index(&self) -> Option<u8> {
        match self {
            Self::Monthly { month_index, .. }
            | Self::Daily { month_index, .. }
            | Self::Hourly { month_index, .. } => Some(*month_index),
            _ => None,
        }
    }

    /// The selected lunar-day index, if the path reaches 流日.
    pub const fn day_index(&self) -> Option<u8> {
        match self {
            Self::Daily { day_index, .. } | Self::Hourly { day_index, .. } => Some(*day_index),
            _ => None,
        }
    }

    /// The selected upstream iztro `timeIndex`, if the path reaches 流时.
    pub const fn hour_index(&self) -> Option<u8> {
        match self {
            Self::Hourly { hour_index, .. } => Some(*hour_index),
            _ => None,
        }
    }
}
