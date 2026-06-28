//! Deterministic decorative runtime star-family placement for the natal chart.
//!
//! The four "twelve gods" families (长生/博士/岁前/将前十二神) reproduce `iztro`
//! 2.5.8 (`getchangsheng12`, `getBoShi12`, `getYearly12` in `src/star`, MIT
//! licensed). Upstream emits these as bare names with no `StarKind`, so they are
//! placed as untyped [`DecorativeStarPlacement`]s rather than
//! [`StarPlacement`](crate::core::model::chart::StarPlacement)s
//! and never appear in [`Chart::stars`].

use crate::core::error::ChartError;
use crate::core::model::chart::{Chart, DecorativeStarFamily, DecorativeStarPlacement, Palace};
use crate::core::model::profile::ChartAlgorithmKind;
use crate::core::model::star::StarName;
use crate::core::model::star::mutagen::Scope;
use crate::core::placement::location::{
    changsheng_start_branch, lu_yang_tuo_ma_branches, twelve_god_direction_forward,
};
use lunar_lite::{EarthlyBranch, HeavenlyStem};

/// 长生十二神 placement order (iztro `getchangsheng12`).
const CHANGSHENG12: [StarName; 12] = [
    StarName::ChangSheng,
    StarName::MuYu,
    StarName::GuanDai,
    StarName::LinGuan,
    StarName::DiWang,
    StarName::Shuai,
    StarName::Bing,
    StarName::Si,
    StarName::Mu,
    StarName::Jue,
    StarName::Tai,
    StarName::Yang,
];

/// 博士十二神 placement order (iztro `getBoShi12`).
const BOSHI12: [StarName; 12] = [
    StarName::BoShi,
    StarName::LiShi,
    StarName::QingLong,
    StarName::XiaoHaoBoshi,
    StarName::JiangJun,
    StarName::ZhouShu,
    StarName::FayLianBoshi,
    StarName::XiShenBoshi,
    StarName::BingFuBoshi,
    StarName::DaHaoBoshi,
    StarName::FuBing,
    StarName::GuanFuBoshi,
];

/// 将前十二神 placement order (iztro `getYearly12` `jq12shen`).
const JIANGQIAN12: [StarName; 12] = [
    StarName::JiangXing,
    StarName::PanAn,
    StarName::SuiYi,
    StarName::XiShenJiangqian,
    StarName::HuaGaiJiangqian,
    StarName::JieSha,
    StarName::ZaiSha,
    StarName::TianSha,
    StarName::ZhiBei,
    StarName::XianChiJiangqian,
    StarName::YueSha,
    StarName::WangShen,
];

/// Inputs required to place the decorative runtime star families.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct DecorativeStarPlacementInput {
    birth_year_stem: HeavenlyStem,
    birth_year_branch: EarthlyBranch,
}

impl DecorativeStarPlacementInput {
    /// Creates decorative placement input from explicit birth-year ganzhi facts.
    pub const fn new(birth_year_stem: HeavenlyStem, birth_year_branch: EarthlyBranch) -> Self {
        Self {
            birth_year_stem,
            birth_year_branch,
        }
    }

    /// Returns the birth year Heavenly Stem.
    pub const fn birth_year_stem(self) -> HeavenlyStem {
        self.birth_year_stem
    }

    /// Returns the birth year Earthly Branch.
    pub const fn birth_year_branch(self) -> EarthlyBranch {
        self.birth_year_branch
    }
}

/// Places decorative runtime star families into a chart.
pub trait DecorativeStarPlacer {
    /// Places decorative star families in `chart` according to `input`.
    fn place_decorative_stars(
        &self,
        chart: Chart,
        input: DecorativeStarPlacementInput,
    ) -> Result<Chart, ChartError>;
}

/// Places the decorative runtime star families deterministically.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct DeterministicDecorativeStarPlacer;

impl DecorativeStarPlacer for DeterministicDecorativeStarPlacer {
    fn place_decorative_stars(
        &self,
        chart: Chart,
        input: DecorativeStarPlacementInput,
    ) -> Result<Chart, ChartError> {
        let placements = decorative_star_placements(&chart, input)?;

        let palaces = chart
            .palaces()
            .iter()
            .map(|palace| {
                let mut decorative = Vec::new();
                for &(branch, name, family) in &placements {
                    if branch == palace.branch() {
                        decorative.push(DecorativeStarPlacement::try_new(
                            name,
                            family,
                            Scope::Natal,
                        )?);
                    }
                }
                Ok(palace.clone().with_decorative_stars(decorative))
            })
            .collect::<Result<Vec<Palace>, ChartError>>()?;

        Chart::try_new(
            chart.birth_context().clone(),
            chart.birth_year(),
            chart.method_profile().clone(),
            palaces,
            chart.body_palace_branch(),
            chart.five_element_bureau(),
        )
    }
}

