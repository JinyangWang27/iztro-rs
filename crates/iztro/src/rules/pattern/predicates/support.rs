//! Shared support (辅佐) discovery within a 三方四正.
//!
//! These helpers collect the explicit auxiliary support set (禄存／左右／曲昌／魁钺
//! plus 化禄/权/科) reachable from an anchor palace's 三方四正. They only *discover*
//! that support exists; whether the support satisfies a pattern's `更于吉星` /
//! `加会` clause is decided by the named detector.

use crate::core::{EarthlyBranch, Mutagen, Scope, StarName};
use crate::rules::pattern::context::PatternContext;
use crate::rules::pattern::model::PatternEvidence;
use crate::rules::pattern::predicates::sanfang::{is_in_san_fang_si_zheng, san_fang_si_zheng};
use crate::rules::pattern::predicates::sort_dedup_branches;
use crate::rules::pattern::query::{
    mutagen_activations_for_scope, selected_star_in_palace, selected_stars_in_palace,
    stars_in_palace_for_scope,
};

/// The explicit auxiliary/assistant support stars recognized by support-driven
/// patterns. Arbitrary soft stars are **not** accepted.
pub(crate) const SUPPORT_STARS: [StarName; 7] = [
    StarName::ZuoFu,
    StarName::YouBi,
    StarName::WenChang,
    StarName::WenQu,
    StarName::TianKui,
    StarName::TianYue,
    StarName::LuCun,
];

/// The support (辅佐) stars and 禄/权/科 mutagens discovered within a 三方四正.
#[derive(Clone, Debug, Default)]
pub(crate) struct PatternSupportMatch {
    pub(crate) stars: Vec<(StarName, EarthlyBranch)>,
    pub(crate) mutagens: Vec<(StarName, Mutagen, Scope, EarthlyBranch)>,
    pub(crate) branches: Vec<EarthlyBranch>,
}

impl PatternSupportMatch {
    pub(crate) fn is_empty(&self) -> bool {
        self.stars.is_empty() && self.mutagens.is_empty()
    }

    pub(crate) fn involved_stars(&self) -> Vec<StarName> {
        let mut stars: Vec<StarName> = self
            .stars
            .iter()
            .map(|(star, _)| *star)
            .chain(self.mutagens.iter().map(|(star, _, _, _)| *star))
            .collect();
        stars.sort();
        stars.dedup();
        stars
    }

    pub(crate) fn involved_mutagens(&self) -> Vec<Mutagen> {
        let mut mutagens: Vec<Mutagen> = self
            .mutagens
            .iter()
            .map(|(_, mutagen, _, _)| *mutagen)
            .collect();
        mutagens.sort();
        mutagens.dedup();
        mutagens
    }

    pub(crate) fn evidence(&self) -> Vec<PatternEvidence> {
        let mut evidence: Vec<PatternEvidence> = self
            .stars
            .iter()
            .map(|(star, branch)| PatternEvidence::StarInPalace {
                star: *star,
                branch: *branch,
            })
            .collect();
        evidence.extend(self.mutagens.iter().map(|(star, mutagen, scope, branch)| {
            PatternEvidence::MutagenOnStar {
                star: *star,
                mutagen: *mutagen,
                scope: *scope,
                branch: *branch,
            }
        }));
        evidence
    }
}

