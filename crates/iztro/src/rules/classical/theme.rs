//! Claim themes (主题) and polarity (吉凶) — typed, closed vocabularies.
//!
//! These are stable machine identities. Localized labels live in `iztro-i18n`
//! Fluent resources; the Chinese display text is never used as a logic key.

use serde::{Deserialize, Serialize};

/// A semantic theme a claim speaks to.
///
/// A closed vocabulary: new themes are added deliberately, not improvised as free
/// strings. Localized labels are rendered from the variant's stable key.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimTheme {
    /// 奔波迁动.
    RestlessMovement,
    /// 远方发展.
    RemoteDevelopment,
    /// 不稳定.
    Instability,
    /// 阻碍.
    Obstruction,
    /// 贵人扶持.
    NoblemanSupport,
    /// 缺乏助力.
    LackOfSupport,
    /// 权威.
    Authority,
    /// 责任.
    Responsibility,
    /// 劳碌压力.
    WorkPressure,
    /// 事业成就.
    CareerAchievement,
    /// 财富积累.
    WealthAccumulation,
    /// 财务波动.
    FinancialVolatility,
    /// 破财.
    FinancialLoss,
    /// 置产.
    AssetBuilding,
    /// 声名.
    Reputation,
    /// 文才.
    LiteraryTalent,
    /// 沟通.
    Communication,
    /// 和谐.
    Harmony,
    /// 冲突.
    Conflict,
    /// 分离.
    Separation,
    /// 孤独.
    Loneliness,
    /// 稳定.
    Stability,
    /// 抱负.
    Ambition,
    /// 冲动.
    Impulsiveness,
    /// 焦虑.
    Anxiety,
    /// 活力.
    Vitality,
    /// 疾病风险.
    IllnessRisk,
    /// 受伤风险.
    InjuryRisk,
    /// 福泽.
    Blessing,
    /// 牵制.
    Constraint,
    /// 损害.
    Damage,
    /// 隐患.
    HiddenRisk,
}

/// The overall valence (吉凶) of a claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimPolarity {
    /// 吉.
    Positive,
    /// 凶.
    Negative,
    /// 吉凶参半.
    Mixed,
    /// 偏吉.
    MixedPositive,
    /// 偏凶.
    MixedNegative,
}