/// Computes the branch, name, and family of every decorative entry.
fn decorative_star_placements(
    chart: &Chart,
    input: DecorativeStarPlacementInput,
) -> Result<Vec<(EarthlyBranch, StarName, DecorativeStarFamily)>, ChartError> {
    let bureau = chart
        .five_element_bureau()
        .ok_or(ChartError::RequiredFiveElementBureauMissing)?;
    let gender = chart.birth_context().gender();
    let year_stem = input.birth_year_stem();
    let year_branch = input.birth_year_branch();
    let forward = twelve_god_direction_forward(gender, year_branch);

    let mut placements = Vec::with_capacity(48);

    // 长生十二神: start from the bureau's branch, 阳男阴女顺行.
    place_twelve(
        &mut placements,
        changsheng_start_branch(bureau),
        forward,
        &CHANGSHENG12,
        DecorativeStarFamily::Changsheng12,
    );

    // 博士十二神: start from 禄存, same direction as 长生十二神.
    let (lu, _, _, _) = lu_yang_tuo_ma_branches(year_stem, year_branch);
    place_twelve(
        &mut placements,
        lu,
        forward,
        &BOSHI12,
        DecorativeStarFamily::Boshi12,
    );

    // 岁前/将前十二神 depend only on the year branch and algorithm, so they are
    // shared with yearly-scope temporal decorative placement.
    placements.extend(suiqian_jiangqian12_placements(
        year_branch,
        chart.method_profile().algorithm_kind(),
    ));

    Ok(placements)
}

/// Computes the 岁前/将前十二神 placements anchored on a year branch.
///
/// Shared by natal decorative placement and yearly-scope temporal decorative
/// placement (`yearlyDecStar`): upstream derives both from the same rule, keyed
/// by the relevant year branch. 岁前 starts on the year branch; 将前 starts on the
/// year-branch triad anchor; both run forward. The Zhongzhou algorithm renames
/// the seventh 岁前 entry 大耗 to 岁破.
pub(crate) fn suiqian_jiangqian12_placements(
    year_branch: EarthlyBranch,
    algorithm: ChartAlgorithmKind,
) -> Vec<(EarthlyBranch, StarName, DecorativeStarFamily)> {
    let mut placements = Vec::with_capacity(24);

    place_twelve(
        &mut placements,
        year_branch,
        true,
        &suiqian12_names(algorithm),
        DecorativeStarFamily::Suiqian12,
    );

    place_twelve(
        &mut placements,
        jiangqian12_start_branch(year_branch),
        true,
        &JIANGQIAN12,
        DecorativeStarFamily::Jiangqian12,
    );

    placements
}

/// Appends a twelve-entry family, advancing forward or backward from `start`.
fn place_twelve(
    out: &mut Vec<(EarthlyBranch, StarName, DecorativeStarFamily)>,
    start: EarthlyBranch,
    forward: bool,
    names: &[StarName; 12],
    family: DecorativeStarFamily,
) {
    for (i, &name) in names.iter().enumerate() {
        let delta = if forward { i as isize } else { -(i as isize) };
        out.push((start.offset(delta), name, family));
    }
}

/// Returns the 岁前十二神 names, renaming 大耗 to 岁破 under Zhongzhou.
fn suiqian12_names(algorithm: ChartAlgorithmKind) -> [StarName; 12] {
    let seventh = match algorithm {
        ChartAlgorithmKind::Zhongzhou => StarName::SuiPo,
        ChartAlgorithmKind::QuanShu | ChartAlgorithmKind::Placeholder => StarName::DaHaoSuiqian,
    };

    [
        StarName::SuiJian,
        StarName::HuiQi,
        StarName::SangMen,
        StarName::GuanSuo,
        StarName::GuanFuSuiqian,
        StarName::XiaoHaoSuiqian,
        seventh,
        StarName::LongDeSuiqian,
        StarName::BaiHu,
        StarName::TianDeSuiqian,
        StarName::DiaoKe,
        StarName::BingFuSuiqian,
    ]
}

/// Returns the 将星 starting branch for a year branch (iztro
/// `getJiangqian12StartIndex`): 寅午戌→午, 申子辰→子, 巳酉丑→酉, 亥卯未→卯.
const fn jiangqian12_start_branch(year_branch: EarthlyBranch) -> EarthlyBranch {
    match year_branch {
        EarthlyBranch::Yin | EarthlyBranch::Wu | EarthlyBranch::Xu => EarthlyBranch::Wu,
        EarthlyBranch::Shen | EarthlyBranch::Zi | EarthlyBranch::Chen => EarthlyBranch::Zi,
        EarthlyBranch::Si | EarthlyBranch::You | EarthlyBranch::Chou => EarthlyBranch::You,
        EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Wei => EarthlyBranch::Mao,
    }
}
