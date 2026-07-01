//! Read-only chart query helpers shared by pattern rules.
//!
//! These helpers borrow chart facts and never mutate them. They centralize
//! common lookups so individual rules stay small and consistent.
//!
//! # Two helper families
//!
//! The helpers split into two intentionally distinct families. Picking the
//! wrong one is the class of mistake that PR #145 fixed, so the distinction is
//! load-bearing:
//!
//! - **Selected-state helpers** (`selected_*`) read the selected
//!   [`EffectiveChartState`]: the selected palace frame plus the active fact
//!   stack (natal facts under the active temporal overlays). These are the
//!   **default** for ordinary formation rules — a natal support star that falls
//!   inside a temporal frame's 三方四正 is visible here.
//! - **Source/layer-specific helpers** (`*_for_scope`, and their `source_*`
//!   aliases) read facts from **one** source scope/layer only. They are for
//!   explicitly source-bound flow-star, mutagen, or cross-scope rules — never a
//!   substitute for a selected-state query. A `Scope::Yearly` query sees only
//!   Yearly-layer facts, not natal support stars projected into the Yearly
//!   frame.
//!
//! The natal-only free functions (e.g. [`stars_in_palace`], [`palace_has_star`])
//! take a bare [`Chart`] and read natal facts directly; they predate the
//! effective-state model and remain for natal rules.

use crate::core::{
    Brightness, Chart, EarthlyBranch, EffectiveChartState, EffectiveStarRef, FlowStarBase,
    FlowStarScope, Mutagen, MutagenActivation, PalaceName, Scope, StarKind, StarName,
    StarPlacement, flow_star_name, try_flow_star_parts,
};
use crate::rules::pattern::context::PatternContext;
use crate::rules::pattern::model::PatternScope;
use crate::rules::pattern::relation::{clamp_branches, san_fang_si_zheng};

/// A pattern-facing star read with its spatial branch.
#[derive(Clone, Copy, Debug)]
pub struct ScopedPatternStarRef<'a> {
    branch: EarthlyBranch,
    placement: &'a StarPlacement,
}

impl<'a> ScopedPatternStarRef<'a> {
    /// Creates a branch-tagged read of a star placement.
    pub const fn new(branch: EarthlyBranch, placement: &'a StarPlacement) -> Self {
        Self { branch, placement }
    }

    /// Returns the branch occupied by this star.
    pub const fn branch(self) -> EarthlyBranch {
        self.branch
    }

    /// Returns the borrowed star placement.
    pub const fn placement(self) -> &'a StarPlacement {
        self.placement
    }
}

/// Maps a core [`Scope`] to the corresponding pattern detection scope.
pub const fn pattern_scope_for(scope: Scope) -> PatternScope {
    match scope {
        Scope::Natal => PatternScope::Natal,
        Scope::Decadal => PatternScope::Decadal,
        Scope::Age => PatternScope::Age,
        Scope::Yearly => PatternScope::Yearly,
        Scope::Monthly => PatternScope::Monthly,
        Scope::Daily => PatternScope::Daily,
        Scope::Hourly => PatternScope::Hourly,
    }
}

/// Returns whether `scope` is visible in the detection context.
pub fn scope_is_visible(ctx: &PatternContext<'_>, scope: Scope) -> bool {
    if !ctx.active_scopes().contains(&scope) {
        return false;
    }
    scope == Scope::Natal || ctx.horoscope_chart().is_some()
}

/// Returns the typed star placements in the palace at `branch`.
pub fn stars_in_palace(chart: &Chart, branch: EarthlyBranch) -> Vec<&StarPlacement> {
    chart
        .palaces()
        .iter()
        .filter(|palace| palace.branch() == branch)
        .flat_map(|palace| palace.stars().iter())
        .collect()
}

