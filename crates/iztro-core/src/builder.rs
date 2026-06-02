//! Deterministic chart builders for core scaffold data.

use crate::{
    calendar::BirthContext,
    chart::{Chart, Palace},
    error::ChartError,
    ganzhi::{EARTHLY_BRANCHES, HEAVENLY_STEMS},
    palace::PALACE_NAMES,
    profile::MethodProfile,
};

/// Builds a deterministic empty twelve-palace chart from typed chart metadata.
///
/// Palace names and earthly branches follow the canonical core order. Heavenly
/// stems are assigned cyclically as placeholder infrastructure only; this is
/// not the final Zi Wei Dou Shu palace stem-placement algorithm.
pub fn build_empty_chart(
    birth_context: BirthContext,
    method_profile: MethodProfile,
) -> Result<Chart, ChartError> {
    let palaces = PALACE_NAMES
        .iter()
        .copied()
        .enumerate()
        .map(|(index, name)| {
            Palace::new(
                name,
                EARTHLY_BRANCHES[index],
                HEAVENLY_STEMS[index % HEAVENLY_STEMS.len()],
                Vec::new(),
            )
        })
        .collect();

    Chart::try_new(birth_context, method_profile, palaces, None, None)
}
