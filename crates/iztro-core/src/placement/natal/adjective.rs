//! Deterministic adjective-star (杂曜) placement for the natal chart.
//!
//! Reproduces the full supported natal 杂曜 set of `iztro` 2.5.8
//! `adjectiveStars` (`getAdjectiveStar` plus the `getLuanXiIndex`,
//! `getMonthlyStarIndex`, `getTimelyStarIndex`, `getDailyStarIndex`,
//! `getHuagaiXianchiIndex`, and `getYearlyStarIndex` helpers in `src/star`,
//! MIT licensed). The default (non-Zhongzhou) algorithm places 38 natal-origin
//! stars. Zhongzhou keeps the common natal stars, replaces 截路/空亡 with
//! 龙德/截空/劫煞/大耗, and may swap 天伤/天使 by year-branch/gender polarity.
//! The common stars are grouped by placement basis:
//!
//! - 红鸾 (HongLuan) / 天喜 (TianXi): from the birth year branch;
//! - 天姚 (TianYao) / 天刑 (TianXing): from the lunar month;
//! - 台辅 (TaiFu) / 封诰 (FengGao): from the birth time branch;
//! - 三台 (SanTai) / 八座 (BaZuo): from the placed 左辅/右弼 and lunar day;
//! - 龙池 (LongChi) / 凤阁 (FengGe): from the birth year branch;
//! - 天哭 (TianKu) / 天虚 (TianXu): from the birth year branch;
//! - 恩光 (EnGuang) / 天贵 (TianGui): from the placed 文昌/文曲 and lunar day;
//! - 天巫 (TianWu) / 天月 (TianYueAdj) / 阴煞 (YinSha) / 解神 (JieShen): fixed
//!   per-lunar-month branch lookups;
//! - 华盖 (HuaGai) / 咸池 (XianChi) / 孤辰 (GuChen) / 寡宿 (GuaSu) /
//!   蜚廉 (FeiLian) / 破碎 (PoSui) / 天德 (TianDe) / 月德 (YueDe) /
//!   年解 (NianJie): from the birth year branch;
//! - 天空 (TianKong): one branch forward from the birth year branch;
//! - 天官 (TianGuan) / 天厨 (TianChu) / 天福 (TianFuAdj): from the birth year
//!   stem;
//! - 天才 (TianCai) / 天寿 (TianShou): Life/Body-palace anchored, counted by the
//!   birth year branch;
//! - 天伤 (TianShang) / 天使 (TianShi): Life-palace anchored (仆役/疾厄 under the
//!   default algorithm; no阴阳 swap);
//! - 截路 (JieLu) / 空亡 (KongWang): from the birth year stem;
//! - 旬空 (XunKong): the 旬中空亡 void branch whose阴阳 polarity matches the birth
//!   year branch.
//!
//! 神煞 beyond this supported natal slice, adjective-star brightness, temporal
//! scopes, horoscope placement, and leap-month behavior stay out of scope.
//! 四化 remain `mutagen: Option<Mutagen>` facts on placements, never independent stars.

use crate::error::ChartError;
use crate::model::calendar::{BirthTime, Gender};
use crate::model::chart::{Chart, Palace, StarPlacement};
use lunar_lite::{EarthlyBranch, HeavenlyStem};
use crate::model::profile::ChartAlgorithmKind;
use crate::model::star::mutagen::Scope;
use crate::model::star::{Brightness, StarMetadata, StarName};
use crate::placement::natal::life_body::{LunarDay, LunarMonth};

/// Inputs required to place the supported natal adjective-star set.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct AdjectiveStarPlacementInput {
    lunar_month: LunarMonth,
    lunar_day: LunarDay,
    daily_star_offset: u8,
    birth_time: BirthTime,
    birth_year_stem: HeavenlyStem,
    birth_year_branch: EarthlyBranch,
}