/// Returns typed star placements in `branch` for the requested scope.
///
/// Source/layer-specific: reads facts from `scope`'s layer only (or natal facts
/// for [`Scope::Natal`]). This is **not** a selected-state query — use
/// [`selected_stars_in_palace`] for ordinary formation rules.
pub fn stars_in_palace_for_scope<'a>(
    ctx: &PatternContext<'a>,
    scope: Scope,
    branch: EarthlyBranch,
) -> Vec<ScopedPatternStarRef<'a>> {
    if !scope_is_visible(ctx, scope) {
        return Vec::new();
    }

    if scope == Scope::Natal {
        return ctx
            .chart()
            .palaces()
            .iter()
            .filter(|palace| palace.branch() == branch)
            .flat_map(|palace| {
                palace
                    .stars()
                    .iter()
                    .map(move |placement| ScopedPatternStarRef::new(branch, placement))
            })
            .collect();
    }

    let Some(horoscope) = ctx.horoscope_chart() else {
        return Vec::new();
    };
    horoscope
        .layers_in_scope(scope)
        .flat_map(|layer| layer.placements())
        .filter(move |placement| placement.branch() == branch)
        .map(|placement| ScopedPatternStarRef::new(placement.branch(), placement.placement()))
        .collect()
}

/// Source/layer-specific alias for [`stars_in_palace_for_scope`].
///
/// Reads facts from `scope`'s layer only. Prefer this `source_*` name in new
/// source-bound rules to make the single-layer intent explicit.
pub fn source_stars_in_palace<'a>(
    ctx: &PatternContext<'a>,
    scope: Scope,
    branch: EarthlyBranch,
) -> Vec<ScopedPatternStarRef<'a>> {
    stars_in_palace_for_scope(ctx, scope, branch)
}

/// Returns whether `star` occupies the palace at `branch`.
pub fn palace_has_star(chart: &Chart, branch: EarthlyBranch, star: StarName) -> bool {
    stars_in_palace(chart, branch)
        .iter()
        .any(|placement| placement.name() == star)
}

/// Returns whether `star` occupies `branch` in `scope`.
pub fn palace_has_star_for_scope(
    ctx: &PatternContext<'_>,
    scope: Scope,
    branch: EarthlyBranch,
    star: StarName,
) -> bool {
    star_in_palace_for_scope(ctx, scope, branch, star).is_some()
}

/// Returns the actual placement matching `star` in `branch` and `scope`.
///
/// Source/layer-specific: reads `scope`'s layer only, not the selected effective
/// state. Use [`selected_star_in_palace`] for ordinary formation rules.
pub fn star_in_palace_for_scope<'a>(
    ctx: &PatternContext<'a>,
    scope: Scope,
    branch: EarthlyBranch,
    star: StarName,
) -> Option<ScopedPatternStarRef<'a>> {
    stars_in_palace_for_scope(ctx, scope, branch)
        .into_iter()
        .find(|placement| star_matches_for_scope(scope, star, placement.placement().name()))
}

/// Source/layer-specific alias for [`star_in_palace_for_scope`].
pub fn source_star_in_palace<'a>(
    ctx: &PatternContext<'a>,
    scope: Scope,
    branch: EarthlyBranch,
    star: StarName,
) -> Option<ScopedPatternStarRef<'a>> {
    star_in_palace_for_scope(ctx, scope, branch, star)
}

/// Returns the branch occupied by a named palace, if present.
pub fn branch_of_palace(chart: &Chart, palace: PalaceName) -> Option<EarthlyBranch> {
    chart.branch_of_palace(palace)
}

/// Returns the branch occupied by a named palace in `scope`, if present.
///
/// Source/layer-specific: resolves the palace name against `scope`'s own frame
/// only. For the selected palace frame use [`selected_branch_of_palace`].
pub fn branch_of_palace_for_scope(
    ctx: &PatternContext<'_>,
    scope: Scope,
    palace: PalaceName,
) -> Option<EarthlyBranch> {
    if !scope_is_visible(ctx, scope) {
        return None;
    }

    if scope == Scope::Natal {
        return ctx.chart().branch_of_palace(palace);
    }

    ctx.horoscope_chart()?
        .layers_in_scope(scope)
        .find_map(|layer| {
            layer.palace_layout().and_then(|layout| {
                layout
                    .names()
                    .iter()
                    .find(|name| name.palace_name() == palace)
                    .map(|name| name.branch())
            })
        })
}

