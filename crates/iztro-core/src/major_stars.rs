//! Deterministic fourteen-major-star (主星) placement for the natal chart.
//!
//! Star positions reproduce `iztro` 2.5.8 (`getStartIndex` and `getMajorStar`
//! in `src/star/location.ts` and `src/star/majorStar.ts`, MIT licensed). Only
//! placement, brightness, and supported birth-year mutagens are implemented
//! here. Scopes beyond the natal chart stay out of scope.

use crate::{
    bureau::FiveElementBureau,
    chart::{Chart, Palace, StarPlacement},
    error::ChartError,
    ganzhi::{EarthlyBranch, HeavenlyStem},
    life_body::LunarDay,
    mutagen::Scope,
    star::{Brightness, StarKind, StarMetadata, StarName},
};

pub use crate::mutagen::birth_year_major_star_mutagen;

/// Inputs required to place the fourteen major stars.
///
/// The lunar day selects the 紫微 (Zi Wei) position relative to the
/// five-element bureau; every other major star derives from 紫微 and 天府.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct MajorStarPlacementInput {
    lunar_day: LunarDay,
    five_element_bureau: FiveElementBureau,
    birth_year_stem: HeavenlyStem,
}

impl MajorStarPlacementInput {
    /// Creates major-star placement input from the lunar day, bureau, and birth year stem.
    pub const fn new(
        lunar_day: LunarDay,
        five_element_bureau: FiveElementBureau,
        birth_year_stem: HeavenlyStem,
    ) -> Self {
        Self {
            lunar_day,
            five_element_bureau,
            birth_year_stem,
        }
    }

    /// Returns the validated lunar day.
    pub const fn lunar_day(self) -> LunarDay {
        self.lunar_day
    }

    /// Returns the five-element bureau used to position 紫微.
    pub const fn five_element_bureau(self) -> FiveElementBureau {
        self.five_element_bureau
    }

    /// Returns the birth year Heavenly Stem used for natal mutagens.
    pub const fn birth_year_stem(self) -> HeavenlyStem {
        self.birth_year_stem
    }
}

/// Places major stars into a chart.
///
/// Implementations must preserve chart invariants and return a valid chart.
pub trait MajorStarPlacer {
    /// Places major stars in `chart` according to `input`.
    fn place_major_stars(
        &self,
        chart: Chart,
        input: MajorStarPlacementInput,
    ) -> Result<Chart, ChartError>;
}

/// The 紫微 series and each star's branch offset from 紫微 (counts backward).
const ZI_WEI_SERIES: [(StarName, isize); 6] = [
    (StarName::ZiWei, 0),
    (StarName::TianJi, -1),
    (StarName::TaiYang, -3),
    (StarName::WuQu, -4),
    (StarName::TianTong, -5),
    (StarName::LianZhen, -8),
];

/// The 天府 series and each star's branch offset from 天府 (counts forward).
const TIAN_FU_SERIES: [(StarName, isize); 8] = [
    (StarName::TianFu, 0),
    (StarName::TaiYin, 1),
    (StarName::TanLang, 2),
    (StarName::JuMen, 3),
    (StarName::TianXiang, 4),
    (StarName::TianLiang, 5),
    (StarName::QiSha, 6),
    (StarName::PoJun, 10),
];

/// Returns factual metadata for the fourteen major stars.
pub const fn major_star_metadata_table() -> &'static [StarMetadata; 14] {
    crate::star::major_star_metadata_table()
}

/// Returns factual metadata for one major star.
pub fn major_star_metadata(star: StarName) -> &'static StarMetadata {
    crate::star::major_star_metadata(star)
}

/// Returns factual metadata for one major star, if it is a represented major star.
pub fn try_major_star_metadata(star: StarName) -> Option<&'static StarMetadata> {
    crate::star::try_major_star_metadata(star)
}

