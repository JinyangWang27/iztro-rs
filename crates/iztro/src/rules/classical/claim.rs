//! Runtime claim (判断) types: the structured, evidence-backed output of the
//! classical rule engine.
//!
//! A [`Claim`] is consumed by future CLI / GUI / MCP / WASM / narrative layers. It
//! carries stable machine identity (`rule_id`, `claim_key`, typed enums) and
//! Chinese source references, but **no human-facing prose** — localized text is
//! rendered by `iztro-i18n` from `claim_key`.

use serde::{Deserialize, Serialize};

use crate::core::Scope;
use crate::rules::classical::evidence::Evidence;
use crate::rules::classical::rule::ClassicalRuleId;
use crate::rules::classical::source::SourceRef;
use crate::rules::classical::theme::{ClaimPolarity, ClaimTheme};

/// The life-area domain (领域) a claim affects.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimDomain {
    /// 命 (life / identity).
    Life,
    /// 身 (body).
    Body,
    /// 性情 (temperament).
    Temperament,
    /// 事业 (career).
    Career,
    /// 财帛 (wealth).
    Wealth,
    /// 迁移 (migration).
    Migration,
    /// 人际 (relationship).
    Relationship,
    /// 婚姻 (marriage).
    Marriage,
    /// 子女 (children).
    Children,
    /// 父母 (parents).
    Parents,
    /// 兄弟 (siblings).
    Siblings,
    /// 交友 (friends).
    Friends,
    /// 田宅 (property).
    Property,
    /// 疾厄 (health).
    Health,
    /// 福德 (fortune).
    Fortune,
    /// 运限 (timing).
    Timing,
}

/// The temporal scope a claim is asserted within.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimScope {
    /// 本命 (natal).
    Natal,
    /// 大限 (decadal).
    Decadal,
    /// 小限 (nominal-age).
    Age,
    /// 流年 (yearly).
    Yearly,
    /// 流月 (monthly).
    Monthly,
    /// 流日 (daily).
    Daily,
    /// 流时 (hourly).
    Hourly,
}

impl ClaimScope {
    /// A stable lowercase token used in [`ClaimId`] derivation.
    pub const fn token(self) -> &'static str {
        match self {
            Self::Natal => "natal",
            Self::Decadal => "decadal",
            Self::Age => "age",
            Self::Yearly => "yearly",
            Self::Monthly => "monthly",
            Self::Daily => "daily",
            Self::Hourly => "hourly",
        }
    }
}

impl From<Scope> for ClaimScope {
    fn from(scope: Scope) -> Self {
        match scope {
            Scope::Natal => Self::Natal,
            Scope::Decadal => Self::Decadal,
            Scope::Age => Self::Age,
            Scope::Yearly => Self::Yearly,
            Scope::Monthly => Self::Monthly,
            Scope::Daily => Self::Daily,
            Scope::Hourly => Self::Hourly,
        }
    }
}

/// A claim's confidence/intensity, normalized to `0.0..=1.0`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ClaimStrength(f32);

impl ClaimStrength {
    /// Creates a strength, clamping into `0.0..=1.0`.
    pub fn new(value: f32) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    /// Returns the normalized strength value.
    pub const fn value(self) -> f32 {
        self.0
    }
}

/// A stable identifier for an emitted claim.
///
/// Derived deterministically from the rule id and scope, so repeated evaluations
/// produce identical ids.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ClaimId(String);

impl ClaimId {
    /// Builds a claim id from a rule id and scope (e.g. `rule.id@natal`).
    pub fn new(rule_id: &ClassicalRuleId, scope: ClaimScope) -> Self {
        Self(format!("{}@{}", rule_id.as_str(), scope.token()))
    }

    /// Returns the id as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A structured, evidence-backed runtime claim produced by a classical rule.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Claim {
    /// Stable claim id (rule id + scope).
    pub id: ClaimId,
    /// The classical rule that produced this claim.
    pub rule_id: ClassicalRuleId,
    /// The life-area domain the claim affects.
    pub domain: ClaimDomain,
    /// The semantic themes the claim speaks to.
    pub themes: Vec<ClaimTheme>,
    /// The claim's overall valence.
    pub polarity: ClaimPolarity,
    /// The claim's normalized strength.
    pub strength: ClaimStrength,
    /// The temporal scope the claim is asserted within.
    pub scope: ClaimScope,
    /// Machine-readable supporting evidence.
    pub evidence: Vec<Evidence>,
    /// Machine-readable counter-evidence, where applicable. Always present in
    /// JSON (as `[]` when empty) so consumers can rely on the field.
    #[serde(default)]
    pub counter_evidence: Vec<Evidence>,
    /// Classical source references backing the claim.
    pub source_refs: Vec<SourceRef>,
    /// The i18n key used to render the claim's localized short text.
    pub claim_key: String,
}

impl Claim {
    /// Returns the i18n key for localized rendering.
    pub fn claim_key(&self) -> &str {
        &self.claim_key
    }
}