/// Source/layer-specific alias for [`branch_of_palace_for_scope`].
pub fn source_branch_of_palace(
    ctx: &PatternContext<'_>,
    scope: Scope,
    palace: PalaceName,
) -> Option<EarthlyBranch> {
    branch_of_palace_for_scope(ctx, scope, palace)
}

/// Returns the branch occupied by a named palace in the effective palace frame.
pub fn effective_branch_of_palace(
    ctx: &PatternContext<'_>,
    match_scope: Scope,
    palace: PalaceName,
) -> Option<EarthlyBranch> {
    effective_state_for_match_scope(ctx, match_scope)?.branch_of_palace(palace)
}

/// Returns the scope supplying the selected palace-name frame.
///
/// Thin free-function wrapper over [`PatternContext::selected_frame_scope`] for
/// call sites that read it alongside other query helpers.
pub fn selected_frame_scope(ctx: &PatternContext<'_>) -> Option<Scope> {
    ctx.selected_frame_scope()
}

/// Returns the branch occupied by a named palace in the selected frame.
///
/// Selected-state: reads the selected [`EffectiveChartState`] (selected palace
/// frame + active fact stack). Default for ordinary formation rules.
pub fn selected_branch_of_palace(
    ctx: &PatternContext<'_>,
    palace: PalaceName,
) -> Option<EarthlyBranch> {
    ctx.effective()?.branch_of_palace(palace)
}

fn effective_state_for_match_scope<'a, 'ctx>(
    ctx: &'ctx PatternContext<'a>,
    match_scope: Scope,
) -> Option<&'ctx EffectiveChartState<'a>> {
    let state = ctx.effective()?;
    (state.palace_frame_scope() == match_scope).then_some(state)
}

/// Returns whether `stars` all occupy the palace at `branch`.
pub fn palace_has_all_stars(chart: &Chart, branch: EarthlyBranch, stars: &[StarName]) -> bool {
    stars
        .iter()
        .all(|star| palace_has_star(chart, branch, *star))
}

/// Returns whether every requested star occupies `branch` in `scope`.
pub fn palace_has_all_stars_for_scope(
    ctx: &PatternContext<'_>,
    scope: Scope,
    branch: EarthlyBranch,
    stars: &[StarName],
) -> bool {
    stars
        .iter()
        .all(|star| palace_has_star_for_scope(ctx, scope, branch, *star))
}

/// Returns effective typed star placements in `branch`.
pub fn effective_stars_in_palace<'a>(
    ctx: &PatternContext<'a>,
    branch: EarthlyBranch,
) -> Vec<EffectiveStarRef<'a>> {
    ctx.effective()
        .map(|state| state.stars_in_palace(branch))
        .unwrap_or_default()
}

/// Returns effective typed star placements in `branch` for the selected frame.
///
/// Selected-state: reads the selected [`EffectiveChartState`]. Default for
/// ordinary formation rules.
pub fn selected_stars_in_palace<'a>(
    ctx: &PatternContext<'a>,
    branch: EarthlyBranch,
) -> Vec<EffectiveStarRef<'a>> {
    ctx.effective()
        .map(|state| state.stars_in_palace(branch))
        .unwrap_or_default()
}

/// Returns the actual effective star matching `star` in `branch`.
pub fn effective_star_in_palace<'a>(
    ctx: &PatternContext<'a>,
    match_scope: Scope,
    branch: EarthlyBranch,
    star: StarName,
) -> Option<EffectiveStarRef<'a>> {
    effective_state_for_match_scope(ctx, match_scope)?
        .stars_in_palace(branch)
        .into_iter()
        .find(|placement| star_matches_for_scope(match_scope, star, placement.placement().name()))
}

