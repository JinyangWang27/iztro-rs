//! Deterministic adjective-star (杂曜) placement for the natal chart.
//!
//! Reproduces a supported subset of `iztro` 2.5.8 `adjectiveStars`
//! (`getAdjectiveStar` plus the `getLuanXiIndex`, `getMonthlyStarIndex`,
//! `getTimelyStarIndex`, `getDailyStarIndex`, and `getYearlyStarIndex` helpers
//! in `src/star`, MIT licensed). Only these eighteen deterministic stars are
//! implemented here:
//!
//! - 红鸾 (HongLuan) / 天喜 (TianXi): from the birth year branch;
//! - 天姚 (TianYao) / 天刑 (TianXing): from the lunar month;
//! - 台辅 (TaiFu) / 封诰 (FengGao): from the birth time branch;
//! - 三台 (SanTai) / 八座 (BaZuo): from the placed 左辅/右弼 and lunar day;
//! - 龙池 (LongChi) / 凤阁 (FengGe): from the birth year branch;
//! - 天哭 (TianKu) / 天虚 (TianXu): from the birth year branch;
//! - 恩光 (EnGuang) / 天贵 (TianGui): from the placed 文昌/文曲 and lunar day;
//! - 天巫 (TianWu) / 天月 (TianYueAdj) / 阴煞 (YinSha) / 解神 (JieShen): fixed
//!   per-lunar-month branch lookups.
//!
//! The remaining adjective stars, brightness for adjective stars, temporal
//! scopes, leap-month behavior, and rat-hour variants stay out of scope.

use crate::{
    chart::{Chart, Palace, StarPlacement},
    error::ChartError,
    ganzhi::EarthlyBranch,
    life_body::{LunarDay, LunarMonth},
    mutagen::Scope,
    star::{Brightness, StarMetadata, StarName},
};

/// Inputs required to place the supported adjective-star subset.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct AdjectiveStarPlacementInput {
    lunar_month: LunarMonth,
    lunar_day: LunarDay,
    birth_time: EarthlyBranch,
    birth_year_branch: EarthlyBranch,
}

impl AdjectiveStarPlacementInput {
    /// Creates adjective-star placement input from explicit lunar and ganzhi facts.
    pub const fn new(
        lunar_month: LunarMonth,
        lunar_day: LunarDay,
        birth_time: EarthlyBranch,
        birth_year_branch: EarthlyBranch,
    ) -> Self {
        Self {
            lunar_month,
            lunar_day,
            birth_time,
            birth_year_branch,
        }
    }

    /// Returns the validated lunar month.
    pub const fn lunar_month(self) -> LunarMonth {
        self.lunar_month
    }

    /// Returns the validated lunar day.
    pub const fn lunar_day(self) -> LunarDay {
        self.lunar_day
    }

    /// Returns the birth time branch.
    pub const fn birth_time(self) -> EarthlyBranch {
        self.birth_time
    }

    /// Returns the birth year Earthly Branch used for branch-based stars.
    pub const fn birth_year_branch(self) -> EarthlyBranch {
        self.birth_year_branch
    }
}

/// Places adjective stars into a chart.
///
/// Implementations must preserve chart invariants and return a valid chart.
pub trait AdjectiveStarPlacer {
    /// Places adjective stars in `chart` according to `input`.
    fn place_adjective_stars(
        &self,
        chart: Chart,
        input: AdjectiveStarPlacementInput,
    ) -> Result<Chart, ChartError>;
}

/// Returns factual metadata for the supported adjective-star subset.
pub const fn adjective_star_metadata_table() -> &'static [StarMetadata; 18] {
    crate::star::adjective_star_metadata_table()
}

/// Returns factual metadata for one supported adjective star.
pub fn adjective_star_metadata(star: StarName) -> &'static StarMetadata {
    crate::star::adjective_star_metadata(star)
}

/// Returns factual metadata for one adjective star, if it is represented.
pub fn try_adjective_star_metadata(star: StarName) -> Option<&'static StarMetadata> {
    crate::star::try_adjective_star_metadata(star)
}

/// Places the supported adjective-star subset deterministically.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct DeterministicAdjectiveStarPlacer;

impl AdjectiveStarPlacer for DeterministicAdjectiveStarPlacer {
    fn place_adjective_stars(
        &self,
        chart: Chart,
        input: AdjectiveStarPlacementInput,
    ) -> Result<Chart, ChartError> {
        let placements = adjective_star_placements(&chart, input)?;

        let palaces = chart
            .palaces()
            .iter()
            .map(|palace| {
                let mut stars = palace.stars().to_vec();
                for &(branch, name) in &placements {
                    if branch == palace.branch() {
                        let metadata = adjective_star_metadata(name);
                        // iztro has no brightness table and no 四化 for these
                        // adjective stars, so brightness is Unknown and the
                        // mutagen slot stays None.
                        stars.push(StarPlacement::new(
                            name,
                            metadata.kind(),
                            Brightness::Unknown,
                            None,
                            Scope::Natal,
                        ));
                    }
                }
                Palace::new(palace.name(), palace.branch(), palace.stem(), stars)
            })
            .collect();

        Chart::try_new(
            chart.birth_context().clone(),
            chart.method_profile().clone(),
            palaces,
            chart.body_palace_branch(),
            chart.five_element_bureau(),
        )
    }
}

