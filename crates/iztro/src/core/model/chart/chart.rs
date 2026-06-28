use lunar_lite::{EarthlyBranch, FourPillars, HeavenlyStem, StemBranch};
use crate::core::{
    error::ChartError,
    model::{
        bureau::FiveElementBureau,
        calendar::BirthContext,
        chart::ChartDiagnosticSnapshot,
        chart::horoscope::{HoroscopeLunarDate, HoroscopeSolarDate},
        chart::palace::PalaceName,
        chart::snapshot::ChartStackSnapshot,
        profile::{ChartAlgorithmKind, ChartPlane, ChartProfile, MethodProfile},
        star::mutagen::{Mutagen, Scope},
        star::{
            Brightness, KnownStarFamily, StarCategory, StarKind, StarName, try_known_star_metadata,
        },
    },
};
use serde::{Deserialize, Deserializer, Serialize};

/// Number of palaces required for a complete chart.
pub const PALACE_COUNT: usize = 12;

/// A complete chart placeholder composed of deterministic chart facts.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Chart {
    birth_context: BirthContext,
    birth_year: StemBranch,
    #[serde(default)]
    four_pillars: Option<FourPillars>,
    /// Flattened so `method_profile` and `chart_plane` stay top-level keys,
    /// preserving the pre-`ChartProfile` serialized shape (and letting charts
    /// without a `chart_plane` key deserialize as [`ChartPlane::Heaven`]).
    #[serde(flatten)]
    chart_profile: ChartProfile,
    palaces: Vec<Palace>,
    body_palace_branch: Option<EarthlyBranch>,
    five_element_bureau: Option<FiveElementBureau>,
    /// Retained natal solar/lunar display dates, when the facade could derive
    /// both (currently only the solar facade). Skipped when absent so charts
    /// built without them serialize unchanged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    natal_date_facts: Option<NatalDateFacts>,
}

/// Retained natal display dates: the original Gregorian/solar date and the
/// converted Chinese lunisolar date.
///
/// These are presentation facts: the canonical chart-generation inputs remain
/// the [`BirthContext`] and birth-year stem-branch. They are retained because a
/// solar-input chart otherwise loses its original Gregorian date once it is
/// converted to lunar facts for star placement.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NatalDateFacts {
    solar: HoroscopeSolarDate,
    lunar: HoroscopeLunarDate,
}

impl NatalDateFacts {
    /// Creates retained natal display dates.
    pub const fn new(solar: HoroscopeSolarDate, lunar: HoroscopeLunarDate) -> Self {
        Self { solar, lunar }
    }

    /// Returns the original Gregorian/solar natal date.
    pub const fn solar(&self) -> HoroscopeSolarDate {
        self.solar
    }

    /// Returns the converted Chinese lunisolar natal date.
    pub const fn lunar(&self) -> HoroscopeLunarDate {
        self.lunar
    }
}

impl Chart {
    /// Creates a chart from typed chart facts after checking core invariants.
    ///
    /// The chart plane defaults to [`ChartPlane::Heaven`], preserving the
    /// existing chart-generation behaviour. Use
    /// [`Chart::try_new_with_profile`] to construct a chart for an explicit
    /// chart plane.
    pub fn try_new(
        birth_context: BirthContext,
        birth_year: StemBranch,
        method_profile: MethodProfile,
        palaces: Vec<Palace>,
        body_palace_branch: Option<EarthlyBranch>,
        five_element_bureau: Option<FiveElementBureau>,
    ) -> Result<Self, ChartError> {
        Self::try_new_with_four_pillars(
            birth_context,
            birth_year,
            None,
            method_profile,
            palaces,
            body_palace_branch,
            five_element_bureau,
        )
    }