/// Returns the actual effective star matching `star` in `branch` for the
/// selected frame.
///
/// Selected-state: reads the selected [`EffectiveChartState`]. Default for
/// ordinary formation rules.
pub fn selected_star_in_palace<'a>(
    ctx: &PatternContext<'a>,
    branch: EarthlyBranch,
    star: StarName,
) -> Option<EffectiveStarRef<'a>> {
    let state = ctx.effective()?;
    let frame_scope = state.palace_frame_scope();
    state
        .stars_in_palace(branch)
        .into_iter()
        .find(|placement| star_matches_for_scope(frame_scope, star, placement.placement().name()))
}

/// Returns whether `star` occupies `branch` in the effective state.
pub fn effective_palace_has_star(
    ctx: &PatternContext<'_>,
    match_scope: Scope,
    branch: EarthlyBranch,
    star: StarName,
) -> bool {
    effective_star_in_palace(ctx, match_scope, branch, star).is_some()
}

/// Returns whether `star` occupies `branch` in the selected frame.
///
/// Selected-state: reads the selected [`EffectiveChartState`].
pub fn selected_palace_has_star(
    ctx: &PatternContext<'_>,
    branch: EarthlyBranch,
    star: StarName,
) -> bool {
    selected_star_in_palace(ctx, branch, star).is_some()
}

/// Returns whether every requested star occupies `branch` in the effective state.
pub fn effective_palace_has_all_stars(
    ctx: &PatternContext<'_>,
    match_scope: Scope,
    branch: EarthlyBranch,
    stars: &[StarName],
) -> bool {
    stars
        .iter()
        .all(|star| effective_palace_has_star(ctx, match_scope, branch, *star))
}

/// Returns whether every requested star occupies `branch` in the selected frame.
///
/// Selected-state: reads the selected [`EffectiveChartState`].
pub fn selected_palace_has_all_stars(
    ctx: &PatternContext<'_>,
    branch: EarthlyBranch,
    stars: &[StarName],
) -> bool {
    stars
        .iter()
        .all(|star| selected_palace_has_star(ctx, branch, *star))
}

/// Returns the number of major stars in the palace at `branch`.
pub fn major_star_count_in_palace(chart: &Chart, branch: EarthlyBranch) -> usize {
    stars_in_palace(chart, branch)
        .iter()
        .filter(|placement| placement.kind() == StarKind::Major)
        .count()
}

/// Returns the number of major stars in `branch` for `scope`.
pub fn major_star_count_in_palace_for_scope(
    ctx: &PatternContext<'_>,
    scope: Scope,
    branch: EarthlyBranch,
) -> usize {
    stars_in_palace_for_scope(ctx, scope, branch)
        .iter()
        .filter(|placement| placement.placement().kind() == StarKind::Major)
        .count()
}

/// Returns the number of major stars in `branch` for the selected frame.
///
/// Selected-state: reads the selected [`EffectiveChartState`].
pub fn selected_major_star_count_in_palace(
    ctx: &PatternContext<'_>,
    branch: EarthlyBranch,
) -> usize {
    selected_stars_in_palace(ctx, branch)
        .iter()
        .filter(|placement| placement.placement().kind() == StarKind::Major)
        .count()
}

/// Returns the branch of the palace containing `star`, if present.
pub fn find_star_branch(chart: &Chart, star: StarName) -> Option<EarthlyBranch> {
    chart.star(star).map(|fact| fact.palace().branch())
}

/// Returns the first actual star and branch matching `star` in `scope`.
pub fn find_star_branch_for_scope(
    ctx: &PatternContext<'_>,
    scope: Scope,
    star: StarName,
) -> Option<(StarName, EarthlyBranch)> {
    find_star_for_scope(ctx, scope, star)
        .map(|placement| (placement.placement().name(), placement.branch()))
}

