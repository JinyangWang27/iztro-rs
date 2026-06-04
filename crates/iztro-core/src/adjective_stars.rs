//! Deterministic adjective-star (杂耀) placement for the natal chart.
//!
//! Reproduces a first small subset of `iztro` 2.5.8 `adjectiveStars`
//! (`getAdjectiveStar` plus the `getLuanXiIndex`, `getMonthlyStarIndex`, and
//! `getTimelyStarIndex` helpers in `src/star`, MIT licensed). Only the six
//! month/time/year-branch deterministic stars are implemented here:
//!
//! - 红鸾 (HongLuan) / 天喜 (TianXi): from the birth year branch;
//! - 天姚 (TianYao) / 天刑 (TianXing): from the lunar month;
//! - 台辅 (TaiFu) / 封诰 (FengGao): from the birth time branch.
//!
//! The remaining adjective stars, brightness for adjective stars, temporal
//! scopes, leap-month behavior, and rat-hour variants stay out of scope.

use crate::{
    chart::{Chart, Palace, StarPlacement},
    error::ChartError,
    ganzhi::EarthlyBranch,
    life_body::LunarMonth,
    mutagen::Scope,
    star::{Brightness, StarMetadata, StarName},
};

/// Inputs required to place the supported adjective-star subset.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct AdjectiveStarPlacementInput {
    lunar_month: LunarMonth,
    birth_time: EarthlyBranch,
    birth_year_branch: EarthlyBranch,
}

impl AdjectiveStarPlacementInput {
    /// Creates adjective-star placement input from explicit lunar and ganzhi facts.
    pub const fn new(
        lunar_month: LunarMonth,
        birth_time: EarthlyBranch,
        birth_year_branch: EarthlyBranch,
    ) -> Self {
        Self {
            lunar_month,
            birth_time,
            birth_year_branch,
        }
    }

    /// Returns the validated lunar month.
    pub const fn lunar_month(self) -> LunarMonth {
        self.lunar_month
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
pub const fn adjective_star_metadata_table() -> &'static [StarMetadata; 6] {
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
        let placements = adjective_star_placements(input);

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
///   (子时 = 0).
fn adjective_star_placements(input: AdjectiveStarPlacementInput) -> [(EarthlyBranch, StarName); 6] {
    let month_offset = isize::from(input.lunar_month().value()) - 1;
    let time_index = input.birth_time().index() as isize;
    let year_branch_index = input.birth_year_branch().index() as isize;

    let hong_luan = EarthlyBranch::Mao.offset(-year_branch_index);
    let tian_xi = hong_luan.offset(6);
    let tian_yao = EarthlyBranch::Chou.offset(month_offset);
    let tian_xing = EarthlyBranch::You.offset(month_offset);
    let tai_fu = EarthlyBranch::Wu.offset(time_index);
    let feng_gao = EarthlyBranch::Yin.offset(time_index);

    [
        (hong_luan, StarName::HongLuan),
        (tian_xi, StarName::TianXi),
        (tian_yao, StarName::TianYao),
        (tian_xing, StarName::TianXing),
        (tai_fu, StarName::TaiFu),
        (feng_gao, StarName::FengGao),
    ]
}