    /// Creates a chart from typed chart facts and optional natal four pillars.
    ///
    /// The chart plane defaults to [`ChartPlane::Heaven`]. Use
    /// [`Chart::try_new_with_four_pillars_and_profile`] to construct a chart for
    /// an explicit chart plane.
    pub fn try_new_with_four_pillars(
        birth_context: BirthContext,
        birth_year: StemBranch,
        four_pillars: Option<FourPillars>,
        method_profile: MethodProfile,
        palaces: Vec<Palace>,
        body_palace_branch: Option<EarthlyBranch>,
        five_element_bureau: Option<FiveElementBureau>,
    ) -> Result<Self, ChartError> {
        Self::try_new_with_four_pillars_and_profile(
            birth_context,
            birth_year,
            four_pillars,
            ChartProfile::new(method_profile, ChartPlane::Heaven),
            palaces,
            body_palace_branch,
            five_element_bureau,
        )
    }

    /// Creates a chart from typed chart facts and an explicit chart profile.
    ///
    /// Like [`Chart::try_new`], but the chart records the supplied
    /// [`ChartProfile`] (method profile + chart plane) instead of defaulting the
    /// plane to [`ChartPlane::Heaven`].
    pub fn try_new_with_profile(
        birth_context: BirthContext,
        birth_year: StemBranch,
        chart_profile: ChartProfile,
        palaces: Vec<Palace>,
        body_palace_branch: Option<EarthlyBranch>,
        five_element_bureau: Option<FiveElementBureau>,
    ) -> Result<Self, ChartError> {
        Self::try_new_with_four_pillars_and_profile(
            birth_context,
            birth_year,
            None,
            chart_profile,
            palaces,
            body_palace_branch,
            five_element_bureau,
        )
    }

    /// Creates a chart from typed chart facts, optional natal four pillars, and
    /// an explicit chart profile.
    pub fn try_new_with_four_pillars_and_profile(
        birth_context: BirthContext,
        birth_year: StemBranch,
        four_pillars: Option<FourPillars>,
        chart_profile: ChartProfile,
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

        if let Some(pillars) = four_pillars {
            if pillars.yearly != birth_year {
                return Err(ChartError::FourPillarsBirthYearMismatch {
                    birth_year,
                    four_pillars_year: pillars.yearly,
                });
            }
        }

        Ok(Self {
            birth_context,
            birth_year,
            four_pillars,
            chart_profile,
            palaces,
            body_palace_branch,
            five_element_bureau,
            natal_date_facts: None,
        })
    }

    /// Returns this chart with retained natal solar/lunar display dates attached.
    pub fn with_natal_date_facts(mut self, facts: NatalDateFacts) -> Self {
        self.natal_date_facts = Some(facts);
        self
    }

    /// Returns this chart with its chart profile replaced.
    ///
    /// Used by the facade to make a generated chart self-describing: low-level
    /// builders produce a default [`ChartPlane::Heaven`] chart, and the facade
    /// attaches the requested chart plane via this consuming method without
    /// mutating placement facts.
    ///
    /// Crate-internal: charts are made self-describing at the facade boundary,
    /// so this is not part of the public construction surface.
    pub(crate) fn with_chart_profile(mut self, chart_profile: ChartProfile) -> Self {
        self.chart_profile = chart_profile;
        self
    }

    /// Returns the retained natal solar/lunar display dates, if attached.
    pub const fn natal_date_facts(&self) -> Option<&NatalDateFacts> {
        self.natal_date_facts.as_ref()
    }

    /// Returns the birth context used by this chart.
    pub const fn birth_context(&self) -> &BirthContext {
        &self.birth_context
    }

    /// Returns the birth-year stem-branch used for natal chart derivation.
    pub const fn birth_year(&self) -> StemBranch {
        self.birth_year
    }

    /// Returns the natal four pillars, if the chart facade could derive them safely.
    pub const fn four_pillars(&self) -> Option<&FourPillars> {
        self.four_pillars.as_ref()
    }

    /// Returns the chart-generation profile metadata (method profile + plane).
    pub const fn chart_profile(&self) -> &ChartProfile {
        &self.chart_profile
    }

    /// Returns the method profile metadata.
    ///
    /// Preserved for backward compatibility; delegates to
    /// [`ChartProfile::method_profile`].
    pub const fn method_profile(&self) -> &MethodProfile {
        self.chart_profile.method_profile()
    }

