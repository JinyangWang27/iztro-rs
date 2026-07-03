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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        BirthContext, Brightness, CalendarDate, Chart, Gender, HeavenlyStem, HoroscopeChart,
        MethodProfile, PALACE_NAMES, Palace, ScopedStarPlacement, StarKind, StarPlacement,
        StemBranch, TemporalContext, TemporalLayer, TemporalPalaceLayout, TemporalPalaceName,
    };
    use crate::rules::pattern::context::PatternContext;

    fn palaces(life_branch: EarthlyBranch, stars: &[(EarthlyBranch, StarName)]) -> Vec<Palace> {
        (0..12)
            .map(|index| {
                let branch = life_branch.offset(index as isize);
                let placements = stars
                    .iter()
                    .filter(|(spec, _)| *spec == branch)
                    .map(|(_, star)| {
                        StarPlacement::new(
                            *star,
                            StarKind::Soft,
                            Brightness::Unknown,
                            None,
                            Scope::Natal,
                        )
                    })
                    .collect();
                Palace::new(PALACE_NAMES[index], branch, HeavenlyStem::Jia, placements)
            })
            .collect()
    }

    fn natal_chart(life_branch: EarthlyBranch, stars: &[(EarthlyBranch, StarName)]) -> Chart {
        Chart::try_new(
            BirthContext::new(
                CalendarDate::solar(1990, 5, 17),
                EarthlyBranch::Chen,
                Gender::Female,
            ),
            StemBranch::try_new(HeavenlyStem::Geng, EarthlyBranch::Wu).expect("stem-branch"),
            MethodProfile::placeholder("support_test"),
            palaces(life_branch, stars),
            None,
            None,
        )
        .expect("synthetic chart")
    }

    fn yearly_layout(life_branch: EarthlyBranch) -> TemporalPalaceLayout {
        let names = PALACE_NAMES
            .iter()
            .enumerate()
            .map(|(index, name)| TemporalPalaceName::new(life_branch.offset(index as isize), *name))
            .collect();
        TemporalPalaceLayout::try_new(Scope::Yearly, names).expect("layout")
    }

    fn yearly_horoscope(
        natal: Chart,
        life_branch: EarthlyBranch,
        stars: &[(EarthlyBranch, StarName)],
    ) -> HoroscopeChart {
        let placements = stars
            .iter()
            .map(|(branch, star)| {
                ScopedStarPlacement::new(
                    *branch,
                    StarPlacement::new(
                        *star,
                        StarKind::Soft,
                        Brightness::Unknown,
                        None,
                        Scope::Yearly,
                    ),
                )
            })
            .collect();
        let layer = TemporalLayer::try_new_with_palace_layout(
            Scope::Yearly,
            TemporalContext::Yearly {
                stem_branch: StemBranch::try_new(HeavenlyStem::Jia, EarthlyBranch::Zi)
                    .expect("stem-branch"),
                lunar_year: 2026,
            },
            placements,
            Vec::new(),
            Some(yearly_layout(life_branch)),
        )
        .expect("yearly layer");
        HoroscopeChart::with_layers(natal, vec![layer])
    }

    /// Under the exact-identity invariant, a yearly 流曲 visible in the selected
    /// Life 三方四正 must not be discovered as 文曲 support, while an exact natal
    /// 文昌 still counts.
    #[test]
    fn support_predicate_does_not_count_liu_qu_as_wen_qu() {
        // Life = Zi; SFSZ(Zi) = {Zi, Wu, Chen, Shen}. Natal 文昌@Wu is exact
        // support; yearly 流曲@Chen is a distinct identity and must not fill the
        // 文曲 support slot.
        let natal = natal_chart(EarthlyBranch::Zi, &[(EarthlyBranch::Wu, StarName::WenChang)]);
        let horoscope =
            yearly_horoscope(natal, EarthlyBranch::Zi, &[(EarthlyBranch::Chen, StarName::LiuQu)]);
        let ctx = PatternContext::horoscope_with_frame(
            &horoscope,
            Scope::Yearly,
            vec![Scope::Natal, Scope::Yearly],
        );

        let support = selected_support_in_san_fang_si_zheng(&ctx, EarthlyBranch::Zi);
        let stars: Vec<StarName> = support.stars.iter().map(|(star, _)| *star).collect();

        assert!(
            stars.contains(&StarName::WenChang),
            "exact natal 文昌 should still count as support"
        );
        assert!(
            !stars.contains(&StarName::WenQu),
            "流曲 must not be recorded as 文曲 support"
        );
        assert!(
            !stars.contains(&StarName::LiuQu),
            "流曲 is not in the recognized support set and must not appear"
        );
    }
}
