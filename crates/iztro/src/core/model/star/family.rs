//! Classical star families (昌/曲/羊/陀/马) and explicit star selectors.
//!
//! A [`StarFamily`] links a natal base star with its temporal flow variants
//! (运/流/月/日/时). Family membership is *taxonomy*, not identity: 文曲
//! ([`StarName::WenQu`]) and 流曲 ([`StarName::LiuQu`]) share
//! [`StarFamily::Qu`], but they remain distinct [`StarName`] identities and are
//! never equal. Generic star matching stays exact by default; a detector must
//! *explicitly* opt into family-level semantics via [`StarSelector::Family`].

use serde::{Deserialize, Serialize};

use crate::core::model::star::flow::{
    FlowStarBase, FlowStarScope, flow_star_name, try_flow_star_parts,
};
use crate::core::model::star::mutagen::Scope;
use crate::core::model::star::name::StarName;

/// A classical star family linking a natal base star with its temporal flow
/// variants.
///
/// The modeled families are the ones that carry a shared 昌/曲/羊/陀/马 lineage
/// across the natal chart and every temporal layer. Family membership is a
/// grouping key for taxonomy, documentation, and explicit family-level rules; it
/// is deliberately **not** an equality relation between [`StarName`]s.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StarFamily {
    /// 昌 family: 文昌 and its 运昌/流昌/月昌/日昌/时昌 flow variants.
    Chang,
    /// 曲 family: 文曲 and its 运曲/流曲/月曲/日曲/时曲 flow variants.
    Qu,
    /// 羊 family: 擎羊 and its 运羊/流羊/月羊/日羊/时羊 flow variants.
    Yang,
    /// 陀 family: 陀罗 and its 运陀/流陀/月陀/日陀/时陀 flow variants.
    Tuo,
    /// 马 family: 天马 and its 运马/流马/月马/日马/时马 flow variants.
    Ma,
}

impl StarFamily {
    /// The natal base member of this family.
    pub const fn base_star(self) -> StarName {
        match self {
            StarFamily::Chang => StarName::WenChang,
            StarFamily::Qu => StarName::WenQu,
            StarFamily::Yang => StarName::QingYang,
            StarFamily::Tuo => StarName::TuoLuo,
            StarFamily::Ma => StarName::TianMa,
        }
    }

    /// The normalized flow-star base identity for this family.
    const fn flow_base(self) -> FlowStarBase {
        match self {
            StarFamily::Chang => FlowStarBase::Chang,
            StarFamily::Qu => FlowStarBase::Qu,
            StarFamily::Yang => FlowStarBase::Yang,
            StarFamily::Tuo => FlowStarBase::Tuo,
            StarFamily::Ma => FlowStarBase::Ma,
        }
    }

    /// Resolves the exact [`StarName`] member of this family for a source/
    /// evaluation layer `scope`.
    ///
    /// Natal (and 岁 [`Scope::Age`], which carries no flow layer) resolve to the
    /// natal [`base_star`](Self::base_star); every temporal scope resolves to the
    /// scope-specific flow star (流曲, 运曲, …). The returned value is an **exact**
    /// star identity, not a family/equivalence matcher: a detector uses this to
    /// ask for "the 曲 star of *this* temporal layer" without reintroducing
    /// base↔flow equivalence into generic matching. Here `scope` names the
    /// source/flow layer, not the selected palace frame.
    pub const fn exact_member_for_scope(self, scope: Scope) -> StarName {
        match flow_star_scope_for_scope(scope) {
            Some(flow_scope) => flow_star_name(flow_scope, self.flow_base()),
            None => self.base_star(),
        }
    }
}

impl StarName {
    /// Returns the [`StarFamily`] this star belongs to, if any.
    ///
    /// Both the natal base star and every temporal flow variant map to the same
    /// family. This is taxonomy only: it never makes two distinct [`StarName`]s
    /// compare equal, and most stars belong to no modeled family (`None`).
    pub const fn family(self) -> Option<StarFamily> {
        match try_flow_star_parts(self) {
            Some((_, base)) => flow_base_family(base),
            None => match self {
                StarName::WenChang => Some(StarFamily::Chang),
                StarName::WenQu => Some(StarFamily::Qu),
                StarName::QingYang => Some(StarFamily::Yang),
                StarName::TuoLuo => Some(StarFamily::Tuo),
                StarName::TianMa => Some(StarFamily::Ma),
                _ => None,
            },
        }
    }
}

/// Maps a [`Scope`] to its horoscope flow-star scope, if it has one.
///
/// Natal and 岁 (age) scopes carry no flow-star layer and return `None`.
pub const fn flow_star_scope_for_scope(scope: Scope) -> Option<FlowStarScope> {
    match scope {
        Scope::Decadal => Some(FlowStarScope::Decadal),
        Scope::Yearly => Some(FlowStarScope::Yearly),
        Scope::Monthly => Some(FlowStarScope::Monthly),
        Scope::Daily => Some(FlowStarScope::Daily),
        Scope::Hourly => Some(FlowStarScope::Hourly),
        Scope::Natal | Scope::Age => None,
    }
}

/// Maps a normalized flow-star base to a modeled classical family, if any.
///
/// Only the 昌/曲/羊/陀/马 lineages are modeled as [`StarFamily`]; the remaining
/// flow bases (魁/钺/禄/鸾/喜) have no classical-pattern family and return `None`.
const fn flow_base_family(base: FlowStarBase) -> Option<StarFamily> {
    match base {
        FlowStarBase::Chang => Some(StarFamily::Chang),
        FlowStarBase::Qu => Some(StarFamily::Qu),
        FlowStarBase::Yang => Some(StarFamily::Yang),
        FlowStarBase::Tuo => Some(StarFamily::Tuo),
        FlowStarBase::Ma => Some(StarFamily::Ma),
        FlowStarBase::Kui
        | FlowStarBase::Yue
        | FlowStarBase::Lu
        | FlowStarBase::Luan
        | FlowStarBase::Xi => None,
    }
}