    /// Returns the chart plane (天盘 / 地盘 / 人盘) this chart represents.
    pub const fn chart_plane(&self) -> ChartPlane {
        self.chart_profile.chart_plane()
    }

    /// Returns the typed chart algorithm kind for this chart.
    pub const fn algorithm_kind(&self) -> ChartAlgorithmKind {
        self.chart_profile.algorithm_kind()
    }

    /// Returns the palaces in this chart.
    pub fn palaces(&self) -> &[Palace] {
        &self.palaces
    }

    /// Returns an owned renderer-neutral stack snapshot of this natal chart.
    pub fn stack_snapshot(&self) -> ChartStackSnapshot {
        ChartStackSnapshot::from_natal_chart(self)
    }

    /// Returns a compact serializable snapshot for structural diagnostics.
    pub fn diagnostic_snapshot(&self) -> ChartDiagnosticSnapshot {
        ChartDiagnosticSnapshot::from_chart(self)
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

        self.palace_by_branch(body_branch)
    }

    /// Returns the five-element bureau (五行局), if calculated.
    pub const fn five_element_bureau(&self) -> Option<FiveElementBureau> {
        self.five_element_bureau
    }

    /// Returns the Life Palace, identified by [`PalaceName::Life`], if present.
    pub fn life_palace(&self) -> Option<&Palace> {
        self.palace_by_name(PalaceName::Life)
    }

    /// Returns the palace with the given palace name, if present.
    ///
    /// Prefer this `Option` variant when the caller can tolerate a
    /// malformed or incomplete chart. For chart-generation logic where a
    /// missing palace indicates a violated invariant, use
    /// [`Chart::required_palace_by_name`] instead.
    pub fn palace_by_name(&self, name: PalaceName) -> Option<&Palace> {
        self.palaces.iter().find(|palace| palace.name() == name)
    }

    /// Returns the palace occupying the given earthly branch, if present.
    ///
    /// Prefer this `Option` variant when the caller can tolerate a
    /// malformed or incomplete chart. For chart-generation logic where a
    /// missing palace indicates a violated invariant, use
    /// [`Chart::required_palace_by_branch`] instead.
    pub fn palace_by_branch(&self, branch: EarthlyBranch) -> Option<&Palace> {
        self.palaces.iter().find(|palace| palace.branch() == branch)
    }

    /// Returns the earthly branch of the palace with the given palace name, if present.
    pub fn branch_of_palace(&self, name: PalaceName) -> Option<EarthlyBranch> {
        self.palace_by_name(name).map(Palace::branch)
    }

    /// Returns the palace name assigned to the given earthly branch, if present.
    pub fn palace_name_at_branch(&self, branch: EarthlyBranch) -> Option<PalaceName> {
        self.palace_by_branch(branch).map(Palace::name)
    }

    /// Returns the palace with the given palace name, or
    /// [`ChartError::RequiredPalaceNameMissing`] if it is absent.
    ///
    /// Use this in chart-generation logic where every canonical palace name
    /// must be present; a missing palace is a violated invariant rather than a
    /// tolerable absence. Callers that can handle absence should use
    /// [`Chart::palace_by_name`].
    pub fn required_palace_by_name(&self, name: PalaceName) -> Result<&Palace, ChartError> {
        self.palace_by_name(name)
            .ok_or(ChartError::RequiredPalaceNameMissing { palace_name: name })
    }

