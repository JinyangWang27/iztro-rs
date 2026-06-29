//! Structured model types for pattern (格局) detection.
//!
//! Pattern detection is an **analytical, read-only** view over deterministic
//! chart facts. A [`PatternDetection`] is a *structured, explainable fact* about
//! how stars and palaces are arranged — it is **not** a narrative reading. No
//! interpretive prose belongs here; downstream narrative rendering consumes these
//! structured facts but lives in a separate layer.
//!
//! Detection never mutates chart facts and never folds temporal facts into natal
//! facts. Temporal scopes remain overlays carried explicitly in
//! [`PatternScope`]; they never rewrite natal placement.

use serde::{Deserialize, Serialize};

use crate::core::pattern::relation::PalaceRelation;
use crate::core::{EarthlyBranch, Mutagen, Scope, StarName};

/// Stable identifier for a recognized pattern (格局).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatternId {
    /// 紫府朝垣.
    ZiFuChaoYuan,
    /// 机月同梁.
    JiYueTongLiang,
    /// 羊陀夹忌.
    YangTuoJiaJi,
    /// 铃昌陀武.
    LingChangTuoWu,
    /// 左右夹命.
    ZuoYouJiaMing,
    /// 昌曲夹命.
    ChangQuJiaMing,
    /// 日月并明.
    RiYueBingMing,
    /// 日月反背.
    RiYueFanBei,
    /// 金灿光辉.
    JinCanGuangHui,
    /// 日出扶桑.
    RiChuFuSang,
    /// 月落亥宫.
    YueLuoHaiGong,
    /// 月生沧海.
    YueShengCangHai,
    /// 马头带剑.
    MaTouDaiJian,
    /// 贪火相逢.
    TanHuoXiangFeng,
    /// 武曲守垣.
    WuQuShouYuan,
    /// 财与囚仇.
    CaiYuQiuChou,
    /// 马落空亡.
    MaLuoKongWang,
}

/// Coarse family a pattern belongs to.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatternFamily {
    /// Combination of major stars.
    MajorStarCombination,
    /// Combination of auxiliary/assistant stars (辅佐星), e.g. 左辅/右弼,
    /// 文昌/文曲 clamping a palace.
    AuxiliaryStarCombination,
    /// Mutagen-driven pattern.
    Mutagen,
    /// Three-sides-four-directions (三方四正) structure.
    SanFangSiZheng,
    /// Adverse-star / 化忌 (煞忌) pattern.
    ShaJi,
    /// Temporal-overlay pattern.
    Temporal,
}

/// Overall valence of a pattern.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatternPolarity {
    /// Auspicious (吉).
    Auspicious,
    /// Inauspicious (凶).
    Inauspicious,
    /// Mixed (吉凶参半).
    Mixed,
}

/// Fulfilment/integrity status of a detected pattern.
///
/// A [`PatternDetection`] is emitted only when the base pattern formation
/// exists. Missing or incomplete base conditions produce no detection — there is
/// no `Partial` / 近格 status. Once a base formation exists, this status records
/// whether modeled weakening or breaker conditions damage it.
///
/// - [`PatternStatus::Fulfilled`] = 成格 (base structure exists, no modeled
///   weakening/breaker applies);
/// - [`PatternStatus::Weakened`] = 成而减力 (base structure exists but modeled
///   weakening factors apply);
/// - [`PatternStatus::Broken`] = 破格 (base structure exists but modeled breaker
///   conditions invalidate it).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatternStatus {
    /// 成格: base structure exists and no modeled weakening/breaker applies.
    Fulfilled,
    /// 成而减力: base structure exists but modeled weakening factors apply.
    Weakened,
    /// 破格: base structure exists but modeled breaker conditions invalidate it.
    Broken,
}

/// Coarse strength estimate for a detected pattern.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatternStrength {
    /// Weak.
    Weak,
    /// Medium.
    Medium,
    /// Strong.
    Strong,
}

/// Scope a pattern is asserted within.
///
/// Temporal variants describe overlays only; they never mutate natal facts.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatternScope {
    /// Natal (本命).
    Natal,
    /// Decadal (大限).
    Decadal,
    /// Nominal-age (小限).
    Age,
    /// Yearly (流年).
    Yearly,
    /// Monthly (流月).
    Monthly,
    /// Daily (流日).
    Daily,
    /// Hourly (流时).
    Hourly,
    /// A pattern spanning multiple temporal scopes.
    Combined(Vec<Scope>),
}

/// The primary chart object a pattern is anchored to.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatternAnchor {
    /// Anchored on a palace, identified by its Earthly Branch.
    Palace(EarthlyBranch),
    /// Anchored on a star.
    Star(StarName),
    /// Anchored on a mutagen.
    Mutagen(Mutagen),
    /// Anchored on the chart as a whole.
    Chart,
}