/// Returns a major star's brightness for a branch.
///
/// The table mirrors iztro 2.5.8 `STARS_INFO` brightness values. iztro stores
/// brightness in palace order from 寅 through 丑, while [`EarthlyBranch::index`]
/// is ordered from 子 through 亥, so the branch index is converted explicitly.
pub fn major_star_brightness(star: StarName, branch: EarthlyBranch) -> Brightness {
    const MIAO: Brightness = Brightness::Temple;
    const WANG: Brightness = Brightness::Prosperous;
    const DE: Brightness = Brightness::Advantage;
    const LI: Brightness = Brightness::Favourable;
    const PING: Brightness = Brightness::Flat;
    const BU: Brightness = Brightness::Weak;
    const XIAN: Brightness = Brightness::Trapped;

    let brightness_by_yin_order = match star {
        StarName::ZiWei => [
            WANG, WANG, DE, WANG, MIAO, MIAO, WANG, WANG, DE, WANG, PING, MIAO,
        ],
        StarName::TianJi => [
            DE, WANG, LI, PING, MIAO, XIAN, DE, WANG, LI, PING, MIAO, XIAN,
        ],
        StarName::TaiYang => [
            WANG, MIAO, WANG, WANG, WANG, DE, DE, XIAN, BU, XIAN, XIAN, BU,
        ],
        StarName::WuQu => [
            DE, LI, MIAO, PING, WANG, MIAO, DE, LI, MIAO, PING, WANG, MIAO,
        ],
        StarName::TianTong => [
            LI, PING, PING, MIAO, XIAN, BU, WANG, PING, PING, MIAO, WANG, BU,
        ],
        StarName::LianZhen => [
            MIAO, PING, LI, XIAN, PING, LI, MIAO, PING, LI, XIAN, PING, LI,
        ],
        StarName::TianFu => [
            MIAO, DE, MIAO, DE, WANG, MIAO, DE, WANG, MIAO, DE, MIAO, MIAO,
        ],
        StarName::TaiYin => [
            WANG, XIAN, XIAN, XIAN, BU, BU, LI, BU, WANG, MIAO, MIAO, MIAO,
        ],
        StarName::TanLang => [
            PING, LI, MIAO, XIAN, WANG, MIAO, PING, LI, MIAO, XIAN, WANG, MIAO,
        ],
        StarName::JuMen => [
            MIAO, MIAO, XIAN, WANG, WANG, BU, MIAO, MIAO, XIAN, WANG, WANG, BU,
        ],
        StarName::TianXiang => [MIAO, XIAN, DE, DE, MIAO, DE, MIAO, XIAN, DE, DE, MIAO, MIAO],
        StarName::TianLiang => [
            MIAO, MIAO, MIAO, XIAN, MIAO, WANG, XIAN, DE, MIAO, XIAN, MIAO, WANG,
        ],
        StarName::QiSha => [
            MIAO, WANG, MIAO, PING, WANG, MIAO, MIAO, MIAO, MIAO, PING, WANG, MIAO,
        ],
        StarName::PoJun => [
            DE, XIAN, WANG, PING, MIAO, WANG, DE, XIAN, WANG, PING, MIAO, WANG,
        ],
        _ => return Brightness::Unknown,
    };
    let yin_order_index = (branch.index() + 12 - EarthlyBranch::Yin.index()) % 12;

    brightness_by_yin_order[yin_order_index]
}

/// Returns the branch holding 紫微 (Zi Wei) for a bureau and lunar day.
///
/// Implements iztro's 起紫微星诀: divide the lunar day by the bureau number,
/// borrowing up to the next multiple; the quotient counts forward from 寅 and
/// the borrowed amount adjusts forward when even and backward when odd.
pub fn zi_wei_branch(bureau: FiveElementBureau, day: LunarDay) -> EarthlyBranch {
    let bureau = i32::from(bureau.number());
    let day = i32::from(day.value());

    // Smallest non-negative amount to reach a multiple of the bureau number.
    let borrow = (bureau - day % bureau) % bureau;
    let quotient = ((day + borrow) / bureau) % 12;
    let base = quotient - 1;
    // 偶加奇减: an even borrow counts forward, an odd borrow counts backward.
    let steps = if borrow % 2 == 0 {
        base + borrow
    } else {
        base - borrow
    };

    // iztro indexes palaces from 寅 (Yin); map that offset back to a branch.
    EarthlyBranch::Yin.offset(steps as isize)
}

/// Returns the branch holding 天府 (Tian Fu) given the 紫微 branch.
///
/// 天府 sits opposite 紫微, reflected across the 寅–申 axis
/// (`tianfuIndex = (12 - ziweiIndex) % 12` in iztro's 寅-based frame).
pub fn tian_fu_branch(zi_wei: EarthlyBranch) -> EarthlyBranch {
    let zi_wei_index = zi_wei.index() as isize - EarthlyBranch::Yin.index() as isize;

    EarthlyBranch::Yin.offset(-zi_wei_index)
}

/// Returns each major star paired with the branch it occupies.
fn major_star_placements(
    bureau: FiveElementBureau,
    day: LunarDay,
) -> Vec<(EarthlyBranch, StarName)> {
    let zi_wei = zi_wei_branch(bureau, day);
    let tian_fu = tian_fu_branch(zi_wei);

    ZI_WEI_SERIES
        .iter()
        .map(|&(star, offset)| (zi_wei.offset(offset), star))
        .chain(
            TIAN_FU_SERIES
                .iter()
                .map(|&(star, offset)| (tian_fu.offset(offset), star)),
        )
        .collect()
}

/// Places the fourteen major stars deterministically from the lunar day and
/// five-element bureau.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct DeterministicMajorStarPlacer;

impl MajorStarPlacer for DeterministicMajorStarPlacer {
    fn place_major_stars(
        &self,
        chart: Chart,
        input: MajorStarPlacementInput,
    ) -> Result<Chart, ChartError> {
        let placements = major_star_placements(input.five_element_bureau(), input.lunar_day());

        let palaces = chart
            .palaces()
            .iter()
            .map(|palace| {
                let mut stars = palace.stars().to_vec();
                for &(branch, name) in &placements {
                    if branch == palace.branch() {
                        stars.push(StarPlacement::new(
                            name,
                            StarKind::Major,
                            major_star_brightness(name, branch),
                            birth_year_major_star_mutagen(input.birth_year_stem(), name),
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
