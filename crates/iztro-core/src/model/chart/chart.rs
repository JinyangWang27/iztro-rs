use crate::{
    error::ChartError,
    model::{
        bureau::FiveElementBureau,
        calendar::BirthContext,
        chart::palace::PalaceName,
        chart::snapshot::ChartStackSnapshot,
        profile::MethodProfile,
        star::mutagen::{Mutagen, Scope},
        star::{
            Brightness, KnownStarFamily, StarCategory, StarKind, StarName, try_known_star_metadata,
        },
    },
};
use lunar_lite::{EarthlyBranch, HeavenlyStem};
use serde::{Deserialize, Deserializer, Serialize};

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

    /// Returns an owned renderer-neutral stack snapshot of this natal chart.
    pub fn stack_snapshot(&self) -> ChartStackSnapshot {
        ChartStackSnapshot::from_natal_chart(self)
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
        self.stars_by_category(StarCategory::Major)
    }

    /// Returns one major-star placement with palace context.
    pub fn major_star(&self, name: StarName) -> Option<MajorStarPlacementRef<'_>> {
        self.star(name)
            .filter(|fact| fact.placement().category() == StarCategory::Major)
    }

    /// Returns the palace containing a major star, if present.
    pub fn palace_by_major_star(&self, name: StarName) -> Option<&Palace> {
        self.major_star(name).map(|fact| fact.palace())
    }

    /// Returns major-star placements in a palace name.
    pub fn major_stars_in_palace(&self, name: PalaceName) -> Vec<MajorStarPlacementRef<'_>> {
        self.stars_in_palace(name)
            .into_iter()
            .filter(|fact| fact.placement().category() == StarCategory::Major)
            .collect()
    }

    /// Returns major-star placements in an Earthly Branch.
    pub fn major_stars_in_branch(&self, branch: EarthlyBranch) -> Vec<MajorStarPlacementRef<'_>> {
        self.stars_in_branch(branch)
            .into_iter()
            .filter(|fact| fact.placement().category() == StarCategory::Major)
            .collect()
    }

    /// Returns all star placements with their palace context.
    pub fn stars(&self) -> Vec<StarPlacementRef<'_>> {
        self.palaces.iter().flat_map(stars_in).collect()
    }

    /// Returns one star placement with palace context.
    pub fn star(&self, name: StarName) -> Option<StarPlacementRef<'_>> {
        self.palaces.iter().find_map(|palace| {
            palace
                .stars()
                .iter()
                .find(|star| star.name() == name)
                .map(|placement| StarPlacementRef::new(palace, placement))
        })
    }

    /// Returns the palace containing a star, if present.
    pub fn palace_containing_star(&self, name: StarName) -> Option<&Palace> {
        self.star(name).map(|fact| fact.palace())
    }

    /// Returns star placements in a palace name.
    pub fn stars_in_palace(&self, name: PalaceName) -> Vec<StarPlacementRef<'_>> {
        self.palaces
            .iter()
            .filter(|palace| palace.name() == name)
            .flat_map(stars_in)
            .collect()
    }

    /// Returns star placements in an Earthly Branch.
    pub fn stars_in_branch(&self, branch: EarthlyBranch) -> Vec<StarPlacementRef<'_>> {
        self.palaces
            .iter()
            .filter(|palace| palace.branch() == branch)
            .flat_map(stars_in)
            .collect()
    }

    /// Returns star placements in a coarse category.
    pub fn stars_by_category(&self, category: StarCategory) -> Vec<StarPlacementRef<'_>> {
        self.stars()
            .into_iter()
            .filter(|fact| fact.placement().category() == category)
            .collect()
    }

    /// Returns star placements in an iztro-compatible fine kind.
    pub fn stars_by_kind(&self, kind: StarKind) -> Vec<StarPlacementRef<'_>> {
        self.stars()
            .into_iter()
            .filter(|fact| fact.placement().kind() == kind)
            .collect()
    }

    /// Returns all decorative (untyped) star placements with palace context.
    ///
    /// Decorative entries are a separate fact surface from typed
    /// [`StarPlacement`]s: they never appear in [`Chart::stars`].
    pub fn decorative_stars(&self) -> Vec<DecorativeStarPlacementRef<'_>> {
        self.palaces.iter().flat_map(decorative_stars_in).collect()
    }

    /// Returns one decorative star placement with palace context.
    pub fn decorative_star(&self, name: StarName) -> Option<DecorativeStarPlacementRef<'_>> {
        self.palaces.iter().find_map(|palace| {
            palace
                .decorative_stars()
                .iter()
                .find(|star| star.name() == name)
                .map(|placement| DecorativeStarPlacementRef::new(palace, placement))
        })
    }
}

