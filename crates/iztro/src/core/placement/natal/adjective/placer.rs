//! Adjective-star placement orchestration and the deterministic placer.

use crate::core::error::ChartError;
use crate::core::model::chart::{Chart, Palace, StarPlacement};
use crate::core::model::profile::ChartAlgorithmKind;
use crate::core::model::star::mutagen::Scope;
use crate::core::model::star::{Brightness, StarName};
use lunar_lite::EarthlyBranch;

use super::formulas::{
    gu_chen_gua_su_branches, hua_gai_branch, jie_lu_branch, kong_wang_branch, po_sui_branch,
    tian_chu_branch, tian_fu_adj_branch, tian_guan_branch, tian_shang_tian_shi_branches,
    xian_chi_branch, xun_kong_branch, zhongzhou_jie_kong_branch, zhongzhou_jie_sha_adj_branch,
};
use super::input::AdjectiveStarPlacementInput;
use super::metadata::adjective_star_metadata;
use super::tables::{
    DA_HAO_ADJ_BY_YEAR_BRANCH, FEI_LIAN_BY_YEAR_BRANCH, JIE_SHEN_BY_HALF_MONTH,
    NIAN_JIE_BY_YEAR_BRANCH, TIAN_WU_BY_MONTH, TIAN_YUE_BY_MONTH, YIN_SHA_BY_MONTH,
};

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
            chart.birth_year(),
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

fn branch_containing_required_star(
    chart: &Chart,
    star: StarName,
) -> Result<EarthlyBranch, ChartError> {
    chart
        .palace_containing_star(star)
        .map(Palace::branch)
        .ok_or(ChartError::RequiredStarMissing { star })
}