impl AdjectiveStarPlacementInput {
    /// Creates adjective-star placement input from explicit lunar and ganzhi facts.
    pub const fn new(
        lunar_month: LunarMonth,
        lunar_day: LunarDay,
        birth_time: EarthlyBranch,
        birth_year_stem: HeavenlyStem,
        birth_year_branch: EarthlyBranch,
    ) -> Self {
        Self::new_with_daily_star_offset(
            lunar_month,
            lunar_day,
            lunar_day.value() - 1,
            BirthTime::from_branch(birth_time),
            birth_year_stem,
            birth_year_branch,
        )
    }

    /// Creates adjective-star placement input with explicit daily-star offset.
    pub const fn new_with_daily_star_offset(
        lunar_month: LunarMonth,
        lunar_day: LunarDay,
        daily_star_offset: u8,
        birth_time: BirthTime,
        birth_year_stem: HeavenlyStem,
        birth_year_branch: EarthlyBranch,
    ) -> Self {
        Self {
            lunar_month,
            lunar_day,
            daily_star_offset,
            birth_time,
            birth_year_stem,
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

    /// Returns the day offset used by daily adjective-star formulas.
    pub const fn daily_star_offset(self) -> u8 {
        self.daily_star_offset
    }

    /// Returns the birth time branch.
    pub const fn birth_time(self) -> EarthlyBranch {
        self.birth_time.branch()
    }

    /// Returns the full birth-time variant.
    pub const fn birth_time_variant(self) -> BirthTime {
        self.birth_time
    }

    /// Returns the birth year Heavenly Stem used for stem-based stars.
    pub const fn birth_year_stem(self) -> HeavenlyStem {
        self.birth_year_stem
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
pub const fn adjective_star_metadata_table() -> &'static [StarMetadata] {
    crate::model::star::adjective_star_metadata_table()
}

/// Returns factual metadata for one supported adjective star.
pub fn adjective_star_metadata(star: StarName) -> &'static StarMetadata {
    crate::model::star::adjective_star_metadata(star)
}

/// Returns factual metadata for one adjective star, if it is represented.
pub fn try_adjective_star_metadata(star: StarName) -> Option<&'static StarMetadata> {
    crate::model::star::try_adjective_star_metadata(star)
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
/// - 华盖, 咸池, 孤辰, 寡宿, 蜚廉, 破碎, 天德, 月德, and 年解 reproduce the
///   birth-year-branch subset of iztro `getYearlyStarIndex`;
/// - 天空 sits one branch forward from the birth year branch;
/// - 天官, 天厨, and 天福 are fixed per-birth-year-stem branch lookups;
/// - 天才 counts forward from the Life Palace, 天寿 from the Body Palace, each
///   by the birth year branch index (子 = 0);
/// - 天伤 sits in the 仆役 palace and 天使 in the 疾厄 palace relative to the
///   Life Palace (the default-algorithm placement, with no阴阳 swap);
/// - 截路 and 空亡 are fixed per-birth-year-stem branch lookups;
/// - 旬空 is the 旬中空亡 void branch matching the birth year branch's阴阳
///   polarity (see [`xun_kong_branch`]).
fn adjective_star_placements(
    chart: &Chart,
    input: AdjectiveStarPlacementInput,
) -> Result<Vec<(EarthlyBranch, StarName)>, ChartError> {
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
    const FEI_LIAN_BY_YEAR_BRANCH: [EarthlyBranch; 12] = [
        EarthlyBranch::Shen,
        EarthlyBranch::You,
        EarthlyBranch::Xu,
        EarthlyBranch::Si,
        EarthlyBranch::Wu,
        EarthlyBranch::Wei,
        EarthlyBranch::Yin,
        EarthlyBranch::Mao,
        EarthlyBranch::Chen,
        EarthlyBranch::Hai,
        EarthlyBranch::Zi,
        EarthlyBranch::Chou,
    ];
    const NIAN_JIE_BY_YEAR_BRANCH: [EarthlyBranch; 12] = [
        EarthlyBranch::Xu,
        EarthlyBranch::You,
        EarthlyBranch::Shen,
        EarthlyBranch::Wei,
        EarthlyBranch::Wu,
        EarthlyBranch::Si,
        EarthlyBranch::Chen,
        EarthlyBranch::Mao,
        EarthlyBranch::Yin,
        EarthlyBranch::Chou,
        EarthlyBranch::Zi,
        EarthlyBranch::Hai,
    ];
    const DA_HAO_ADJ_BY_YEAR_BRANCH: [EarthlyBranch; 12] = [
        EarthlyBranch::Wei,
        EarthlyBranch::Wu,
        EarthlyBranch::You,
        EarthlyBranch::Shen,
        EarthlyBranch::Hai,
        EarthlyBranch::Xu,
        EarthlyBranch::Chou,
        EarthlyBranch::Zi,
        EarthlyBranch::Mao,
        EarthlyBranch::Yin,
        EarthlyBranch::Si,
        EarthlyBranch::Chen,
    ];

    let month_index = usize::from(input.lunar_month().value()) - 1;
    let month_offset = month_index as isize;
    let day_offset = isize::from(input.daily_star_offset());
    let time_index = isize::from(input.birth_time_variant().iztro_time_index() % 12);
    let year_stem = input.birth_year_stem();
    let year_branch = input.birth_year_branch();
    let year_branch_index = year_branch.index();
    let year_branch_offset = year_branch_index as isize;
    let life_branch = chart
        .life_palace()
        .map(Palace::branch)
        .ok_or(ChartError::RequiredLifeBodyPalaceMissing)?;
    let body_branch = chart
        .body_palace_branch()
        .ok_or(ChartError::RequiredLifeBodyPalaceMissing)?;
    let zuo_fu = branch_containing_required_star(chart, StarName::ZuoFu)?;
    let you_bi = branch_containing_required_star(chart, StarName::YouBi)?;
    let wen_chang = branch_containing_required_star(chart, StarName::WenChang)?;
    let wen_qu = branch_containing_required_star(chart, StarName::WenQu)?;

    let hong_luan = EarthlyBranch::Mao.offset(-year_branch_offset);
    let tian_xi = hong_luan.offset(6);
    let tian_yao = EarthlyBranch::Chou.offset(month_offset);
    let tian_xing = EarthlyBranch::You.offset(month_offset);
    let tai_fu = EarthlyBranch::Wu.offset(time_index);
    let feng_gao = EarthlyBranch::Yin.offset(time_index);
    let san_tai = zuo_fu.offset(day_offset);
    let ba_zuo = you_bi.offset(-day_offset);
    let long_chi = EarthlyBranch::Chen.offset(year_branch_offset);
    let feng_ge = EarthlyBranch::Xu.offset(-year_branch_offset);
    let tian_ku = EarthlyBranch::Wu.offset(-year_branch_offset);
    let tian_xu = EarthlyBranch::Wu.offset(year_branch_offset);
    let en_guang = wen_chang.offset(day_offset - 1);
    let tian_gui = wen_qu.offset(day_offset - 1);
    let tian_wu = TIAN_WU_BY_MONTH[month_index % 4];
    let tian_yue = TIAN_YUE_BY_MONTH[month_index];
    let yin_sha = YIN_SHA_BY_MONTH[month_index % 6];
    let jie_shen = JIE_SHEN_BY_HALF_MONTH[month_index / 2];
    let hua_gai = hua_gai_branch(year_branch);
    let (gu_chen, gua_su) = gu_chen_gua_su_branches(year_branch);
    let fei_lian = FEI_LIAN_BY_YEAR_BRANCH[year_branch_index];
    let po_sui = po_sui_branch(year_branch);
    let tian_de = EarthlyBranch::You.offset(year_branch_offset);
    let yue_de = EarthlyBranch::Si.offset(year_branch_offset);
    let nian_jie = NIAN_JIE_BY_YEAR_BRANCH[year_branch_index];
    // Birth-year-branch group continued.
    let xian_chi = xian_chi_branch(year_branch);
    let tian_kong = year_branch.offset(1);
    // Birth-year-stem group.
    let tian_guan = tian_guan_branch(year_stem);
    let tian_chu = tian_chu_branch(year_stem);
    let tian_fu_adj = tian_fu_adj_branch(year_stem);
    let jie_lu = jie_lu_branch(year_stem);
    let kong_wang = kong_wang_branch(year_stem);
    // Life/Body-palace anchored group. 天才/天寿 count forward from the Life /
    // Body palaces by the birth year branch index. 天伤/天使 occupy the 仆役
    // (Life + 5) and 疾厄 (Life + 7) palaces under the default algorithm, but
    // iztro `getTianshiTianshangIndex` swaps them for Zhongzhou when the birth
    // year branch polarity and gender polarity differ.
    let tian_cai = life_branch.offset(year_branch_offset);
    let tian_shou = body_branch.offset(year_branch_offset);
    let (tian_shang, tian_shi) = tian_shang_tian_shi_branches(
        chart.method_profile().algorithm_kind(),
        chart.birth_context().gender(),
        year_branch,
        life_branch,
    );
    // Void / 空亡 family.
    let xun_kong = xun_kong_branch(year_stem, year_branch);

    let mut placements = Vec::with_capacity(40);
    placements.extend([
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
        (hua_gai, StarName::HuaGai),
        (gu_chen, StarName::GuChen),
        (gua_su, StarName::GuaSu),
        (fei_lian, StarName::FeiLian),
        (po_sui, StarName::PoSui),
        (tian_de, StarName::TianDe),
        (yue_de, StarName::YueDe),
        (nian_jie, StarName::NianJie),
        (xian_chi, StarName::XianChi),
        (tian_kong, StarName::TianKong),
        (tian_guan, StarName::TianGuan),
        (tian_chu, StarName::TianChu),
        (tian_fu_adj, StarName::TianFuAdj),
        (tian_cai, StarName::TianCai),
        (tian_shou, StarName::TianShou),
        (tian_shang, StarName::TianShang),
        (tian_shi, StarName::TianShi),
        (xun_kong, StarName::XunKong),
    ]);

    match chart.method_profile().algorithm_kind() {
        ChartAlgorithmKind::Zhongzhou => {
            placements.extend([
                // iztro `getAdjectiveStar` reads 龙德 from Zhongzhou `getYearly12`
                // suiqian12: LongDe sits seven palaces forward from the year branch.
                (year_branch.offset(7), StarName::LongDeAdj),
                // iztro `getYearlyStarIndex` `jiekongIndex`: yang year branch
                // uses 截路's branch, yin year branch uses 空亡's branch.
                (
                    zhongzhou_jie_kong_branch(year_branch, jie_lu, kong_wang),
                    StarName::JieKong,
                ),
                (
                    zhongzhou_jie_sha_adj_branch(year_branch),
                    StarName::JieShaAdj,
                ),
                (
                    DA_HAO_ADJ_BY_YEAR_BRANCH[year_branch_index],
                    StarName::DaHaoAdj,
                ),
            ]);
        }
        ChartAlgorithmKind::QuanShu | ChartAlgorithmKind::Placeholder => {
            // Placeholder keeps the historical default path for backward
            // compatibility until callers opt into an explicit algorithm.
            placements.extend([(jie_lu, StarName::JieLu), (kong_wang, StarName::KongWang)]);
        }
    }

    Ok(placements)
}

fn hua_gai_branch(year_branch: EarthlyBranch) -> EarthlyBranch {
    match year_branch {
        EarthlyBranch::Shen | EarthlyBranch::Zi | EarthlyBranch::Chen => EarthlyBranch::Chen,
        EarthlyBranch::Si | EarthlyBranch::You | EarthlyBranch::Chou => EarthlyBranch::Chou,
        EarthlyBranch::Yin | EarthlyBranch::Wu | EarthlyBranch::Xu => EarthlyBranch::Xu,
        EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Wei => EarthlyBranch::Wei,
    }
}

fn gu_chen_gua_su_branches(year_branch: EarthlyBranch) -> (EarthlyBranch, EarthlyBranch) {
    match year_branch {
        EarthlyBranch::Yin | EarthlyBranch::Mao | EarthlyBranch::Chen => {
            (EarthlyBranch::Si, EarthlyBranch::Chou)
        }
        EarthlyBranch::Si | EarthlyBranch::Wu | EarthlyBranch::Wei => {
            (EarthlyBranch::Shen, EarthlyBranch::Chen)
        }
        EarthlyBranch::Shen | EarthlyBranch::You | EarthlyBranch::Xu => {
            (EarthlyBranch::Hai, EarthlyBranch::Wei)
        }
        EarthlyBranch::Hai | EarthlyBranch::Zi | EarthlyBranch::Chou => {
            (EarthlyBranch::Yin, EarthlyBranch::Xu)
        }
    }
}

fn po_sui_branch(year_branch: EarthlyBranch) -> EarthlyBranch {
    match year_branch {
        EarthlyBranch::Zi | EarthlyBranch::Wu | EarthlyBranch::Mao | EarthlyBranch::You => {
            EarthlyBranch::Si
        }
        EarthlyBranch::Yin | EarthlyBranch::Shen | EarthlyBranch::Si | EarthlyBranch::Hai => {
            EarthlyBranch::You
        }
        EarthlyBranch::Chen | EarthlyBranch::Xu | EarthlyBranch::Chou | EarthlyBranch::Wei => {
            EarthlyBranch::Chou
        }
    }
}

/// 咸池 (iztro `getHuagaiXianchiIndex`): the peach-blossom branch of the birth
/// year branch's 三合 (triad) family — the opposite member from 华盖.
fn xian_chi_branch(year_branch: EarthlyBranch) -> EarthlyBranch {
    match year_branch {
        EarthlyBranch::Yin | EarthlyBranch::Wu | EarthlyBranch::Xu => EarthlyBranch::Mao,
        EarthlyBranch::Shen | EarthlyBranch::Zi | EarthlyBranch::Chen => EarthlyBranch::You,
        EarthlyBranch::Si | EarthlyBranch::You | EarthlyBranch::Chou => EarthlyBranch::Wu,
        EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Wei => EarthlyBranch::Zi,
    }
}

/// 天官 (iztro `getYearlyStarIndex`): a fixed branch per birth year stem.
fn tian_guan_branch(year_stem: HeavenlyStem) -> EarthlyBranch {
    match year_stem {
        HeavenlyStem::Jia => EarthlyBranch::Wei,
        HeavenlyStem::Yi => EarthlyBranch::Chen,
        HeavenlyStem::Bing => EarthlyBranch::Si,
        HeavenlyStem::Ding => EarthlyBranch::Yin,
        HeavenlyStem::Wu => EarthlyBranch::Mao,
        HeavenlyStem::Ji => EarthlyBranch::You,
        HeavenlyStem::Geng => EarthlyBranch::Hai,
        HeavenlyStem::Xin => EarthlyBranch::You,
        HeavenlyStem::Ren => EarthlyBranch::Xu,
        HeavenlyStem::Gui => EarthlyBranch::Wu,
    }
}

/// 天厨 (iztro `getYearlyStarIndex`): a fixed branch per birth year stem.
fn tian_chu_branch(year_stem: HeavenlyStem) -> EarthlyBranch {
    match year_stem {
        HeavenlyStem::Jia => EarthlyBranch::Si,
        HeavenlyStem::Yi => EarthlyBranch::Wu,
        HeavenlyStem::Bing => EarthlyBranch::Zi,
        HeavenlyStem::Ding => EarthlyBranch::Si,
        HeavenlyStem::Wu => EarthlyBranch::Wu,
        HeavenlyStem::Ji => EarthlyBranch::Shen,
        HeavenlyStem::Geng => EarthlyBranch::Yin,
        HeavenlyStem::Xin => EarthlyBranch::Wu,
        HeavenlyStem::Ren => EarthlyBranch::You,
        HeavenlyStem::Gui => EarthlyBranch::Hai,
    }
}

/// 天福 adjective star (iztro `getYearlyStarIndex` `tianfuIndex`): a fixed
/// branch per birth year stem. Distinct from the major star 天府.
fn tian_fu_adj_branch(year_stem: HeavenlyStem) -> EarthlyBranch {
    match year_stem {
        HeavenlyStem::Jia => EarthlyBranch::You,
        HeavenlyStem::Yi => EarthlyBranch::Shen,
        HeavenlyStem::Bing => EarthlyBranch::Zi,
        HeavenlyStem::Ding => EarthlyBranch::Hai,
        HeavenlyStem::Wu => EarthlyBranch::Mao,
        HeavenlyStem::Ji => EarthlyBranch::Yin,
        HeavenlyStem::Geng => EarthlyBranch::Wu,
        HeavenlyStem::Xin => EarthlyBranch::Si,
        HeavenlyStem::Ren => EarthlyBranch::Wu,
        HeavenlyStem::Gui => EarthlyBranch::Si,
    }
}

/// 截路 (iztro `getYearlyStarIndex`): a fixed branch per 五鼠遁-style stem pair
/// (stem index mod 5).
fn jie_lu_branch(year_stem: HeavenlyStem) -> EarthlyBranch {
    match year_stem {
        HeavenlyStem::Jia | HeavenlyStem::Ji => EarthlyBranch::Shen,
        HeavenlyStem::Yi | HeavenlyStem::Geng => EarthlyBranch::Wu,
        HeavenlyStem::Bing | HeavenlyStem::Xin => EarthlyBranch::Chen,
        HeavenlyStem::Ding | HeavenlyStem::Ren => EarthlyBranch::Yin,
        HeavenlyStem::Wu | HeavenlyStem::Gui => EarthlyBranch::Zi,
    }
}

/// 空亡 (iztro `getYearlyStarIndex`): the branch one step forward from 截路,
/// also fixed per stem pair (stem index mod 5).
fn kong_wang_branch(year_stem: HeavenlyStem) -> EarthlyBranch {
    match year_stem {
        HeavenlyStem::Jia | HeavenlyStem::Ji => EarthlyBranch::You,
        HeavenlyStem::Yi | HeavenlyStem::Geng => EarthlyBranch::Wei,
        HeavenlyStem::Bing | HeavenlyStem::Xin => EarthlyBranch::Si,
        HeavenlyStem::Ding | HeavenlyStem::Ren => EarthlyBranch::Mao,
        HeavenlyStem::Wu | HeavenlyStem::Gui => EarthlyBranch::Chou,
    }
}

fn zhongzhou_jie_kong_branch(
    year_branch: EarthlyBranch,
    jie_lu: EarthlyBranch,
    kong_wang: EarthlyBranch,
) -> EarthlyBranch {
    if year_branch.index() % 2 == 0 {
        jie_lu
    } else {
        kong_wang
    }
}

fn zhongzhou_jie_sha_adj_branch(year_branch: EarthlyBranch) -> EarthlyBranch {
    match year_branch {
        EarthlyBranch::Shen | EarthlyBranch::Zi | EarthlyBranch::Chen => EarthlyBranch::Si,
        EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Wei => EarthlyBranch::Shen,
        EarthlyBranch::Yin | EarthlyBranch::Wu | EarthlyBranch::Xu => EarthlyBranch::Hai,
        EarthlyBranch::Si | EarthlyBranch::You | EarthlyBranch::Chou => EarthlyBranch::Yin,
    }
}

fn tian_shang_tian_shi_branches(
    algorithm: ChartAlgorithmKind,
    gender: Gender,
    year_branch: EarthlyBranch,
    life_branch: EarthlyBranch,
) -> (EarthlyBranch, EarthlyBranch) {
    let default_tian_shang = life_branch.offset(5);
    let default_tian_shi = life_branch.offset(7);
    let same_yinyang = year_branch.index() % 2
        == match gender {
            Gender::Male => 0,
            Gender::Female => 1,
        };

    if algorithm == ChartAlgorithmKind::Zhongzhou && !same_yinyang {
        (default_tian_shi, default_tian_shang)
    } else {
        (default_tian_shang, default_tian_shi)
    }
}

/// 旬空 (旬中空亡, iztro `getYearlyStarIndex` `xunkongIndex`).
///
/// The birth year stem-branch pair sits inside one of the six 甲-旬 (sexagenary
/// decades); two branches are absent from that decade and form the 旬空 pair.
/// iztro picks the void branch whose阴阳 polarity matches the birth year branch
/// (yang year branch → yang void branch, yin → yin).
///
/// iztro computes a base palace index `year_branch_palace + 癸 - stem + 1`, then
/// advances one palace when the base palace parity differs from the year
/// branch's. Palace indices and branch indices differ by the fixed offset of 寅
/// (2), so parity is preserved and the rule translates directly to branch
/// space: `base = year_branch_index + 10 - stem_index (mod 12)`.
fn xun_kong_branch(year_stem: HeavenlyStem, year_branch: EarthlyBranch) -> EarthlyBranch {
    let stem = year_stem.index() as isize;
    let year_branch_index = year_branch.index() as isize;
    // 癸 is stem index 9, and the iztro formula adds a further +1.
    let mut index = (year_branch_index + 10 - stem).rem_euclid(12);
    let year_polarity = year_branch_index.rem_euclid(2);
    if year_polarity != index.rem_euclid(2) {
        index = (index + 1).rem_euclid(12);
    }
    EarthlyBranch::from_index(index as usize)
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

#[cfg(test)]
mod tests {
    use super::{
        jie_lu_branch, kong_wang_branch, tian_chu_branch, tian_fu_adj_branch, tian_guan_branch,
        xian_chi_branch, xun_kong_branch,
    };
    use lunar_lite::{EARTHLY_BRANCHES, EarthlyBranch, HEAVENLY_STEMS, HeavenlyStem};

    // Tables transcribed from iztro 2.5.8 `getYearlyStarIndex` /
    // `getHuagaiXianchiIndex` and cross-checked against `astro.byLunar` output.

    /// 咸池, by birth year branch (子..亥). Same 三合 family as 华盖.
    #[test]
    fn xian_chi_matches_iztro_branch_table() {
        use EarthlyBranch::*;
        let expected = [You, Wu, Mao, Zi, You, Wu, Mao, Zi, You, Wu, Mao, Zi];
        for (index, branch) in EARTHLY_BRANCHES.into_iter().enumerate() {
            assert_eq!(
                xian_chi_branch(branch),
                expected[index],
                "咸池 for {branch:?}"
            );
        }
    }

    /// 天空 sits one branch forward from the birth year branch.
    #[test]
    fn tian_kong_is_one_branch_after_year_branch() {
        for branch in EARTHLY_BRANCHES {
            assert_eq!(
                branch.offset(1),
                EarthlyBranch::from_index(branch.index() + 1)
            );
        }
    }

    /// 天官, by birth year stem (甲..癸).
    #[test]
    fn tian_guan_matches_iztro_stem_table() {
        use EarthlyBranch::*;
        let expected = [Wei, Chen, Si, Yin, Mao, You, Hai, You, Xu, Wu];
        for (index, stem) in HEAVENLY_STEMS.into_iter().enumerate() {
            assert_eq!(tian_guan_branch(stem), expected[index], "天官 for {stem:?}");
        }
    }

    /// 天厨, by birth year stem (甲..癸).
    #[test]
    fn tian_chu_matches_iztro_stem_table() {
        use EarthlyBranch::*;
        let expected = [Si, Wu, Zi, Si, Wu, Shen, Yin, Wu, You, Hai];
        for (index, stem) in HEAVENLY_STEMS.into_iter().enumerate() {
            assert_eq!(tian_chu_branch(stem), expected[index], "天厨 for {stem:?}");
        }
    }

    /// 天福 adjective star, by birth year stem (甲..癸).
    #[test]
    fn tian_fu_adj_matches_iztro_stem_table() {
        use EarthlyBranch::*;
        let expected = [You, Shen, Zi, Hai, Mao, Yin, Wu, Si, Wu, Si];
        for (index, stem) in HEAVENLY_STEMS.into_iter().enumerate() {
            assert_eq!(
                tian_fu_adj_branch(stem),
                expected[index],
                "天福 for {stem:?}"
            );
        }
    }

    /// 截路 / 空亡, by birth year stem pair (stem index mod 5). 空亡 is one
    /// branch forward from 截路.
    #[test]
    fn jie_lu_and_kong_wang_match_iztro_stem_table() {
        use EarthlyBranch::*;
        let jie_lu = [Shen, Wu, Chen, Yin, Zi, Shen, Wu, Chen, Yin, Zi];
        let kong_wang = [You, Wei, Si, Mao, Chou, You, Wei, Si, Mao, Chou];
        for (index, stem) in HEAVENLY_STEMS.into_iter().enumerate() {
            assert_eq!(jie_lu_branch(stem), jie_lu[index], "截路 for {stem:?}");
            assert_eq!(
                kong_wang_branch(stem),
                kong_wang[index],
                "空亡 for {stem:?}"
            );
            assert_eq!(
                kong_wang_branch(stem),
                jie_lu_branch(stem).offset(1),
                "空亡 should follow 截路 for {stem:?}"
            );
        }
    }

    /// 旬空 (旬中空亡) across the full sexagenary cycle: the void branch of the
    /// year's 甲-旬 whose 阴阳 polarity matches the year branch.
    #[test]
    fn xun_kong_matches_iztro_over_full_sexagenary_cycle() {
        use EarthlyBranch::*;
        // i = sexagenary position; stem = i % 10, branch = i % 12.
        let expected = [
            Xu, Hai, Xu, Hai, Xu, Hai, Xu, Hai, Xu, Hai, // 甲子旬: void 戌亥
            Shen, You, Shen, You, Shen, You, Shen, You, Shen, You, // 甲戌旬: void 申酉
            Wu, Wei, Wu, Wei, Wu, Wei, Wu, Wei, Wu, Wei, // 甲申旬: void 午未
            Chen, Si, Chen, Si, Chen, Si, Chen, Si, Chen, Si, // 甲午旬: void 辰巳
            Yin, Mao, Yin, Mao, Yin, Mao, Yin, Mao, Yin, Mao, // 甲辰旬: void 寅卯
            Zi, Chou, Zi, Chou, Zi, Chou, Zi, Chou, Zi, Chou, // 甲寅旬: void 子丑
        ];
        for (i, &want) in expected.iter().enumerate() {
            let stem = HeavenlyStem::from_index(i % 10);
            let branch = EarthlyBranch::from_index(i % 12);
            assert_eq!(
                xun_kong_branch(stem, branch),
                want,
                "旬空 for sexagenary position {i} ({stem:?}{branch:?})"
            );
        }
    }

    /// The 旬空 result always shares the birth year branch's 阴阳 polarity.
    #[test]
    fn xun_kong_polarity_matches_year_branch() {
        for i in 0..60usize {
            let stem = HeavenlyStem::from_index(i % 10);
            let branch = EarthlyBranch::from_index(i % 12);
            let void = xun_kong_branch(stem, branch);
            assert_eq!(
                void.index() % 2,
                branch.index() % 2,
                "旬空 polarity mismatch for {stem:?}{branch:?}"
            );
        }
    }
}
