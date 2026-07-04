use crate::core::model::star::{StarCategory, StarName, star_metadata};
use lunar_lite::HeavenlyStem;
use serde::{Deserialize, Serialize};

/// Four transformations, also known as mutagens.
///
/// The derived [`Ord`]/[`PartialOrd`] follow the variant declaration order and
/// exist only to give facade/export snapshots a stable, deterministic star
/// ordering key (see [`crate::core::model::chart::facade_snapshot`]). They do not
/// affect placement.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mutagen {
    /// Lu transformation (化禄).
    Lu,
    /// Quan transformation (化权).
    Quan,
    /// Ke transformation (化科).
    Ke,
    /// Ji transformation (化忌).
    Ji,
}

/// Time scope for a chart fact or transformation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    /// Natal chart (本命盘).
    Natal,
    /// Minor Limit / nominal-age period (小限).
    ///
    /// 小限 is keyed by nominal age (虚岁): each year of life advances one
    /// palace from a birth-branch-determined start, stepping by gender. It is
    /// the domain representation of the Minor Limit and is distinct from
    /// [`Yearly`](Self::Yearly) (流年): 小限 is age-driven, whereas 流年 is the
    /// selected calendar year, keyed by its stem-branch / 太岁.
    Age,
    /// Decadal period (大限).
    Decadal,
    /// Yearly period (流年).
    ///
    /// 流年 is the selected calendar year, keyed by its stem-branch (太岁). It
    /// is distinct from [`Age`](Self::Age) (小限), which is the nominal-age
    /// (虚岁) Minor Limit marker.
    Yearly,
    /// Monthly period (流月).
    Monthly,
    /// Daily period (流日).
    Daily,
    /// Hourly period (流时).
    Hourly,
}

/// Returns the four stars a Heavenly Stem transforms, paired with the mutagen
/// each receives, in canonical 禄 / 权 / 科 / 忌 (Lu, Quan, Ke, Ji) order.
///
/// This is the forward 十干四化 table: given a stem it lists exactly which four
/// stars are transformed and how. It mirrors iztro 2.5.8 and is the single
/// source of truth from which [`birth_year_star_mutagen`] is derived. Callers
/// that need the four targets of a stem (for example palace-stem mutagen-flow
/// derivation) should read this directly rather than probing every star with
/// [`birth_year_star_mutagen`], because it enumerates the targets deterministically
/// and independently of which stars a chart happens to place.
pub const fn stem_mutagen_targets(year_stem: HeavenlyStem) -> [(Mutagen, StarName); 4] {
    use Mutagen::{Ji, Ke, Lu, Quan};

    match year_stem {
        HeavenlyStem::Jia => [
            (Lu, StarName::LianZhen),
            (Quan, StarName::PoJun),
            (Ke, StarName::WuQu),
            (Ji, StarName::TaiYang),
        ],
        HeavenlyStem::Yi => [
            (Lu, StarName::TianJi),
            (Quan, StarName::TianLiang),
            (Ke, StarName::ZiWei),
            (Ji, StarName::TaiYin),
        ],
        HeavenlyStem::Bing => [
            (Lu, StarName::TianTong),
            (Quan, StarName::TianJi),
            (Ke, StarName::WenChang),
            (Ji, StarName::LianZhen),
        ],
        HeavenlyStem::Ding => [
            (Lu, StarName::TaiYin),
            (Quan, StarName::TianTong),
            (Ke, StarName::TianJi),
            (Ji, StarName::JuMen),
        ],
        HeavenlyStem::Wu => [
            (Lu, StarName::TanLang),
            (Quan, StarName::TaiYin),
            (Ke, StarName::YouBi),
            (Ji, StarName::TianJi),
        ],
        HeavenlyStem::Ji => [
            (Lu, StarName::WuQu),
            (Quan, StarName::TanLang),
            (Ke, StarName::TianLiang),
            (Ji, StarName::WenQu),
        ],
        HeavenlyStem::Geng => [
            (Lu, StarName::TaiYang),
            (Quan, StarName::WuQu),
            (Ke, StarName::TaiYin),
            (Ji, StarName::TianTong),
        ],
        HeavenlyStem::Xin => [
            (Lu, StarName::JuMen),
            (Quan, StarName::TaiYang),
            (Ke, StarName::WenQu),
            (Ji, StarName::WenChang),
        ],
        HeavenlyStem::Ren => [
            (Lu, StarName::TianLiang),
            (Quan, StarName::ZiWei),
            (Ke, StarName::ZuoFu),
            (Ji, StarName::WuQu),
        ],
        HeavenlyStem::Gui => [
            (Lu, StarName::PoJun),
            (Quan, StarName::JuMen),
            (Ke, StarName::TaiYin),
            (Ji, StarName::TanLang),
        ],
    }
}

/// Returns the birth-year mutagen for any represented star, if supported.
///
/// The table mirrors iztro 2.5.8 Heavenly Stem mutagens in Lu, Quan, Ke, Ji
/// order for represented major stars and represented minor targets. It is a
/// reverse lookup over [`stem_mutagen_targets`], so the two never drift.
pub fn birth_year_star_mutagen(year_stem: HeavenlyStem, star: StarName) -> Option<Mutagen> {
    stem_mutagen_targets(year_stem)
        .into_iter()
        .find_map(|(mutagen, target)| (target == star).then_some(mutagen))
}

/// Returns the birth-year mutagen for a represented major star, if supported.
pub fn birth_year_major_star_mutagen(year_stem: HeavenlyStem, star: StarName) -> Option<Mutagen> {
    if star_metadata(star).category() != StarCategory::Major {
        return None;
    }

    birth_year_star_mutagen(year_stem, star)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lunar_lite::HEAVENLY_STEMS;

    #[test]
    fn each_stem_transforms_four_stars_in_lu_quan_ke_ji_order() {
        for &stem in &HEAVENLY_STEMS {
            let mutagens: Vec<Mutagen> = stem_mutagen_targets(stem)
                .into_iter()
                .map(|(mutagen, _)| mutagen)
                .collect();
            assert_eq!(
                mutagens,
                vec![Mutagen::Lu, Mutagen::Quan, Mutagen::Ke, Mutagen::Ji],
                "{stem:?} should transform four stars in 禄/权/科/忌 order",
            );
        }
    }

    #[test]
    fn birth_year_lookup_agrees_with_forward_table() {
        for &stem in &HEAVENLY_STEMS {
            for (mutagen, star) in stem_mutagen_targets(stem) {
                assert_eq!(
                    birth_year_star_mutagen(stem, star),
                    Some(mutagen),
                    "reverse lookup must agree with the forward table for {stem:?} -> {star:?}",
                );
            }
        }
    }
}
