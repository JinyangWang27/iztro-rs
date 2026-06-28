//! Shared Heavenly Stem mutagen-activation builder for temporal overlays.

use crate::core::model::chart::{Chart, MutagenActivation};
use lunar_lite::HeavenlyStem;
use crate::core::model::star::mutagen::{Scope, birth_year_star_mutagen};

/// Builds the [`MutagenActivation`]s a Heavenly Stem produces over a natal chart.
///
/// Shared by the temporal mutagen overlay builders (yearly, decadal, …): for
/// every represented star placed in `natal`, the shared 天干四化 table
/// ([`birth_year_star_mutagen`]) decides whether `stem` maps that star to a
/// [`Mutagen`](crate::core::model::star::mutagen::Mutagen); each mapped, present star
/// yields one `scope`-tagged activation targeting the branch of the palace it
/// occupies. Stars absent from `natal` (or not in the table) produce no
/// activation, so an unsupported or missing target is skipped rather than
/// invented. The natal chart is only read, never mutated, and the returned
/// [`Vec`] is freshly owned, so callers share no mutable state.
pub(crate) fn stem_mutagen_activations(
    natal: &Chart,
    scope: Scope,
    stem: HeavenlyStem,
) -> Vec<MutagenActivation> {
    natal
        .stars()
        .into_iter()
        .filter_map(|fact| {
            let star = fact.placement().name();
            birth_year_star_mutagen(stem, star)
                .map(|mutagen| MutagenActivation::new(scope, star, fact.palace().branch(), mutagen))
        })
        .collect()
}