/// An explicit request for either one exact star identity or a whole family.
///
/// This is the type detectors use to say what they mean. [`Exact`] is the
/// default classical semantics: `Exact(WenQu)` matches only 文曲, never 流曲.
/// [`Family`] is opt-in: `Family(Qu)` matches every 曲-family star (文曲, 流曲,
/// 运曲, …). There is no implicit base↔flow equivalence — a caller must choose.
///
/// [`Exact`]: StarSelector::Exact
/// [`Family`]: StarSelector::Family
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum StarSelector {
    /// Matches exactly one [`StarName`] identity.
    Exact(StarName),
    /// Matches any member of a [`StarFamily`].
    Family(StarFamily),
}

impl StarSelector {
    /// Returns whether `actual` satisfies this selector.
    pub fn matches(self, actual: StarName) -> bool {
        match self {
            StarSelector::Exact(star) => star == actual,
            StarSelector::Family(family) => actual.family() == Some(family),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every base star and its temporal flow variants share one family, yet each
    /// remains a distinct [`StarName`] identity — family is taxonomy, not equality.
    #[test]
    fn star_family_links_base_and_flow_but_identity_remains_distinct() {
        // (base, family, [flow variants across 运/流/月/日/时]).
        let groups: [(StarName, StarFamily, [StarName; 5]); 5] = [
            (
                StarName::WenChang,
                StarFamily::Chang,
                [
                    StarName::YunChang,
                    StarName::LiuChang,
                    StarName::YueChang,
                    StarName::RiChang,
                    StarName::ShiChang,
                ],
            ),
            (
                StarName::WenQu,
                StarFamily::Qu,
                [
                    StarName::YunQu,
                    StarName::LiuQu,
                    StarName::YueQu,
                    StarName::RiQu,
                    StarName::ShiQu,
                ],
            ),
            (
                StarName::QingYang,
                StarFamily::Yang,
                [
                    StarName::YunYang,
                    StarName::LiuYang,
                    StarName::YueYang,
                    StarName::RiYang,
                    StarName::ShiYang,
                ],
            ),
            (
                StarName::TuoLuo,
                StarFamily::Tuo,
                [
                    StarName::YunTuo,
                    StarName::LiuTuo,
                    StarName::YueTuo,
                    StarName::RiTuo,
                    StarName::ShiTuo,
                ],
            ),
            (
                StarName::TianMa,
                StarFamily::Ma,
                [
                    StarName::YunMa,
                    StarName::LiuMa,
                    StarName::YueMa,
                    StarName::RiMa,
                    StarName::ShiMa,
                ],
            ),
        ];

        for (base, family, flows) in groups {
            assert_eq!(base.family(), Some(family));
            assert_eq!(family.base_star(), base);
            for flow in flows {
                assert_ne!(base, flow, "base and flow must be distinct identities");
                assert_eq!(
                    flow.family(),
                    Some(family),
                    "flow variant shares the base family"
                );
            }
        }
    }

    #[test]
    fn family_resolves_exact_member_per_scope() {
        assert_eq!(
            StarFamily::Qu.exact_member_for_scope(Scope::Natal),
            StarName::WenQu
        );
        assert_eq!(
            StarFamily::Qu.exact_member_for_scope(Scope::Yearly),
            StarName::LiuQu
        );
        assert_eq!(
            StarFamily::Yang.exact_member_for_scope(Scope::Decadal),
            StarName::YunYang
        );
        assert_eq!(
            StarFamily::Tuo.exact_member_for_scope(Scope::Monthly),
            StarName::YueTuo
        );
        // 岁 (age) carries no flow layer, so it resolves to the natal base.
        assert_eq!(
            StarFamily::Ma.exact_member_for_scope(Scope::Age),
            StarName::TianMa
        );
    }

    #[test]
    fn unrelated_and_non_family_flow_stars_have_no_family() {
        assert_eq!(StarName::ZiWei.family(), None);
        assert_eq!(StarName::TaiYang.family(), None);
        // 禄/魁/钺/鸾/喜 flow lineages are not modeled as classical families.
        assert_eq!(StarName::LiuLu.family(), None);
        assert_eq!(StarName::LiuKui.family(), None);
    }

    #[test]
    fn exact_selector_matches_only_that_identity() {
        assert!(StarSelector::Exact(StarName::WenQu).matches(StarName::WenQu));
        assert!(!StarSelector::Exact(StarName::WenQu).matches(StarName::LiuQu));
        assert!(StarSelector::Exact(StarName::LiuQu).matches(StarName::LiuQu));
        assert!(!StarSelector::Exact(StarName::LiuQu).matches(StarName::WenQu));
        // Exact base-blade queries never match flow blades.
        assert!(!StarSelector::Exact(StarName::QingYang).matches(StarName::LiuYang));
        assert!(!StarSelector::Exact(StarName::TuoLuo).matches(StarName::LiuTuo));
    }

    #[test]
    fn family_qu_selector_matches_wen_qu_and_liu_qu() {
        let qu = StarSelector::Family(StarFamily::Qu);
        assert!(qu.matches(StarName::WenQu));
        assert!(qu.matches(StarName::LiuQu));
        assert!(qu.matches(StarName::YunQu));
        // But not the 昌 family, and not unrelated stars.
        assert!(!qu.matches(StarName::WenChang));
        assert!(!qu.matches(StarName::LiuChang));
        assert!(!qu.matches(StarName::ZiWei));
    }
}