/// Returns the first actual star placement matching `star` in `scope`.
pub fn find_star_for_scope<'a>(
    ctx: &PatternContext<'a>,
    scope: Scope,
    star: StarName,
) -> Option<ScopedPatternStarRef<'a>> {
    if !scope_is_visible(ctx, scope) {
        return None;
    }

    if scope == Scope::Natal {
        return ctx.chart().stars().into_iter().find_map(|fact| {
            star_matches_for_scope(scope, star, fact.placement().name())
                .then(|| ScopedPatternStarRef::new(fact.palace().branch(), fact.placement()))
        });
    }

    let horoscope = ctx.horoscope_chart()?;
    horoscope
        .layers_in_scope(scope)
        .flat_map(|layer| layer.placements())
        .find_map(|placement| {
            star_matches_for_scope(scope, star, placement.placement().name())
                .then(|| ScopedPatternStarRef::new(placement.branch(), placement.placement()))
        })
}

/// Returns each requested star found within the 三方四正 of `anchor`, paired with
/// the branch it was found in.
pub fn stars_in_san_fang_si_zheng(
    chart: &Chart,
    anchor: EarthlyBranch,
    stars: &[StarName],
) -> Vec<(StarName, EarthlyBranch)> {
    let mut found = Vec::new();
    for branch in san_fang_si_zheng(anchor) {
        for placement in stars_in_palace(chart, branch) {
            if stars.contains(&placement.name()) {
                found.push((placement.name(), branch));
            }
        }
    }
    found
}

/// Returns requested stars found within the 三方四正 of `anchor` in `scope`.
///
/// Source/layer-specific: only stars in `scope`'s layer are considered. A natal
/// support star projected into a temporal frame's 三方四正 is **not** seen here —
/// use [`selected_stars_in_san_fang_si_zheng`] for that. This mismatch is the
/// exact distinction that PR #145 fixed.
pub fn stars_in_san_fang_si_zheng_for_scope(
    ctx: &PatternContext<'_>,
    scope: Scope,
    anchor: EarthlyBranch,
    stars: &[StarName],
) -> Vec<(StarName, EarthlyBranch)> {
    let mut found = Vec::new();
    for branch in san_fang_si_zheng(anchor) {
        for placement in stars_in_palace_for_scope(ctx, scope, branch) {
            if stars
                .iter()
                .any(|star| star_matches_for_scope(scope, *star, placement.placement().name()))
            {
                found.push((placement.placement().name(), branch));
            }
        }
    }
    found
}

/// Source/layer-specific alias for [`stars_in_san_fang_si_zheng_for_scope`].
pub fn source_stars_in_san_fang_si_zheng(
    ctx: &PatternContext<'_>,
    scope: Scope,
    anchor: EarthlyBranch,
    stars: &[StarName],
) -> Vec<(StarName, EarthlyBranch)> {
    stars_in_san_fang_si_zheng_for_scope(ctx, scope, anchor, stars)
}

/// Returns requested stars found within the effective 三方四正 of `anchor`.
pub fn effective_stars_in_san_fang_si_zheng(
    ctx: &PatternContext<'_>,
    match_scope: Scope,
    anchor: EarthlyBranch,
    stars: &[StarName],
) -> Vec<(StarName, EarthlyBranch)> {
    let Some(state) = effective_state_for_match_scope(ctx, match_scope) else {
        return Vec::new();
    };
    let mut found = Vec::new();
    for branch in san_fang_si_zheng(anchor) {
        for placement in state.stars_in_palace(branch) {
            if stars.iter().any(|star| {
                star_matches_for_scope(match_scope, *star, placement.placement().name())
            }) {
                found.push((placement.placement().name(), branch));
            }
        }
    }
    found
}

/// Returns requested stars found within the selected 三方四正 of `anchor`.
///
/// Selected-state: reads the selected [`EffectiveChartState`], so natal support
/// stars projected into the selected frame's 三方四正 are visible. Default for
/// ordinary formation rules.
pub fn selected_stars_in_san_fang_si_zheng(
    ctx: &PatternContext<'_>,
    anchor: EarthlyBranch,
    stars: &[StarName],
) -> Vec<(StarName, EarthlyBranch)> {
    let Some(state) = ctx.effective() else {
        return Vec::new();
    };
    let frame_scope = state.palace_frame_scope();
    let mut found = Vec::new();
    for branch in san_fang_si_zheng(anchor) {
        for placement in state.stars_in_palace(branch) {
            if stars.iter().any(|star| {
                star_matches_for_scope(frame_scope, *star, placement.placement().name())
            }) {
                found.push((placement.placement().name(), branch));
            }
        }
    }
    found
}