/// A concrete, machine-checkable reason a rule matched.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatternEvidence {
    /// A star sits in a specific palace branch.
    StarInPalace {
        /// The star.
        star: StarName,
        /// The palace branch containing it.
        branch: EarthlyBranch,
    },
    /// A star sits in a palace standing in a relation to the anchor.
    StarInPalaceRelation {
        /// The star.
        star: StarName,
        /// The anchor palace branch.
        anchor: EarthlyBranch,
        /// The branch containing the star.
        branch: EarthlyBranch,
        /// The relation of `branch` to `anchor`.
        relation: PalaceRelation,
    },
    /// Several stars share one palace branch.
    StarsInSamePalace {
        /// The stars.
        stars: Vec<StarName>,
        /// The shared palace branch.
        branch: EarthlyBranch,
    },
    /// Several stars fall within the 三方四正 of the anchor.
    StarsInSanFangSiZheng {
        /// The stars found.
        stars: Vec<StarName>,
        /// The anchor palace branch.
        anchor: EarthlyBranch,
        /// The branches the stars were found in.
        branches: Vec<EarthlyBranch>,
    },
    /// A mutagen is active on a star in a given scope and branch.
    MutagenOnStar {
        /// The star carrying the mutagen.
        star: StarName,
        /// The mutagen.
        mutagen: Mutagen,
        /// The scope producing the mutagen.
        scope: Scope,
        /// The branch containing the star.
        branch: EarthlyBranch,
    },
    /// Two palaces stand in a relation.
    ///
    /// `from` is the anchor/target palace, `to` is the related palace, and
    /// `relation` describes the relation of `to` to `from`. For example, a
    /// [`PalaceRelation::ClampedBy`] entry reads as "`to` clamps `from`".
    PalaceRelation {
        /// The anchor/target branch.
        from: EarthlyBranch,
        /// The related branch.
        to: EarthlyBranch,
        /// The relation of `to` to `from`.
        relation: PalaceRelation,
    },
}

/// A required, missing, weakening, or breaking condition for a pattern.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatternCondition {
    /// Requires a star to be present anywhere.
    RequiresStar {
        /// The required star.
        star: StarName,
    },
    /// Requires a star in a specific palace branch.
    RequiresStarInPalace {
        /// The required star.
        star: StarName,
        /// The branch it must occupy.
        branch: EarthlyBranch,
    },
    /// Requires a star in a palace standing in a relation to the anchor.
    RequiresStarInRelation {
        /// The required star.
        star: StarName,
        /// The anchor palace branch.
        anchor: EarthlyBranch,
        /// The required relation.
        relation: PalaceRelation,
    },
    /// Requires a mutagen on a star.
    RequiresMutagen {
        /// The star.
        star: StarName,
        /// The required mutagen.
        mutagen: Mutagen,
    },
    /// The pattern is weakened by a star in a branch.
    WeakenedByStar {
        /// The weakening star.
        star: StarName,
        /// The branch it occupies.
        branch: EarthlyBranch,
    },
    /// The pattern is broken by a star in a branch.
    BrokenByStar {
        /// The breaking star.
        star: StarName,
        /// The branch it occupies.
        branch: EarthlyBranch,
    },
}

/// A detected pattern (格局) fact on a specific chart.
///
/// This is a structured, explainable fact, not a narrative reading. `name_zh`
/// is a static label drawn from rule metadata; because it is a `&'static str`,
/// this struct derives [`Serialize`] but not `Deserialize` (a borrowed
/// `'static` string cannot be deserialized). All contained enums round-trip via
/// serde independently.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PatternDetection {
    /// Stable pattern identifier.
    pub id: PatternId,
    /// Canonical Chinese name (格局名), from static rule metadata.
    pub name_zh: &'static str,
    /// Coarse family.
    pub family: PatternFamily,
    /// Valence.
    pub polarity: PatternPolarity,
    /// Fulfilment status.
    pub status: PatternStatus,
    /// Coarse strength estimate.
    pub strength: PatternStrength,
    /// Scope the pattern is asserted within.
    pub scope: PatternScope,
    /// Primary anchor object.
    pub anchor: PatternAnchor,
    /// Palace branches involved in the pattern.
    pub involved_palaces: Vec<EarthlyBranch>,
    /// Stars involved in the pattern.
    pub involved_stars: Vec<StarName>,
    /// Mutagens involved in the pattern.
    pub involved_mutagens: Vec<Mutagen>,
    /// Evidence explaining why the rule matched.
    pub evidence: Vec<PatternEvidence>,
    /// Conditions that were required but missing.
    pub missing_conditions: Vec<PatternCondition>,
    /// Factors weakening the pattern.
    pub weakening_factors: Vec<PatternCondition>,
    /// Factors breaking the pattern.
    pub breaking_factors: Vec<PatternCondition>,
}
