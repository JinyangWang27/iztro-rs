//! Generic read-only query helpers shared by rule engines.
//!
//! These helpers operate on chart facts and the shared
//! [`RuleEvaluationContext`](crate::core::RuleEvaluationContext). Rule engines
//! keep their own domain-specific wrappers, request types, and output models.

use crate::core::{
    Brightness, Chart, EarthlyBranch, EffectiveChartState, EffectiveStarRef, FlowStarBase,
    FlowStarScope, PalaceName, RuleEvaluationContext, Scope, StarName, StarPlacement,
    flow_star_name, try_flow_star_parts,
};

/// Returns the typed star placements in the natal palace at `branch`.
pub fn stars_in_palace(chart: &Chart, branch: EarthlyBranch) -> Vec<&StarPlacement> {
    chart
        .palaces()
        .iter()
        .filter(|palace| palace.branch() == branch)
        .flat_map(|palace| palace.stars().iter())
        .collect()
}

/// Returns whether `star` occupies the natal palace at `branch`.
pub fn palace_has_star(chart: &Chart, branch: EarthlyBranch, star: StarName) -> bool {
    stars_in_palace(chart, branch)
        .iter()
        .any(|placement| placement.name() == star)
}

/// Returns the branch occupied by a named natal palace, if present.
pub fn branch_of_palace(chart: &Chart, palace: PalaceName) -> Option<EarthlyBranch> {
    chart.branch_of_palace(palace)
}

/// Returns the branch occupied by a named palace in the effective palace frame
/// for `match_scope`.
pub fn effective_branch_of_palace(
    ctx: &RuleEvaluationContext<'_>,
    match_scope: Scope,
    palace: PalaceName,
) -> Option<EarthlyBranch> {
    effective_state_for_match_scope(ctx, match_scope)?.branch_of_palace(palace)
}

/// Returns the scope supplying the selected palace-name frame.
pub fn selected_frame_scope(ctx: &RuleEvaluationContext<'_>) -> Option<Scope> {
    ctx.selected_frame_scope()
}

/// Returns the branch occupied by a named palace in the selected frame.
pub fn selected_branch_of_palace(
    ctx: &RuleEvaluationContext<'_>,
    palace: PalaceName,
) -> Option<EarthlyBranch> {
    ctx.effective()?.branch_of_palace(palace)
}

fn effective_state_for_match_scope<'a, 'ctx>(
    ctx: &'ctx RuleEvaluationContext<'a>,
    match_scope: Scope,
) -> Option<&'ctx EffectiveChartState<'a>> {
    let state = ctx.effective()?;
    (state.palace_frame_scope() == match_scope).then_some(state)
}

/// Returns whether `stars` all occupy the natal palace at `branch`.
pub fn palace_has_all_stars(chart: &Chart, branch: EarthlyBranch, stars: &[StarName]) -> bool {
    stars
        .iter()
        .all(|star| palace_has_star(chart, branch, *star))
}

/// Returns effective typed star placements in `branch`.
pub fn effective_stars_in_palace<'a>(
    ctx: &RuleEvaluationContext<'a>,
    branch: EarthlyBranch,
) -> Vec<EffectiveStarRef<'a>> {
    ctx.effective()
        .map(|state| state.stars_in_palace(branch))
        .unwrap_or_default()
}

/// Returns effective typed star placements in `branch` for the selected frame.
pub fn selected_stars_in_palace<'a>(
    ctx: &RuleEvaluationContext<'a>,
    branch: EarthlyBranch,
) -> Vec<EffectiveStarRef<'a>> {
    ctx.effective()
        .map(|state| state.stars_in_palace(branch))
        .unwrap_or_default()
}

/// Returns the actual effective star matching `star` in `branch`.
pub fn effective_star_in_palace<'a>(
    ctx: &RuleEvaluationContext<'a>,
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
pub fn selected_star_in_palace<'a>(
    ctx: &RuleEvaluationContext<'a>,
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
    ctx: &RuleEvaluationContext<'_>,
    match_scope: Scope,
    branch: EarthlyBranch,
    star: StarName,
) -> bool {
    effective_star_in_palace(ctx, match_scope, branch, star).is_some()
}

/// Returns whether `star` occupies `branch` in the selected frame.
pub fn selected_palace_has_star(
    ctx: &RuleEvaluationContext<'_>,
    branch: EarthlyBranch,
    star: StarName,
) -> bool {
    selected_star_in_palace(ctx, branch, star).is_some()
}

/// Returns whether every requested star occupies `branch` in the effective state.
pub fn effective_palace_has_all_stars(
    ctx: &RuleEvaluationContext<'_>,
    match_scope: Scope,
    branch: EarthlyBranch,
    stars: &[StarName],
) -> bool {
    stars
        .iter()
        .all(|star| effective_palace_has_star(ctx, match_scope, branch, *star))
}

/// Returns whether every requested star occupies `branch` in the selected frame.
pub fn selected_palace_has_all_stars(
    ctx: &RuleEvaluationContext<'_>,
    branch: EarthlyBranch,
    stars: &[StarName],
) -> bool {
    stars
        .iter()
        .all(|star| selected_palace_has_star(ctx, branch, *star))
}

/// Returns the branch of the natal palace containing `star`, if present.
pub fn find_star_branch(chart: &Chart, star: StarName) -> Option<EarthlyBranch> {
    chart.star(star).map(|fact| fact.palace().branch())
}

/// Returns the two stars clamping (夹) the palace at `anchor` in natal facts.
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

/// Returns a clamp match over the effective state, preserving actual star names.
pub fn effective_clamp_pair_matches(
    ctx: &RuleEvaluationContext<'_>,
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
pub fn selected_clamp_pair_matches(
    ctx: &RuleEvaluationContext<'_>,
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

/// Returns whether `brightness` is a clearly bright/auspicious state.
pub fn is_bright(brightness: Brightness) -> bool {
    matches!(
        brightness,
        Brightness::Temple
            | Brightness::Prosperous
            | Brightness::Advantage
            | Brightness::Favourable
    )
}

/// Returns whether `brightness` is a clearly dim/fallen state.
pub fn is_dim(brightness: Brightness) -> bool {
    matches!(brightness, Brightness::Weak | Brightness::Trapped)
}

pub(crate) fn star_matches_for_scope(scope: Scope, requested: StarName, actual: StarName) -> bool {
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

fn clamp_branches(branch: EarthlyBranch) -> [EarthlyBranch; 2] {
    [branch.offset(-1), branch.offset(1)]
}