/// Returns the two stars clamping (夹) the palace at `anchor` when `left_star`
/// and `right_star` occupy its two clamp palaces, one on each side.
///
/// The two clamp palaces are the `-1` and `+1` neighbours of `anchor` (see
/// [`clamp_branches`]); the lower-offset neighbour is the "low" clamp and the
/// higher-offset neighbour is the "high" clamp. Either orientation matches:
/// `left_star` in the low clamp with `right_star` in the high clamp, or the
/// reverse. The returned array is always ordered `[(low_star, low_branch),
/// (high_star, high_branch)]` so callers get a stable clamp ordering. Returns
/// `None` unless both clamp palaces are occupied, one by each requested star.
pub fn clamp_pair_matches(
    chart: &Chart,
    anchor: EarthlyBranch,
    left_star: StarName,
    right_star: StarName,
) -> Option<[(StarName, EarthlyBranch); 2]> {
    let [low, high] = clamp_branches(anchor);

    if palace_has_star(chart, low, left_star) && palace_has_star(chart, high, right_star) {
        Some([(left_star, low), (right_star, high)])
    } else if palace_has_star(chart, low, right_star) && palace_has_star(chart, high, left_star) {
        Some([(right_star, low), (left_star, high)])
    } else {
        None
    }
}

/// Returns a same-scope clamp match, preserving actual matched star names.
///
/// Source/layer-specific: both clamp palaces are read from `scope`'s layer only.
/// For the selected frame use [`selected_clamp_pair_matches`].
pub fn clamp_pair_matches_for_scope(
    ctx: &PatternContext<'_>,
    scope: Scope,
    anchor: EarthlyBranch,
    left_star: StarName,
    right_star: StarName,
) -> Option<[(StarName, EarthlyBranch); 2]> {
    let [low, high] = clamp_branches(anchor);

    let low_left = star_in_palace_for_scope(ctx, scope, low, left_star);
    let high_right = star_in_palace_for_scope(ctx, scope, high, right_star);
    if let (Some(low_left), Some(high_right)) = (low_left, high_right) {
        return Some([
            (low_left.placement().name(), low),
            (high_right.placement().name(), high),
        ]);
    }

    let low_right = star_in_palace_for_scope(ctx, scope, low, right_star);
    let high_left = star_in_palace_for_scope(ctx, scope, high, left_star);
    if let (Some(low_right), Some(high_left)) = (low_right, high_left) {
        return Some([
            (low_right.placement().name(), low),
            (high_left.placement().name(), high),
        ]);
    }

    None
}

/// Source/layer-specific alias for [`clamp_pair_matches_for_scope`].
pub fn source_clamp_pair_matches(
    ctx: &PatternContext<'_>,
    scope: Scope,
    anchor: EarthlyBranch,
    left_star: StarName,
    right_star: StarName,
) -> Option<[(StarName, EarthlyBranch); 2]> {
    clamp_pair_matches_for_scope(ctx, scope, anchor, left_star, right_star)
}

/// Returns a clamp match over the effective state, preserving actual star names.
pub fn effective_clamp_pair_matches(
    ctx: &PatternContext<'_>,
    match_scope: Scope,
    anchor: EarthlyBranch,
    left_star: StarName,
    right_star: StarName,
) -> Option<[(StarName, EarthlyBranch); 2]> {
    let [low, high] = clamp_branches(anchor);

    let low_left = effective_star_in_palace(ctx, match_scope, low, left_star);
    let high_right = effective_star_in_palace(ctx, match_scope, high, right_star);
    if let (Some(low_left), Some(high_right)) = (low_left, high_right) {
        return Some([
            (low_left.placement().name(), low),
            (high_right.placement().name(), high),
        ]);
    }

    let low_right = effective_star_in_palace(ctx, match_scope, low, right_star);
    let high_left = effective_star_in_palace(ctx, match_scope, high, left_star);
    if let (Some(low_right), Some(high_left)) = (low_right, high_left) {
        return Some([
            (low_right.placement().name(), low),
            (high_left.placement().name(), high),
        ]);
    }

    None
}

