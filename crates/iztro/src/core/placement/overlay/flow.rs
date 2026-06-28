//! Deterministic scoped flow-star (流耀) placement.
//!
//! Reproduces `iztro` 2.5.8 `getHoroscopeStar` (`src/star/horoscopeStar.ts`, MIT
//! licensed). One scope-generic algorithm places the ten matrix flow stars
//! (魁钺昌曲禄羊陀马鸾喜) for every horoscope scope, parameterized only by the
//! period's stem-branch; the yearly scope additionally places 年解 (`NianJieYearly`),
//! which is not part of the [`FlowStarBase`] matrix.
//!
//! Placement is fully determined by the period stem-branch — no natal chart and
//! no horoscope palace-name derivation is required. Each placement records its
//! branch through [`ScopedStarPlacement`].

use crate::core::error::ChartError;
use crate::core::model::chart::{
    ScopedStarPlacement, StarPlacement, TemporalContext, TemporalLayer,
};
use lunar_lite::{EarthlyBranch, HeavenlyStem};
use crate::core::model::star::mutagen::Scope;
use crate::core::model::star::{
    Brightness, FlowStarBase, FlowStarScope, StarName, flow_star_name, known_star_metadata,
};
use crate::core::placement::location::{
    chang_qu_branches_by_stem, kui_yue_branches, lu_yang_tuo_ma_branches, luan_xi_branches,
    nian_jie_branch,
};

/// Builds the scoped flow-star placements for one horoscope period.
///
/// Returns a [`TemporalLayer`] holding only the period's flow-star placements
/// (no mutagen activations). The layer scope is taken from `context`.
pub fn build_flow_star_layer(context: TemporalContext) -> Result<TemporalLayer, ChartError> {
    if context.scope() == Scope::Age {
        return Err(ChartError::FlowStarsUnavailableForScope { scope: Scope::Age });
    }

    let flow_scope = flow_scope_of(&context);
    let scope = context.scope();
    let stem_branch = context.stem_branch();
    let stem = stem_branch.stem();
    let branch = stem_branch.branch();

    let mut placements = Vec::with_capacity(11);

    // 年解 is yearly-only and sits outside the FlowStarBase matrix; iztro pushes
    // it before the matrix stars.
    if flow_scope == FlowStarScope::Yearly {
        placements.push(scoped_placement(
            StarName::NianJieYearly,
            nian_jie_branch(branch),
            scope,
        ));
    }

    for (matrix_branch, base) in flow_star_branches(stem, branch) {
        placements.push(scoped_placement(
            flow_star_name(flow_scope, base),
            matrix_branch,
            scope,
        ));
    }

    TemporalLayer::try_new(scope, context, placements, Vec::new())
}

/// Returns the branch of each of the ten matrix flow stars for a stem-branch.
fn flow_star_branches(
    stem: HeavenlyStem,
    branch: EarthlyBranch,
) -> [(EarthlyBranch, FlowStarBase); 10] {
    let (kui, yue) = kui_yue_branches(stem);
    let (chang, qu) = chang_qu_branches_by_stem(stem);
    let (lu, yang, tuo, ma) = lu_yang_tuo_ma_branches(stem, branch);
    let (luan, xi) = luan_xi_branches(branch);

    [
        (kui, FlowStarBase::Kui),
        (yue, FlowStarBase::Yue),
        (chang, FlowStarBase::Chang),
        (qu, FlowStarBase::Qu),
        (lu, FlowStarBase::Lu),
        (yang, FlowStarBase::Yang),
        (tuo, FlowStarBase::Tuo),
        (ma, FlowStarBase::Ma),
        (luan, FlowStarBase::Luan),
        (xi, FlowStarBase::Xi),
    ]
}

/// Builds one branch-positioned flow placement, sourcing the `StarKind` from the
/// known-star inventory (every flow star carries a concrete kind upstream).
fn scoped_placement(name: StarName, branch: EarthlyBranch, scope: Scope) -> ScopedStarPlacement {
    let kind = known_star_metadata(name)
        .kind()
        .expect("flow stars carry a concrete StarKind");

    ScopedStarPlacement::new(
        branch,
        StarPlacement::new(name, kind, Brightness::Unknown, None, scope),
    )
}

/// Maps a temporal context to its flow-star scope. Total because
/// [`TemporalContext`] has no natal variant.
fn flow_scope_of(context: &TemporalContext) -> FlowStarScope {
    match context {
        TemporalContext::Age { .. } => unreachable!("age scope has no flow-star mapping"),
        TemporalContext::Decadal { .. } => FlowStarScope::Decadal,
        TemporalContext::Yearly { .. } => FlowStarScope::Yearly,
        TemporalContext::Monthly { .. } => FlowStarScope::Monthly,
        TemporalContext::Daily { .. } => FlowStarScope::Daily,
        TemporalContext::Hourly { .. } => FlowStarScope::Hourly,
    }
}
