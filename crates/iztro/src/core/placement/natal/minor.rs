//! Deterministic fourteen-minor-star (辅星) placement for the natal chart.
//!
//! Star positions reproduce `iztro` 2.5.8 (`getMinorStar` and helper location
//! functions in `src/star`, MIT licensed). Only natal placement, iztro kind,
//! supported brightness tables, and represented birth-year mutagens are
//! implemented here.

use crate::core::error::ChartError;
use crate::core::model::calendar::BirthTime;
use crate::core::model::chart::{Chart, Palace, StarPlacement};
use crate::core::model::ganzhi::{EarthlyBranch, HeavenlyStem};
use crate::core::model::star::mutagen::{Scope, birth_year_star_mutagen};
use crate::core::model::star::{Brightness, StarMetadata, StarName};
use crate::core::placement::location::{kui_yue_branches, lu_yang_tuo_ma_branches};
use crate::core::placement::natal::life_body::LunarMonth;

/// Inputs required to place the supported fourteen minor stars.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct MinorStarPlacementInput {
    lunar_month: LunarMonth,
    birth_time: BirthTime,
    birth_year_stem: HeavenlyStem,
    birth_year_branch: EarthlyBranch,
}

impl MinorStarPlacementInput {
    /// Creates minor-star placement input from explicit lunar and ganzhi facts.
    pub const fn new(
        lunar_month: LunarMonth,
        birth_time: EarthlyBranch,
        birth_year_stem: HeavenlyStem,
        birth_year_branch: EarthlyBranch,
    ) -> Self {
        Self::new_with_birth_time_variant(
            lunar_month,
            BirthTime::from_branch(birth_time),
            birth_year_stem,
            birth_year_branch,
        )
    }

    /// Creates minor-star placement input from the full birth-time variant.
    pub const fn new_with_birth_time_variant(
        lunar_month: LunarMonth,
        birth_time: BirthTime,
        birth_year_stem: HeavenlyStem,
        birth_year_branch: EarthlyBranch,
    ) -> Self {
        Self {
            lunar_month,
            birth_time,
            birth_year_stem,
            birth_year_branch,
        }
    }

    /// Returns the validated lunar month.
    pub const fn lunar_month(self) -> LunarMonth {
        self.lunar_month
    }

    /// Returns the birth time branch.
    pub const fn birth_time(self) -> EarthlyBranch {
        self.birth_time.branch()
    }

    /// Returns the full birth-time variant.
    pub const fn birth_time_variant(self) -> BirthTime {
        self.birth_time
    }

    /// Returns the birth year Heavenly Stem used for natal mutagens.
    pub const fn birth_year_stem(self) -> HeavenlyStem {
        self.birth_year_stem
    }

    /// Returns the birth year Earthly Branch used for branch-based minor stars.
    pub const fn birth_year_branch(self) -> EarthlyBranch {
        self.birth_year_branch
    }
}

/// Places minor stars into a chart.
///
/// Implementations must preserve chart invariants and return a valid chart.
pub trait MinorStarPlacer {
    /// Places minor stars in `chart` according to `input`.
    fn place_minor_stars(
        &self,
        chart: Chart,
        input: MinorStarPlacementInput,
    ) -> Result<Chart, ChartError>;
}

/// Returns factual metadata for the supported fourteen minor stars.
pub const fn minor_star_metadata_table() -> &'static [StarMetadata; 14] {
    crate::core::model::star::minor_star_metadata_table()
}

/// Returns factual metadata for one supported minor star.
pub fn minor_star_metadata(star: StarName) -> &'static StarMetadata {
    crate::core::model::star::minor_star_metadata(star)
}

/// Returns factual metadata for one minor star, if it is a represented minor star.
pub fn try_minor_star_metadata(star: StarName) -> Option<&'static StarMetadata> {
    crate::core::model::star::try_minor_star_metadata(star)
}

/// Returns a minor star's brightness for a branch.
///
/// iztro 2.5.8 `STARS_INFO` only has brightness tables for 文昌, 文曲, 火星,
/// 铃星, 擎羊, and 陀罗; the tables below reproduce those values verbatim. None
/// of the minor-star tables use 不 ([`Brightness::Weak`]) — upstream reserves 不
/// for major stars — so it never appears here. The other eight represented
/// minor stars (左辅, 右弼, 天魁, 天钺, 禄存, 天马, 地空, 地劫) have no upstream
/// table and return [`Brightness::Unknown`].
pub fn minor_star_brightness(star: StarName, branch: EarthlyBranch) -> Brightness {
    const MIAO: Brightness = Brightness::Temple;
    const WANG: Brightness = Brightness::Prosperous;
    const DE: Brightness = Brightness::Advantage;
    const LI: Brightness = Brightness::Favourable;
    const PING: Brightness = Brightness::Flat;
    const XIAN: Brightness = Brightness::Trapped;
    const UNKNOWN: Brightness = Brightness::Unknown;

    let brightness_by_yin_order = match star {
        StarName::WenChang => [XIAN, LI, DE, MIAO, XIAN, LI, DE, MIAO, XIAN, LI, DE, MIAO],
        StarName::WenQu => [
            PING, WANG, DE, MIAO, XIAN, WANG, DE, MIAO, XIAN, WANG, DE, MIAO,
        ],
        StarName::HuoXing | StarName::LingXing => {
            [MIAO, LI, XIAN, DE, MIAO, LI, XIAN, DE, MIAO, LI, XIAN, DE]
        }
        StarName::QingYang => [
            UNKNOWN, XIAN, MIAO, UNKNOWN, XIAN, MIAO, UNKNOWN, XIAN, MIAO, UNKNOWN, XIAN, MIAO,
        ],
        StarName::TuoLuo => [
            XIAN, UNKNOWN, MIAO, XIAN, UNKNOWN, MIAO, XIAN, UNKNOWN, MIAO, XIAN, UNKNOWN, MIAO,
        ],
        _ => return Brightness::Unknown,
    };
    let yin_order_index = (branch.index() + 12 - EarthlyBranch::Yin.index()) % 12;

    brightness_by_yin_order[yin_order_index]
}