/// Returns a selected-frame clamp match, preserving actual matched star names.
///
/// Selected-state: both clamp palaces are read from the selected
/// [`EffectiveChartState`]. Default for ordinary formation rules.
pub fn selected_clamp_pair_matches(
    ctx: &PatternContext<'_>,
    anchor: EarthlyBranch,
    left_star: StarName,
    right_star: StarName,
) -> Option<[(StarName, EarthlyBranch); 2]> {
    let [low, high] = clamp_branches(anchor);

    let low_left = selected_star_in_palace(ctx, low, left_star);
    let high_right = selected_star_in_palace(ctx, high, right_star);
    if let (Some(low_left), Some(high_right)) = (low_left, high_right) {
        return Some([
            (low_left.placement().name(), low),
            (high_right.placement().name(), high),
        ]);
    }

    let low_right = selected_star_in_palace(ctx, low, right_star);
    let high_left = selected_star_in_palace(ctx, high, left_star);
    if let (Some(low_right), Some(high_left)) = (low_right, high_left) {
        return Some([
            (low_right.placement().name(), low),
            (high_left.placement().name(), high),
        ]);
    }

    None
}

/// Returns whether `brightness` is a clearly bright/auspicious state
/// (庙/旺/得/利).
///
/// Conservative by design: `Flat` (平) is treated as neutral, and `Weak`,
/// `Trapped`, and `Unknown` are never bright. Rules that depend on brightness
/// must not emit when brightness is `Unknown`; this helper returning `false`
/// for `Unknown` enforces that for the bright case.
pub fn is_bright(brightness: Brightness) -> bool {
    matches!(
        brightness,
        Brightness::Temple
            | Brightness::Prosperous
            | Brightness::Advantage
            | Brightness::Favourable
    )
}

/// Returns whether `brightness` is a clearly dim/fallen state (不/陷).
///
/// Conservative by design: only `Weak` (不) and `Trapped` (陷) count. `Flat`
/// (平) is neutral, and `Unknown` is never dim, so a rule gated on this helper
/// never emits on an uncalculated brightness.
pub fn is_dim(brightness: Brightness) -> bool {
    matches!(brightness, Brightness::Weak | Brightness::Trapped)
}

/// Returns whether any adverse "煞星" (sha star) sits in the palace at `branch`.
///
/// Conservative by design: this uses the already-modeled [`StarKind::Tough`]
/// classification, which currently covers 擎羊 (QingYang), 陀罗 (TuoLuo),
/// 火星 (HuoXing), 铃星 (LingXing), 地空 (DiKong), and 地劫 (DiJie). No stars or
/// aliases are invented here.
pub fn any_sha_star_in_palace(chart: &Chart, branch: EarthlyBranch) -> bool {
    stars_in_palace(chart, branch)
        .iter()
        .any(|placement| placement.kind() == StarKind::Tough)
}

/// Returns whether any adverse "煞星" sits in `branch` for `scope`.
pub fn any_sha_star_in_palace_for_scope(
    ctx: &PatternContext<'_>,
    scope: Scope,
    branch: EarthlyBranch,
) -> bool {
    stars_in_palace_for_scope(ctx, scope, branch)
        .iter()
        .any(|placement| placement.placement().kind() == StarKind::Tough)
}

/// Returns whether `star` is one of the modeled 空亡-family stars.
///
/// This mirrors the existing classical void family without importing the rule
/// engine into `rules::pattern`: 旬空, 空亡, 截路, and 截空 count; 天空, 地空, and
/// 地劫 do not.
pub const fn is_modeled_void_star(star: StarName) -> bool {
    matches!(
        star,
        StarName::XunKong | StarName::KongWang | StarName::JieLu | StarName::JieKong
    )
}