    /// Returns the palace occupying the given earthly branch, or
    /// [`ChartError::RequiredPalaceBranchMissing`] if no palace occupies it.
    ///
    /// Use this in chart-generation logic where the branch is expected to be
    /// occupied; an empty branch is a violated invariant rather than a
    /// tolerable absence. Callers that can handle absence should use
    /// [`Chart::palace_by_branch`].
    pub fn required_palace_by_branch(&self, branch: EarthlyBranch) -> Result<&Palace, ChartError> {
        self.palace_by_branch(branch)
            .ok_or(ChartError::RequiredPalaceBranchMissing { branch })
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
///
/// The derived [`Ord`]/[`PartialOrd`] follow the variant declaration order and
/// exist only to give facade/export snapshots a stable, deterministic
/// decorative-star ordering key (see
/// [`crate::core::model::chart::facade_snapshot`]). They do not affect placement.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use lunar_lite::{EARTHLY_BRANCHES, HEAVENLY_STEMS};
    use crate::core::model::{
        calendar::{CalendarDate, Gender},
        chart::PALACE_NAMES,
        profile::ChartAlgorithmKind,
    };
    use serde_json::Value;

    fn method_profile() -> MethodProfile {
        MethodProfile::new("zhongzhou_test", ChartAlgorithmKind::Zhongzhou, "test")
    }

    fn sample_chart(chart_profile: ChartProfile) -> Chart {
        let palaces = PALACE_NAMES
            .iter()
            .copied()
            .enumerate()
            .map(|(index, name)| {
                Palace::new(
                    name,
                    EARTHLY_BRANCHES[index],
                    HEAVENLY_STEMS[index % HEAVENLY_STEMS.len()],
                    Vec::new(),
                )
            })
            .collect();

        Chart::try_new_with_profile(
            BirthContext::new(
                CalendarDate::solar(1990, 5, 17),
                EarthlyBranch::Chen,
                Gender::Female,
            ),
            StemBranch::try_new(HeavenlyStem::Geng, EarthlyBranch::Wu)
                .expect("valid sexagenary pair"),
            chart_profile,
            palaces,
            None,
            None,
        )
        .expect("sample chart should build")
    }

    #[test]
    fn serialization_is_flattened_with_top_level_keys() {
        let chart = sample_chart(ChartProfile::new(method_profile(), ChartPlane::Heaven));
        let value: Value = serde_json::to_value(&chart).expect("chart should serialize");
        let object = value
            .as_object()
            .expect("chart should serialize to an object");

        assert!(
            object.contains_key("method_profile"),
            "expected top-level method_profile key",
        );
        assert!(
            object.contains_key("chart_plane"),
            "expected top-level chart_plane key",
        );
        assert!(
            !object.contains_key("chart_profile"),
            "chart_profile must not appear as a nested key",
        );
        assert_eq!(object["chart_plane"], Value::String("heaven".to_owned()));
    }

    #[test]
    fn old_json_without_chart_plane_deserializes_as_heaven() {
        let chart = sample_chart(ChartProfile::new(method_profile(), ChartPlane::Heaven));
        let mut value: Value = serde_json::to_value(&chart).expect("chart should serialize");
        value
            .as_object_mut()
            .expect("chart object")
            .remove("chart_plane");

        let decoded: Chart = serde_json::from_value(value).expect("legacy JSON should deserialize");

        assert_eq!(decoded.chart_plane(), ChartPlane::Heaven);
        assert_eq!(decoded.method_profile(), &method_profile());
    }

    #[test]
    fn json_with_earth_chart_plane_deserializes_as_earth() {
        let chart = sample_chart(ChartProfile::new(method_profile(), ChartPlane::Heaven));
        let mut value: Value = serde_json::to_value(&chart).expect("chart should serialize");
        value
            .as_object_mut()
            .expect("chart object")
            .insert("chart_plane".to_owned(), Value::String("earth".to_owned()));

        let decoded: Chart = serde_json::from_value(value).expect("JSON should deserialize");

        assert_eq!(decoded.chart_plane(), ChartPlane::Earth);
    }

    #[test]
    fn round_trip_preserves_chart_profile() {
        let original = sample_chart(ChartProfile::new(method_profile(), ChartPlane::Human));
        let encoded = serde_json::to_string(&original).expect("chart should serialize");
        let decoded: Chart = serde_json::from_str(&encoded).expect("chart should deserialize");

        assert_eq!(decoded.chart_profile(), original.chart_profile());
        assert_eq!(decoded.chart_plane(), ChartPlane::Human);
        assert_eq!(decoded.method_profile(), original.method_profile());
        assert_eq!(decoded, original);
    }
}