fn stars_in(palace: &Palace) -> impl Iterator<Item = StarPlacementRef<'_>> {
    palace
        .stars()
        .iter()
        .map(|placement| StarPlacementRef::new(palace, placement))
}

fn decorative_stars_in(palace: &Palace) -> impl Iterator<Item = DecorativeStarPlacementRef<'_>> {
    palace
        .decorative_stars()
        .iter()
        .map(|placement| DecorativeStarPlacementRef::new(palace, placement))
}

/// A borrowed star placement together with the palace containing it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StarPlacementRef<'a> {
    palace: &'a Palace,
    placement: &'a StarPlacement,
}

impl<'a> StarPlacementRef<'a> {
    /// Creates a borrowed star placement fact with palace context.
    pub const fn new(palace: &'a Palace, placement: &'a StarPlacement) -> Self {
        Self { palace, placement }
    }

    /// Returns the palace containing this star.
    pub const fn palace(&self) -> &'a Palace {
        self.palace
    }

    /// Returns the star placement.
    pub const fn placement(&self) -> &'a StarPlacement {
        self.placement
    }
}

/// A borrowed major-star placement together with the palace containing it.
pub type MajorStarPlacementRef<'a> = StarPlacementRef<'a>;

/// A borrowed decorative star placement together with the palace containing it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DecorativeStarPlacementRef<'a> {
    palace: &'a Palace,
    placement: &'a DecorativeStarPlacement,
}

impl<'a> DecorativeStarPlacementRef<'a> {
    /// Creates a borrowed decorative star placement fact with palace context.
    pub const fn new(palace: &'a Palace, placement: &'a DecorativeStarPlacement) -> Self {
        Self { palace, placement }
    }

    /// Returns the palace containing this decorative star.
    pub const fn palace(&self) -> &'a Palace {
        self.palace
    }

    /// Returns the decorative star placement.
    pub const fn placement(&self) -> &'a DecorativeStarPlacement {
        self.placement
    }

    /// Returns the branch of the palace containing this decorative star.
    pub const fn branch(&self) -> EarthlyBranch {
        self.palace.branch()
    }

    /// Returns the decorative star name.
    pub const fn name(&self) -> StarName {
        self.placement.name()
    }
}

/// A palace with its branch, stem, and star placements.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Palace {
    name: PalaceName,
    branch: EarthlyBranch,
    stem: HeavenlyStem,
    stars: Vec<StarPlacement>,
    /// Untyped decorative runtime entries (长生/博士/岁前/将前十二神). Skipped when
    /// empty so charts without decorative placement serialize unchanged.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    decorative_stars: Vec<DecorativeStarPlacement>,
}

impl Palace {
    /// Creates a palace fact container with no decorative entries.
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
            decorative_stars: Vec::new(),
        }
    }

    /// Returns this palace with its decorative star placements replaced.
    ///
    /// Decorative entries are a separate fact surface from typed [`StarPlacement`]s
    /// and never alter [`Palace::stars`].
    pub fn with_decorative_stars(mut self, decorative_stars: Vec<DecorativeStarPlacement>) -> Self {
        self.decorative_stars = decorative_stars;
        self
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

    /// Returns typed stars placed in this palace.
    pub fn stars(&self) -> &[StarPlacement] {
        &self.stars
    }

    /// Returns decorative (untyped) star placements in this palace.
    pub fn decorative_stars(&self) -> &[DecorativeStarPlacement] {
        &self.decorative_stars
    }
}

/// A star placement within a palace.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StarPlacement {
    name: StarName,
    kind: StarKind,
    brightness: Brightness,
    mutagen: Option<Mutagen>,
    scope: Scope,
}