/// Places the supported fourteen minor stars deterministically.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct DeterministicMinorStarPlacer;

impl MinorStarPlacer for DeterministicMinorStarPlacer {
    fn place_minor_stars(
        &self,
        chart: Chart,
        input: MinorStarPlacementInput,
    ) -> Result<Chart, ChartError> {
        let placements = minor_star_placements(input);

        let palaces = chart
            .palaces()
            .iter()
            .map(|palace| {
                let mut stars = palace.stars().to_vec();
                for &(branch, name) in &placements {
                    if branch == palace.branch() {
                        let metadata = minor_star_metadata(name);
                        stars.push(StarPlacement::new(
                            name,
                            metadata.kind(),
                            minor_star_brightness(name, branch),
                            birth_year_star_mutagen(input.birth_year_stem(), name),
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

fn minor_star_placements(input: MinorStarPlacementInput) -> [(EarthlyBranch, StarName); 14] {
    let (zuo, you) = zuo_you_branches(input.lunar_month());
    let (chang, qu) = chang_qu_branches(input.birth_time_variant());
    let (kui, yue) = kui_yue_branches(input.birth_year_stem());
    let (lu, yang, tuo, ma) =
        lu_yang_tuo_ma_branches(input.birth_year_stem(), input.birth_year_branch());
    let (kong, jie) = kong_jie_branches(input.birth_time_variant());
    let (huo, ling) = huo_ling_branches(input.birth_year_branch(), input.birth_time_variant());

    [
        (zuo, StarName::ZuoFu),
        (you, StarName::YouBi),
        (chang, StarName::WenChang),
        (qu, StarName::WenQu),
        (kui, StarName::TianKui),
        (yue, StarName::TianYue),
        (lu, StarName::LuCun),
        (ma, StarName::TianMa),
        (kong, StarName::DiKong),
        (jie, StarName::DiJie),
        (huo, StarName::HuoXing),
        (ling, StarName::LingXing),
        (yang, StarName::QingYang),
        (tuo, StarName::TuoLuo),
    ]
}

fn zuo_you_branches(lunar_month: LunarMonth) -> (EarthlyBranch, EarthlyBranch) {
    let month_offset = isize::from(lunar_month.value()) - 1;

    (
        EarthlyBranch::Chen.offset(month_offset),
        EarthlyBranch::Xu.offset(-month_offset),
    )
}

fn chang_qu_branches(birth_time: BirthTime) -> (EarthlyBranch, EarthlyBranch) {
    let time_index = birth_time_index(birth_time);

    (
        EarthlyBranch::Xu.offset(-time_index),
        EarthlyBranch::Chen.offset(time_index),
    )
}

fn kong_jie_branches(birth_time: BirthTime) -> (EarthlyBranch, EarthlyBranch) {
    let time_index = birth_time_index(birth_time);

    (
        EarthlyBranch::Hai.offset(-time_index),
        EarthlyBranch::Hai.offset(time_index),
    )
}

fn huo_ling_branches(
    year_branch: EarthlyBranch,
    birth_time: BirthTime,
) -> (EarthlyBranch, EarthlyBranch) {
    let time_index = birth_time_index(birth_time);
    let (huo_start, ling_start) = match year_branch {
        EarthlyBranch::Yin | EarthlyBranch::Wu | EarthlyBranch::Xu => {
            (EarthlyBranch::Chou, EarthlyBranch::Mao)
        }
        EarthlyBranch::Shen | EarthlyBranch::Zi | EarthlyBranch::Chen => {
            (EarthlyBranch::Yin, EarthlyBranch::Xu)
        }
        EarthlyBranch::Si | EarthlyBranch::You | EarthlyBranch::Chou => {
            (EarthlyBranch::Mao, EarthlyBranch::Xu)
        }
        EarthlyBranch::Hai | EarthlyBranch::Wei | EarthlyBranch::Mao => {
            (EarthlyBranch::You, EarthlyBranch::Xu)
        }
    };

    (huo_start.offset(time_index), ling_start.offset(time_index))
}

fn birth_time_index(birth_time: BirthTime) -> isize {
    isize::from(birth_time.iztro_time_index() % 12)
}