/// Collects the explicit support set (禄存／左右／曲昌／魁钺 plus 化禄/权/科) within
/// the 三方四正 of `anchor`, reading scope-effective placements.
///
/// Support is restricted to the named [`SUPPORT_STARS`] and the
/// 禄/权/科 mutagens. Arbitrary [`crate::core::StarKind::Soft`] stars are **not**
/// accepted: the detector requires the specific auxiliary set the maintained
/// conditions name.
pub(crate) fn support_in_san_fang_si_zheng_for_scope(
    ctx: &PatternContext<'_>,
    scope: Scope,
    anchor: EarthlyBranch,
) -> PatternSupportMatch {
    let mut support = PatternSupportMatch::default();

    for branch in san_fang_si_zheng(anchor) {
        for placement in stars_in_palace_for_scope(ctx, scope, branch) {
            let star = placement.placement().name();
            if SUPPORT_STARS.contains(&star) {
                support.stars.push((star, branch));
                support.branches.push(branch);
            }
            if matches!(
                placement.placement().mutagen(),
                Some(Mutagen::Lu | Mutagen::Quan | Mutagen::Ke)
            ) {
                let mutagen = placement.placement().mutagen().expect("checked mutagen");
                support
                    .mutagens
                    .push((star, mutagen, placement.placement().scope(), branch));
                support.branches.push(branch);
            }
        }
    }

    if scope != Scope::Natal {
        for activation in mutagen_activations_for_scope(ctx, scope) {
            if matches!(
                activation.mutagen(),
                Mutagen::Lu | Mutagen::Quan | Mutagen::Ke
            ) && is_in_san_fang_si_zheng(anchor, activation.target_branch())
            {
                support.mutagens.push((
                    activation.target_star(),
                    activation.mutagen(),
                    activation.source_scope(),
                    activation.target_branch(),
                ));
                support.branches.push(activation.target_branch());
            }
        }
    }

    sort_dedup_branches(&mut support.branches);
    support
}

/// Collects the explicit support set within the 三方四正 of `anchor`, reading the
/// selected (effective) view rather than a single named scope.
pub(crate) fn selected_support_in_san_fang_si_zheng(
    ctx: &PatternContext<'_>,
    anchor: EarthlyBranch,
) -> PatternSupportMatch {
    let mut support = PatternSupportMatch::default();
    let branches = san_fang_si_zheng(anchor);

    for branch in branches {
        for requested_star in SUPPORT_STARS {
            if let Some(placement) = selected_star_in_palace(ctx, branch, requested_star) {
                support.stars.push((placement.placement().name(), branch));
                support.branches.push(branch);
            }
        }

        for placement in selected_stars_in_palace(ctx, branch) {
            let star = placement.placement().name();
            if placement.source_scope() == Scope::Natal
                && matches!(
                    placement.placement().mutagen(),
                    Some(Mutagen::Lu | Mutagen::Quan | Mutagen::Ke)
                )
            {
                let mutagen = placement.placement().mutagen().expect("checked mutagen");
                support
                    .mutagens
                    .push((star, mutagen, placement.source_scope(), branch));
                support.branches.push(branch);
            }
        }
    }

    if let Some(state) = ctx.effective() {
        for activation in state.mutagen_activations() {
            if matches!(
                activation.activation().mutagen(),
                Mutagen::Lu | Mutagen::Quan | Mutagen::Ke
            ) && is_in_san_fang_si_zheng(anchor, activation.activation().target_branch())
            {
                support.mutagens.push((
                    activation.activation().target_star(),
                    activation.activation().mutagen(),
                    activation.source_scope(),
                    activation.activation().target_branch(),
                ));
                support
                    .branches
                    .push(activation.activation().target_branch());
            }
        }
    }

    support
        .stars
        .sort_by_key(|(star, branch)| (*star, branch.index()));
    support.stars.dedup();
    support
        .mutagens
        .sort_by_key(|(star, mutagen, scope, branch)| {
            (*star, *mutagen, scope_sort_key(*scope), branch.index())
        });
    support.mutagens.dedup();
    sort_dedup_branches(&mut support.branches);
    support
}

/// Stable ordering rank for a scope, used to sort discovered support mutagens.
pub(crate) const fn scope_sort_key(scope: Scope) -> u8 {
    match scope {
        Scope::Natal => 0,
        Scope::Decadal => 1,
        Scope::Age => 2,
        Scope::Yearly => 3,
        Scope::Monthly => 4,
        Scope::Daily => 5,
        Scope::Hourly => 6,
    }
}