impl StarPlacement {
    /// Creates a typed star placement fact.
    pub const fn new(
        name: StarName,
        kind: StarKind,
        brightness: Brightness,
        mutagen: Option<Mutagen>,
        scope: Scope,
    ) -> Self {
        Self {
            name,
            kind,
            brightness,
            mutagen,
            scope,
        }
    }

    /// Returns the star name.
    pub const fn name(&self) -> StarName {
        self.name
    }

    /// Returns the iztro-compatible fine star type.
    pub const fn kind(&self) -> StarKind {
        self.kind
    }

    /// Returns the coarse palace grouping.
    pub const fn category(&self) -> StarCategory {
        self.kind.category()
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

/// One of the four untyped "twelve gods" runtime star families.
///
/// These families have no concrete [`StarKind`] upstream, so their entries are
/// modelled as [`DecorativeStarPlacement`]s rather than typed [`StarPlacement`]s.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecorativeStarFamily {
    /// 长生十二神 (Changsheng twelve phases).
    Changsheng12,
    /// 博士十二神 (Boshi twelve gods).
    Boshi12,
    /// 岁前十二神 (Suiqian twelve gods).
    Suiqian12,
    /// 将前十二神 (Jiangqian twelve gods).
    Jiangqian12,
}

impl DecorativeStarFamily {
    /// Returns the broad runtime-inventory family for this decorative family.
    pub const fn known_family(self) -> KnownStarFamily {
        match self {
            Self::Changsheng12 => KnownStarFamily::Changsheng12,
            Self::Boshi12 => KnownStarFamily::Boshi12,
            Self::Suiqian12 => KnownStarFamily::Suiqian12,
            Self::Jiangqian12 => KnownStarFamily::Jiangqian12,
        }
    }

    /// Returns the decorative family for a runtime-inventory family, if it is one
    /// of the four decorative "twelve gods" families.
    pub const fn from_known_family(family: KnownStarFamily) -> Option<Self> {
        match family {
            KnownStarFamily::Changsheng12 => Some(Self::Changsheng12),
            KnownStarFamily::Boshi12 => Some(Self::Boshi12),
            KnownStarFamily::Suiqian12 => Some(Self::Suiqian12),
            KnownStarFamily::Jiangqian12 => Some(Self::Jiangqian12),
            _ => None,
        }
    }
}

/// An untyped decorative star placement within a palace.
///
/// Unlike [`StarPlacement`], decorative entries carry no [`StarKind`]: upstream
/// iztro emits them as bare names. The [`DecorativeStarPlacement::try_new`]
/// constructor validates that the name is a known decorative star whose family
/// matches and whose known metadata has no [`StarKind`].
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DecorativeStarPlacement {
    name: StarName,
    family: DecorativeStarFamily,
    scope: Scope,
}

impl DecorativeStarPlacement {
    /// Creates a decorative star placement after validating it against the
    /// known-star inventory.
    ///
    /// Returns [`ChartError::InvalidDecorativeStarPlacement`] when `name` is not a
    /// known star, when its known family differs from `family`, or when its known
    /// metadata carries a [`StarKind`] (i.e. it is a typed star, not decorative).
    pub fn try_new(
        name: StarName,
        family: DecorativeStarFamily,
        scope: Scope,
    ) -> Result<Self, ChartError> {
        let metadata = try_known_star_metadata(name)
            .ok_or(ChartError::InvalidDecorativeStarPlacement { star: name })?;
        if metadata.family() != family.known_family() || metadata.kind().is_some() {
            return Err(ChartError::InvalidDecorativeStarPlacement { star: name });
        }

        Ok(Self {
            name,
            family,
            scope,
        })
    }

    /// Returns the decorative star name.
    pub const fn name(&self) -> StarName {
        self.name
    }

    /// Returns the decorative star family.
    pub const fn family(&self) -> DecorativeStarFamily {
        self.family
    }

    /// Returns the scope of this decorative placement.
    pub const fn scope(&self) -> Scope {
        self.scope
    }
}

impl<'de> Deserialize<'de> for DecorativeStarPlacement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct DecorativeStarPlacementData {
            name: StarName,
            family: DecorativeStarFamily,
            scope: Scope,
        }

        let data = DecorativeStarPlacementData::deserialize(deserializer)?;
        Self::try_new(data.name, data.family, data.scope).map_err(serde::de::Error::custom)
    }
}
