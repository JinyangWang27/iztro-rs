//! Deterministic yearly-scope decorative star placement (`yearlyDecStar`).
//!
//! Upstream `FunctionalHoroscope` exposes a flowing year's 岁前/将前十二神 under
//! `yearlyDecStar`. These reproduce the natal 岁前/将前 rule, but anchored on the
//! flowing-year branch rather than the birth-year branch. They are untyped
//! decorative facts (no [`StarKind`](crate::core::model::star::StarKind)), so they
//! are modelled as [`ScopedDecorativeStarPlacement`]s scoped to [`Scope::Yearly`]
//! and never as typed star placements.

use crate::core::error::ChartError;
use crate::core::model::chart::{
    DecorativeStarPlacement, ScopedDecorativeStarPlacement, YearlyPeriod,
};
use crate::core::model::profile::ChartAlgorithmKind;
use crate::core::model::star::mutagen::Scope;
use crate::core::placement::natal::decorative::suiqian_jiangqian12_placements;

/// Builds the yearly `yearlyDecStar` placements (岁前/将前十二神) for a yearly period.
///
/// Anchors the shared 岁前/将前 rule on the flowing-year branch and tags every
/// entry with [`Scope::Yearly`]. The `algorithm` selects the Zhongzhou 岁破 rename
/// of the seventh 岁前 entry, matching the natal placement's behaviour.
pub fn build_yearly_decorative_star_placements(
    period: &YearlyPeriod,
    algorithm: ChartAlgorithmKind,
) -> Result<Vec<ScopedDecorativeStarPlacement>, ChartError> {
    let year_branch = period.stem_branch().branch();

    suiqian_jiangqian12_placements(year_branch, algorithm)
        .into_iter()
        .map(|(branch, name, family)| {
            let placement = DecorativeStarPlacement::try_new(name, family, Scope::Yearly)?;
            ScopedDecorativeStarPlacement::try_new(branch, placement)
        })
        .collect()
}
