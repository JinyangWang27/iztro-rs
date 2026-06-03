use crate::{
    bureau::FiveElementBureau,
    calendar::BirthContext,
    error::ChartError,
    ganzhi::{EarthlyBranch, HeavenlyStem},
    mutagen::{Mutagen, Scope},
    palace::PalaceName,
    profile::MethodProfile,
    star::{Brightness, StarCategory, StarName},
};
use serde::{Deserialize, Serialize};

/// Number of palaces required for a complete chart.
pub const PALACE_COUNT: usize = 12;

/// A complete chart placeholder composed of deterministic chart facts.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Chart {
    birth_context: BirthContext,
    method_profile: MethodProfile,
    palaces: Vec<Palace>,
    body_palace_branch: Option<EarthlyBranch>,
    five_element_bureau: Option<FiveElementBureau>,
}

impl Chart {
    /// Creates a chart from typed chart facts after checking core invariants.
    pub fn try_new(
        birth_context: BirthContext,
        method_profile: MethodProfile,
        palaces: Vec<Palace>,
        body_palace_branch: Option<EarthlyBranch>,
        five_element_bureau: Option<FiveElementBureau>,
    ) -> Result<Self, ChartError> {
        if palaces.len() != PALACE_COUNT {
            return Err(ChartError::InvalidPalaceCount {
                expected: PALACE_COUNT,
                actual: palaces.len(),
            });
        }

        Ok(Self {
            birth_context,
            method_profile,
            palaces,
            body_palace_branch,
            five_element_bureau,
        })
    }

    /// Returns the birth context used by this chart.
    pub const fn birth_context(&self) -> &BirthContext {
        &self.birth_context
    }

    /// Returns the method profile metadata.
    pub const fn method_profile(&self) -> &MethodProfile {
        &self.method_profile
    }

    /// Returns the palaces in this chart.
    pub fn palaces(&self) -> &[Palace] {
        &self.palaces
    }

    /// Returns the branch containing the Body Palace, if known.
    pub const fn body_palace_branch(&self) -> Option<EarthlyBranch> {
        self.body_palace_branch
    }

    /// Returns whether the given branch is the Body Palace branch.
    pub fn is_body_palace_branch(&self, branch: EarthlyBranch) -> bool {
        self.body_palace_branch == Some(branch)
    }

    /// Returns the palace containing the Body Palace, if known.
    pub fn body_palace(&self) -> Option<&Palace> {
        let body_branch = self.body_palace_branch?;

        self.palaces
            .iter()
            .find(|palace| palace.branch() == body_branch)
    }

    /// Returns the five-element bureau (五行局), if calculated.
    pub const fn five_element_bureau(&self) -> Option<FiveElementBureau> {
        self.five_element_bureau
    }

    /// Returns the Life Palace, identified by [`PalaceName::Life`], if present.
    pub fn life_palace(&self) -> Option<&Palace> {
        self.palaces
            .iter()
            .find(|palace| palace.name() == PalaceName::Life)
    }

    /// Returns all major-star placements with their palace context.
    pub fn major_stars(&self) -> Vec<MajorStarPlacementRef<'_>> {
        self.palaces
            .iter()
            .flat_map(|palace| {
                palace
                    .stars()
                    .iter()
                    .filter(|star| star.category() == StarCategory::Major)
                    .map(|placement| MajorStarPlacementRef::new(palace, placement))
            })
            .collect()
    }

    /// Returns one major-star placement with palace context.
    pub fn major_star(&self, name: StarName) -> Option<MajorStarPlacementRef<'_>> {
        self.palaces.iter().find_map(|palace| {
            palace
                .stars()
                .iter()
                .find(|star| star.category() == StarCategory::Major && star.name() == name)
                .map(|placement| MajorStarPlacementRef::new(palace, placement))
        })
    }

    /// Returns the palace containing a major star, if present.
    pub fn palace_by_major_star(&self, name: StarName) -> Option<&Palace> {
        self.major_star(name).map(|fact| fact.palace())
    }

    /// Returns major-star placements in a palace name.
    pub fn major_stars_in_palace(&self, name: PalaceName) -> Vec<MajorStarPlacementRef<'_>> {
        self.palaces
            .iter()
            .filter(|palace| palace.name() == name)
            .flat_map(major_stars_in)
            .collect()
    }

    /// Returns major-star placements in an Earthly Branch.
    pub fn major_stars_in_branch(&self, branch: EarthlyBranch) -> Vec<MajorStarPlacementRef<'_>> {
        self.palaces
            .iter()
            .filter(|palace| palace.branch() == branch)
            .flat_map(major_stars_in)
            .collect()
    }
}

fn major_stars_in(palace: &Palace) -> impl Iterator<Item = MajorStarPlacementRef<'_>> {
    palace
        .stars()
        .iter()
        .filter(|star| star.category() == StarCategory::Major)
        .map(|placement| MajorStarPlacementRef::new(palace, placement))
}

/// A borrowed major-star placement together with the palace containing it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MajorStarPlacementRef<'a> {
    palace: &'a Palace,
    placement: &'a StarPlacement,
}

impl<'a> MajorStarPlacementRef<'a> {
    /// Creates a borrowed major-star placement fact with palace context.
    pub const fn new(palace: &'a Palace, placement: &'a StarPlacement) -> Self {
        Self { palace, placement }
    }

    /// Returns the palace containing this major star.
    pub const fn palace(&self) -> &'a Palace {
        self.palace
    }

    /// Returns the star placement.
    pub const fn placement(&self) -> &'a StarPlacement {
        self.placement
    }
}

/// A palace with its branch, stem, and star placements.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Palace {
    name: PalaceName,
    branch: EarthlyBranch,
    stem: HeavenlyStem,
    stars: Vec<StarPlacement>,
}

impl Palace {
    /// Creates a palace fact container.
    pub fn new(
        name: PalaceName,
        branch: EarthlyBranch,
        stem: HeavenlyStem,
        stars: Vec<StarPlacement>,
    ) -> Self {
        Self {
            name,
            branch,
            stem,
            stars,
        }
    }

    /// Returns the palace name.
    pub const fn name(&self) -> PalaceName {
        self.name
    }

    /// Returns the palace branch.
    pub const fn branch(&self) -> EarthlyBranch {
        self.branch
    }

    /// Returns the palace stem.
    pub const fn stem(&self) -> HeavenlyStem {
        self.stem
    }

    /// Returns stars placed in this palace.
    pub fn stars(&self) -> &[StarPlacement] {
        &self.stars
    }
}

/// A star placement within a palace.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StarPlacement {
    name: StarName,
    category: StarCategory,
    brightness: Brightness,
    mutagen: Option<Mutagen>,
    scope: Scope,
}

impl StarPlacement {
    /// Creates a typed star placement fact.
    pub const fn new(
        name: StarName,
        category: StarCategory,
        brightness: Brightness,
        mutagen: Option<Mutagen>,
        scope: Scope,
    ) -> Self {
        Self {
            name,
            category,
            brightness,
            mutagen,
            scope,
        }
    }

    /// Returns the star name.
    pub const fn name(&self) -> StarName {
        self.name
    }

    /// Returns the star category.
    pub const fn category(&self) -> StarCategory {
        self.category
    }

    /// Returns the star brightness.
    pub const fn brightness(&self) -> Brightness {
        self.brightness
    }

    /// Returns the optional mutagen attached to this placement.
    pub const fn mutagen(&self) -> Option<Mutagen> {
        self.mutagen
    }

    /// Returns the scope of this placement.
    pub const fn scope(&self) -> Scope {
        self.scope
    }
}