/// Returns the first modeled 空亡-family star sharing `branch`, if any.
pub fn modeled_void_star_in_palace(chart: &Chart, branch: EarthlyBranch) -> Option<StarName> {
    stars_in_palace(chart, branch)
        .iter()
        .map(|placement| placement.name())
        .find(|star| is_modeled_void_star(*star))
}

/// Returns the first modeled 空亡-family star sharing `branch` in `scope`.
pub fn modeled_void_star_in_palace_for_scope(
    ctx: &PatternContext<'_>,
    scope: Scope,
    branch: EarthlyBranch,
) -> Option<StarName> {
    stars_in_palace_for_scope(ctx, scope, branch)
        .iter()
        .map(|placement| placement.placement().name())
        .find(|star| is_modeled_void_star(*star))
}

/// Returns temporal mutagen activations for `scope`.
///
/// Source/layer-specific: returns activations from `scope`'s layer only (empty
/// for [`Scope::Natal`], which carries no temporal mutagen layer). This is a
/// source-bound helper, not a selected-state query; selected-state mutagen
/// support is read from the effective state's activation stack.
pub fn mutagen_activations_for_scope<'a>(
    ctx: &PatternContext<'a>,
    scope: Scope,
) -> Vec<&'a MutagenActivation> {
    if scope == Scope::Natal || !scope_is_visible(ctx, scope) {
        return Vec::new();
    }
    let Some(horoscope) = ctx.horoscope_chart() else {
        return Vec::new();
    };
    horoscope
        .layers_in_scope(scope)
        .flat_map(|layer| layer.activations())
        .collect()
}

/// Source/layer-specific alias for [`mutagen_activations_for_scope`].
pub fn source_mutagen_activations<'a>(
    ctx: &PatternContext<'a>,
    scope: Scope,
) -> Vec<&'a MutagenActivation> {
    mutagen_activations_for_scope(ctx, scope)
}

/// Returns whether `scope` activates `mutagen` on `star` at `branch`.
pub fn star_has_mutagen_activation_for_scope(
    ctx: &PatternContext<'_>,
    scope: Scope,
    star: StarName,
    mutagen: Mutagen,
    branch: EarthlyBranch,
) -> bool {
    mutagen_activations_for_scope(ctx, scope)
        .iter()
        .any(|activation| {
            activation.target_star() == star
                && activation.mutagen() == mutagen
                && activation.target_branch() == branch
        })
}

fn star_matches_for_scope(scope: Scope, requested: StarName, actual: StarName) -> bool {
    if requested == actual {
        return true;
    }

    let Some(flow_scope) = flow_scope_for_scope(scope) else {
        return false;
    };
    let Some(base) = base_flow_match(requested) else {
        return false;
    };

    actual == flow_star_name(flow_scope, base)
        || try_flow_star_parts(actual) == Some((flow_scope, base))
}

fn flow_scope_for_scope(scope: Scope) -> Option<FlowStarScope> {
    match scope {
        Scope::Decadal => Some(FlowStarScope::Decadal),
        Scope::Yearly => Some(FlowStarScope::Yearly),
        Scope::Monthly => Some(FlowStarScope::Monthly),
        Scope::Daily => Some(FlowStarScope::Daily),
        Scope::Hourly => Some(FlowStarScope::Hourly),
        Scope::Natal | Scope::Age => None,
    }
}

fn base_flow_match(star: StarName) -> Option<FlowStarBase> {
    match star {
        StarName::WenChang => Some(FlowStarBase::Chang),
        StarName::WenQu => Some(FlowStarBase::Qu),
        StarName::QingYang => Some(FlowStarBase::Yang),
        StarName::TuoLuo => Some(FlowStarBase::Tuo),
        StarName::TianMa => Some(FlowStarBase::Ma),
        _ => None,
    }
}