/// Returns each supported adjective star paired with the branch it occupies.
///
/// Branch formulas reproduce iztro 2.5.8, translated from its 寅-based palace
/// index frame into direct branch offsets:
///
/// - 红鸾 counts backward from 卯 by the birth year branch index; 天喜 sits
///   opposite (+6);
/// - 天姚 counts forward from 丑, 天刑 forward from 酉, by the lunar month
///   offset (正月 = 0);
/// - 台辅 counts forward from 午, 封诰 forward from 寅, by the birth time index
///   (子时 = 0);
/// - 三台 counts forward from the placed 左辅 by the lunar day offset
///   (初一 = 0), 八座 counts backward from the placed 右弼 by the same offset;
/// - 龙池 counts forward from 辰, 凤阁 backward from 戌, 天哭 backward from 午,
///   and 天虚 forward from 午, all by the birth year branch index;
/// - 恩光 counts forward from the placed 文昌, 天贵 forward from the placed 文曲,
///   each by the lunar day offset minus one (iztro `getDailyStarIndex`:
///   `(changIndex + dayIndex) % 12 - 1`);
/// - 天巫, 天月, 阴煞, and 解神 are fixed per-lunar-month branch lookups
///   (iztro `getMonthlyStarIndex`), indexed by the zero-based month.
fn adjective_star_placements(
    chart: &Chart,
    input: AdjectiveStarPlacementInput,
) -> Result<[(EarthlyBranch, StarName); 18], ChartError> {
    // iztro `getMonthlyStarIndex` lookup tables, indexed by the zero-based lunar
    // month (正月 = 0). Each entry is the target Earthly Branch.
    const TIAN_WU_BY_MONTH: [EarthlyBranch; 4] = [
        EarthlyBranch::Si,
        EarthlyBranch::Shen,
        EarthlyBranch::Yin,
        EarthlyBranch::Hai,
    ];
    const TIAN_YUE_BY_MONTH: [EarthlyBranch; 12] = [
        EarthlyBranch::Xu,
        EarthlyBranch::Si,
        EarthlyBranch::Chen,
        EarthlyBranch::Yin,
        EarthlyBranch::Wei,
        EarthlyBranch::Mao,
        EarthlyBranch::Hai,
        EarthlyBranch::Wei,
        EarthlyBranch::Yin,
        EarthlyBranch::Wu,
        EarthlyBranch::Xu,
        EarthlyBranch::Yin,
    ];
    const YIN_SHA_BY_MONTH: [EarthlyBranch; 6] = [
        EarthlyBranch::Yin,
        EarthlyBranch::Zi,
        EarthlyBranch::Xu,
        EarthlyBranch::Shen,
        EarthlyBranch::Wu,
        EarthlyBranch::Chen,
    ];
    const JIE_SHEN_BY_HALF_MONTH: [EarthlyBranch; 6] = [
        EarthlyBranch::Shen,
        EarthlyBranch::Xu,
        EarthlyBranch::Zi,
        EarthlyBranch::Yin,
        EarthlyBranch::Chen,
        EarthlyBranch::Wu,
    ];

    let month_index = usize::from(input.lunar_month().value()) - 1;
    let month_offset = month_index as isize;
    let day_offset = isize::from(input.lunar_day().value()) - 1;
    let time_index = input.birth_time().index() as isize;
    let year_branch_index = input.birth_year_branch().index() as isize;
    let zuo_fu = branch_containing_required_star(chart, StarName::ZuoFu)?;
    let you_bi = branch_containing_required_star(chart, StarName::YouBi)?;
    let wen_chang = branch_containing_required_star(chart, StarName::WenChang)?;
    let wen_qu = branch_containing_required_star(chart, StarName::WenQu)?;

    let hong_luan = EarthlyBranch::Mao.offset(-year_branch_index);
    let tian_xi = hong_luan.offset(6);
    let tian_yao = EarthlyBranch::Chou.offset(month_offset);
    let tian_xing = EarthlyBranch::You.offset(month_offset);
    let tai_fu = EarthlyBranch::Wu.offset(time_index);
    let feng_gao = EarthlyBranch::Yin.offset(time_index);
    let san_tai = zuo_fu.offset(day_offset);
    let ba_zuo = you_bi.offset(-day_offset);
    let long_chi = EarthlyBranch::Chen.offset(year_branch_index);
    let feng_ge = EarthlyBranch::Xu.offset(-year_branch_index);
    let tian_ku = EarthlyBranch::Wu.offset(-year_branch_index);
    let tian_xu = EarthlyBranch::Wu.offset(year_branch_index);
    let en_guang = wen_chang.offset(day_offset - 1);
    let tian_gui = wen_qu.offset(day_offset - 1);
    let tian_wu = TIAN_WU_BY_MONTH[month_index % 4];
    let tian_yue = TIAN_YUE_BY_MONTH[month_index];
    let yin_sha = YIN_SHA_BY_MONTH[month_index % 6];
    let jie_shen = JIE_SHEN_BY_HALF_MONTH[month_index / 2];

    Ok([
        (hong_luan, StarName::HongLuan),
        (tian_xi, StarName::TianXi),
        (tian_yao, StarName::TianYao),
        (tian_xing, StarName::TianXing),
        (tai_fu, StarName::TaiFu),
        (feng_gao, StarName::FengGao),
        (san_tai, StarName::SanTai),
        (ba_zuo, StarName::BaZuo),
        (long_chi, StarName::LongChi),
        (feng_ge, StarName::FengGe),
        (tian_ku, StarName::TianKu),
        (tian_xu, StarName::TianXu),
        (en_guang, StarName::EnGuang),
        (tian_gui, StarName::TianGui),
        (tian_wu, StarName::TianWu),
        (tian_yue, StarName::TianYueAdj),
        (yin_sha, StarName::YinSha),
        (jie_shen, StarName::JieShen),
    ])
}

fn branch_containing_required_star(
    chart: &Chart,
    star: StarName,
) -> Result<EarthlyBranch, ChartError> {
    chart
        .palace_containing_star(star)
        .map(Palace::branch)
        .ok_or(ChartError::RequiredStarMissing { star })
}
