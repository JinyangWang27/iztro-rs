use crate::rules::classical::claim::ClaimScope;

// Stable rule ids handled by the classical evaluator.
pub(crate) const TIAN_MA_VOID: &str = "migration.tian_ma_void.restless_movement";
pub(crate) const YANG_TUO_CLAMP_LIFE: &str = "life.yang_tuo_clamp_life.constraint_damage";
pub(crate) const CHANG_QU_CLAMP_LIFE: &str = "life.chang_qu_clamp_life.literary_reputation";
pub(crate) const LU_MA_JIAO_CHI: &str = "fortune.lu_ma_jiao_chi.favorable_convergence";
pub(crate) const RI_YUE_FAN_BEI: &str = "life.ri_yue_fan_bei.hardship_pressure";
pub(crate) const TAN_LANG_HAI_ZI: &str = "relationship.tan_ju_hai_zi.water_romance";
pub(crate) const XING_YU_TAN_LANG: &str = "relationship.xing_yu_tan_lang.romance_with_penalty";
pub(crate) const SHAN_FU_JU_KONG: &str = "fortune.shan_fu_ju_kong.monastic_life";

/// Every rule id the evaluator dispatch has a predicate arm for.
pub(crate) const HANDLED_RULE_IDS: [&str; 8] = [
    TIAN_MA_VOID,
    YANG_TUO_CLAMP_LIFE,
    CHANG_QU_CLAMP_LIFE,
    LU_MA_JIAO_CHI,
    RI_YUE_FAN_BEI,
    TAN_LANG_HAI_ZI,
    XING_YU_TAN_LANG,
    SHAN_FU_JU_KONG,
];

pub(crate) const NATAL_ONLY_SCOPES: &[ClaimScope] = &[ClaimScope::Natal];

pub(crate) const ALL_SELECTED_SCOPES: &[ClaimScope] = &[
    ClaimScope::Natal,
    ClaimScope::Decadal,
    ClaimScope::Age,
    ClaimScope::Yearly,
    ClaimScope::Monthly,
    ClaimScope::Daily,
    ClaimScope::Hourly,
];

pub(crate) const OVERLAY_AWARE_RULES: &[(&str, &'static [ClaimScope])] =
    &[(CHANG_QU_CLAMP_LIFE, ALL_SELECTED_SCOPES)];

pub(crate) fn applicable_scopes_for_rule_id(rule_id: &str) -> &'static [ClaimScope] {
    OVERLAY_AWARE_RULES
        .iter()
        .find_map(|(overlay_rule_id, scopes)| (*overlay_rule_id == rule_id).then_some(*scopes))
        .unwrap_or(NATAL_ONLY_SCOPES)
}

pub(crate) fn is_overlay_aware_rule(rule_id: &str) -> bool {
    OVERLAY_AWARE_RULES
        .iter()
        .any(|(overlay_rule_id, _)| *overlay_rule_id == rule_id)
}
