//! Test-only validation for the 《紫微斗数全书》 source inventory.
//!
//! The source inventory under `rule-corpus/quan-shu/source/` is
//! **corpus-management data**, not runtime chart-evaluation data: nothing in
//! `src/` parses it, and `evaluate_classical` never depends on it. These tests
//! exist only to keep the inventory internally consistent and correctly linked
//! to the executable rule corpus (`rule-corpus/quan-shu/rules.toml`).
//!
//! Model: a `source_item` is one atomic cited QuanShu source unit (a
//! rule-candidate aphorism) identified by a stable mnemonic `source_id`; it
//! links to zero or more rules via `linked_rule_ids`, and each linked rule
//! mirrors that link via its own `source_id`. `source_order` preserves source
//! order separately from stable identity.
//!
//! The deserialization shapes live in the shared, test-only `support` module
//! (`tests/support/classical_source.rs`). They are intentionally not exported
//! from the crate; adding runtime APIs purely to validate corpus tracking data
//! would blur the layer boundary.

mod support;

use std::collections::HashSet;
use support::classical_source::{
    QUAN_SHU_WORK, pattern_rules_corpus, rules_corpus, source_inventory, strip_punct,
};

const TAI_WEI_FU_PREFIX: &str = "quan_shu.v01.tai_wei_fu.";
const SOURCE_BACKED_PATTERN_SECTIONS: [(&str, &str, usize); 4] = [
    ("quan_shu.v01.ding_fu_ju.", "定富局", 6),
    ("quan_shu.v01.ding_gui_ju.", "定贵局", 27),
    ("quan_shu.v01.ding_pin_jian_ju.", "定贫贱局", 8),
    ("quan_shu.v01.ding_za_ju.", "定杂局", 8),
];

// ---- A. Inventory parses and source ids are unique -----------------------

#[test]
fn quan_shu_source_inventory_parses() {
    let inventory = source_inventory();
    assert!(
        !inventory.source_item.is_empty(),
        "source inventory must record at least one source item"
    );
}

#[test]
fn quan_shu_source_inventory_has_unique_source_ids() {
    let inventory = source_inventory();
    let mut seen = HashSet::new();
    for item in &inventory.source_item {
        assert!(
            seen.insert(item.source_id.as_str()),
            "duplicate source_id in inventory: {}",
            item.source_id
        );
    }
}

// ---- B. Required source-item fields are non-empty ------------------------

#[test]
fn quan_shu_source_inventory_required_fields_are_not_empty() {
    let inventory = source_inventory();
    for item in &inventory.source_item {
        // `anchor = "TODO"` and `section = "待校"` are intentionally allowed in
        // the pilot slice, so we only require non-empty, not non-placeholder.
        for (field, value) in [
            ("source_id", &item.source_id),
            ("work", &item.work),
            ("section", &item.section),
            ("category", &item.category),
            ("status", &item.status),
            ("doc_path", &item.doc_path),
            ("anchor", &item.anchor),
            ("source_text_zh_hans", &item.source_text_zh_hans),
        ] {
            assert!(
                !value.trim().is_empty(),
                "source item {} has empty {field}",
                item.source_id
            );
        }
    }
}

// ---- C. source_order is continuous and increasing for Vol.1 太微赋 -------

/// `source_order` preserves the source order of the 例曰 aphorisms. For the
/// Volume 1 太微赋 slice it must start at 1, have no duplicates, and—when sorted
/// by `source_order`—run 1..=N with no gaps.
#[test]
fn tai_wei_fu_source_order_is_continuous() {
    let inventory = source_inventory();
    let mut orders: Vec<usize> = inventory
        .source_item
        .iter()
        .filter(|item| item.volume == 1 && item.section == "太微赋")
        .map(|item| item.source_order)
        .collect();
    assert!(
        !orders.is_empty(),
        "expected 太微赋 source items in the inventory"
    );

    let n = orders.len();
    orders.sort_unstable();
    orders.dedup();
    assert_eq!(orders.len(), n, "太微赋 source_order values must be unique");
    for (idx, order) in orders.iter().enumerate() {
        assert_eq!(
            *order,
            idx + 1,
            "太微赋 source_order must be continuous 1..=N; missing or misnumbered at position {}",
            idx + 1
        );
    }

    // The first aphorism in source order is the opening 例曰 rule candidate.
    let first = inventory
        .source_item
        .iter()
        .filter(|item| item.volume == 1 && item.section == "太微赋")
        .min_by_key(|item| item.source_order)
        .expect("太微赋 source items present");
    assert_eq!(
        first.category, "aphorism_rule",
        "first 太微赋 source item must be the first 例曰 rule-candidate aphorism"
    );
    assert!(
        first.source_text_zh_hans.starts_with("禄逢冲破"),
        "first 太微赋 source item must be the first 例曰 rule-candidate aphorism, got {:?}",
        first.source_text_zh_hans
    );
}

// ---- D. Source ids are stable mnemonic ids -------------------------------

/// 太微赋 source ids are stable mnemonics, not fragile numeric-only ids: they
/// share the `quan_shu.v01.tai_wei_fu.` prefix and their final segment is not
/// purely numeric (so `.001`-style ids are rejected).
#[test]
fn tai_wei_fu_source_ids_are_stable_mnemonics() {
    let inventory = source_inventory();
    for item in &inventory.source_item {
        if item.volume != 1 || item.section != "太微赋" {
            continue;
        }
        let suffix = item
            .source_id
            .strip_prefix(TAI_WEI_FU_PREFIX)
            .unwrap_or_else(|| panic!("unexpected 太微赋 source_id format: {}", item.source_id));
        assert!(
            !suffix.is_empty(),
            "太微赋 source_id {} has an empty mnemonic segment",
            item.source_id
        );
        assert!(
            suffix.chars().any(|c| !c.is_ascii_digit()),
            "太微赋 source_id {} must be a stable mnemonic, not a purely numeric id",
            item.source_id
        );
    }
}

// ---- D2. Source-backed pattern catalogues --------------------------------

#[test]
fn source_backed_pattern_sections_are_segmented() {
    let inventory = source_inventory();

    for (prefix, section, expected_len) in SOURCE_BACKED_PATTERN_SECTIONS {
        let items: Vec<_> = inventory
            .source_item
            .iter()
            .filter(|item| item.volume == 1 && item.section == section)
            .collect();

        assert_eq!(
            items.len(),
            expected_len,
            "{section} must have one source item per named pattern entry"
        );

        for item in items {
            assert!(
                item.source_id.starts_with(prefix),
                "{section} item {} must use prefix {prefix}",
                item.source_id
            );
            assert_eq!(item.category, "pattern_rule");
            assert_eq!(item.status, "segmented");
            assert_eq!(item.doc_path, "docs/zh-CN/sources/quan_shu/volume-01.md");
            assert_eq!(item.anchor, section);
            assert!(
                !item.source_text_zh_hans.trim().is_empty(),
                "{section} item {} has empty source_text_zh_hans",
                item.source_id
            );
            assert!(
                !item.source_text_zh_hans.ends_with('。'),
                "{section} item {} must not include final sentence punctuation",
                item.source_id
            );

            let suffix = item
                .source_id
                .strip_prefix(prefix)
                .unwrap_or_else(|| panic!("unexpected source id {}", item.source_id));
            assert!(
                suffix.chars().any(|c| !c.is_ascii_digit()),
                "{section} item {} must use a stable mnemonic key",
                item.source_id
            );
        }
    }
}

#[test]
fn source_backed_pattern_section_orders_are_continuous() {
    let inventory = source_inventory();

    for (_, section, expected_len) in SOURCE_BACKED_PATTERN_SECTIONS {
        let mut orders: Vec<usize> = inventory
            .source_item
            .iter()
            .filter(|item| item.volume == 1 && item.section == section)
            .map(|item| item.source_order)
            .collect();
        orders.sort_unstable();

        let expected: Vec<usize> = (1..=expected_len).collect();
        assert_eq!(
            orders, expected,
            "{section} source_order must be section-local and continuous"
        );
    }
}

#[test]
fn pattern_rule_category_is_accepted_without_classical_rule_links() {
    let inventory = source_inventory();
    let pattern_items: Vec<_> = inventory
        .source_item
        .iter()
        .filter(|item| item.category == "pattern_rule")
        .collect();

    assert!(
        !pattern_items.is_empty(),
        "expected source-backed pattern inventory items"
    );
    for item in pattern_items {
        assert_eq!(
            item.status, "segmented",
            "pattern-rule source item {} must stay segmented until linked to executable pattern metadata",
            item.source_id
        );
        assert!(
            item.linked_rule_ids.is_empty(),
            "pattern-rule source item {} must not link to classical claim rules",
            item.source_id
        );
    }
}

// ---- E. Linked ids by status ---------------------------------------------

/// Every `rule_linked` source item must carry at least one linked rule. The
/// model still permits future `raw` / `segmented` items with empty
/// `linked_rule_ids`, but `rule_linked` items may not regress to empty.
#[test]
fn rule_linked_source_items_have_links() {
    let inventory = source_inventory();
    for item in &inventory.source_item {
        if item.status == "rule_linked" {
            assert!(
                !item.linked_rule_ids.is_empty(),
                "rule_linked source item {} has no linked_rule_ids",
                item.source_id
            );
        }
    }
}

// ---- F. Every QuanShu rule's source_id exists in the inventory -----------

#[test]
fn classical_rules_reference_known_source_items() {
    let inventory = source_inventory();
    let rules = rules_corpus();
    let source_ids: HashSet<&str> = inventory
        .source_item
        .iter()
        .map(|item| item.source_id.as_str())
        .collect();

    for rule in &rules.rule {
        assert!(
            source_ids.contains(rule.source_id.as_str()),
            "rule {} references source_id {} not present in the source inventory",
            rule.id,
            rule.source_id
        );
    }
}

// ---- G. Every linked rule id exists in rules.toml ------------------------

#[test]
fn source_inventory_linked_rule_ids_exist() {
    let inventory = source_inventory();
    let rules = rules_corpus();
    let rule_ids: HashSet<&str> = rules.rule.iter().map(|r| r.id.as_str()).collect();

    for item in &inventory.source_item {
        for linked in &item.linked_rule_ids {
            assert!(
                rule_ids.contains(linked.as_str()),
                "source item {} links to unknown rule id {linked}",
                item.source_id
            );
        }
    }
}

// ---- H. Linked source item and rule agree on source_id and work ----------

#[test]
fn source_inventory_links_match_rules() {
    let inventory = source_inventory();
    let rules = rules_corpus();

    for item in &inventory.source_item {
        for linked in &item.linked_rule_ids {
            let rule = rules
                .rule
                .iter()
                .find(|r| r.id == *linked)
                .unwrap_or_else(|| {
                    panic!(
                        "source item {} links to unknown rule id {linked}",
                        item.source_id
                    )
                });
            assert_eq!(
                rule.source_id, item.source_id,
                "rule {} source_id {} disagrees with linking source item {}",
                rule.id, rule.source_id, item.source_id
            );
            assert_eq!(
                rule.work, item.work,
                "rule {} work {} disagrees with linking source item {}",
                rule.id, rule.work, item.source_id
            );
        }
    }
}

// ---- I. Source item text matches the linked rule's source text -----------

/// A linked source item must quote the same source unit as its rule: their text
/// must be **equal** after light punctuation normalization (`strip_punct`). A
/// generic `notes_zh_hans` does NOT bypass this — interpretation/paraphrase must
/// never diverge from the cited source. If a genuine source variant arises
/// later, add an explicit opt-in field (e.g. `source_text_variant_ok = true`)
/// rather than relaxing this equality.
#[test]
fn source_item_text_matches_rule_source_text() {
    let inventory = source_inventory();
    let rules = rules_corpus();

    for item in &inventory.source_item {
        let item_text = strip_punct(&item.source_text_zh_hans);
        for linked in &item.linked_rule_ids {
            let rule = rules
                .rule
                .iter()
                .find(|r| r.id == *linked)
                .expect("linked rule must exist");
            let rule_text = strip_punct(&rule.source_text_zh_hans);
            assert_eq!(
                item_text, rule_text,
                "source item {} text {:?} does not match linked rule {} source text {:?}",
                item.source_id, item.source_text_zh_hans, rule.id, rule.source_text_zh_hans
            );
        }
    }
}

// ---- J. 天马空亡 cites the imported wording ------------------------------

/// The 天马空亡 source item must use the imported Volume 1 太微赋 wording
/// "马遇空亡，终身奔走".
#[test]
fn tian_ma_void_source_uses_imported_wording() {
    const CANONICAL: &str = "马遇空亡，终身奔走";
    const RULE_ID: &str = "migration.tian_ma_void.restless_movement";

    let inventory = source_inventory();
    let item = inventory
        .source_item
        .iter()
        .find(|item| item.linked_rule_ids.iter().any(|id| id == RULE_ID))
        .unwrap_or_else(|| panic!("no source item links to {RULE_ID}"));

    assert_eq!(
        item.source_text_zh_hans, CANONICAL,
        "source item linking {RULE_ID} must use imported wording {CANONICAL:?}"
    );
}

// ---- K. 禄马交驰 cites the QuanShu source wording ------------------------

/// Regression: 禄马最喜交驰 must quote the actual QuanShu 太微赋 source unit,
/// not a later interpretation or another tradition's wording. The phrase
/// "发财远方" does not appear in the current QuanShu source inventory and must
/// not be stored as `source_text_zh_hans` anywhere in the QuanShu corpus.
#[test]
fn lu_ma_jiao_chi_uses_quan_shu_source_wording() {
    const RULE_ID: &str = "fortune.lu_ma_jiao_chi.favorable_convergence";
    const SOURCE_TEXT: &str = "禄马最喜交驰";

    let rules = rules_corpus();
    let rule = rules
        .rule
        .iter()
        .find(|r| r.id == RULE_ID)
        .unwrap_or_else(|| panic!("missing rule {RULE_ID}"));
    assert_eq!(rule.source_text_zh_hans, SOURCE_TEXT);

    // No QuanShu rule may carry the later "发财远方" wording as source text.
    for rule in &rules.rule {
        assert!(
            !rule.source_text_zh_hans.contains("发财远方"),
            "rule {} stores non-source interpretation '发财远方' as source_text_zh_hans",
            rule.id
        );
    }

    // The source item links the rule with the faithful source wording.
    let inventory = source_inventory();
    let item = inventory
        .source_item
        .iter()
        .find(|item| item.linked_rule_ids.iter().any(|id| id == RULE_ID))
        .unwrap_or_else(|| panic!("no source item links to {RULE_ID}"));
    assert_eq!(item.source_text_zh_hans, SOURCE_TEXT);
    assert_eq!(item.linked_rule_ids, vec![RULE_ID.to_string()]);
}

// ---- L. 日月反背 cites the actual source wording -------------------------

/// Regression: `life.ri_yue_fan_bei.hardship_pressure` must cite the actual
/// 太微赋 source unit 日月最嫌反背, not the interpreted phrasing 日月反背，劳碌辛苦.
/// The interpretation 劳碌辛苦 may live in `normalized_note_zh_hans` or i18n
/// claim text, but never as the QuanShu `source_text_zh_hans`.
#[test]
fn ri_yue_fan_bei_uses_quan_shu_source_wording() {
    const RULE_ID: &str = "life.ri_yue_fan_bei.hardship_pressure";
    const SOURCE_TEXT: &str = "日月最嫌反背";
    const INTERPRETATION: &str = "劳碌辛苦";

    let rules = rules_corpus();
    let rule = rules
        .rule
        .iter()
        .find(|r| r.id == RULE_ID)
        .unwrap_or_else(|| panic!("missing rule {RULE_ID}"));
    assert_eq!(rule.source_text_zh_hans, SOURCE_TEXT);
    assert!(
        !rule.source_text_zh_hans.contains(INTERPRETATION),
        "rule {RULE_ID} stores interpretation {INTERPRETATION:?} as source_text_zh_hans"
    );

    let inventory = source_inventory();
    let item = inventory
        .source_item
        .iter()
        .find(|item| item.linked_rule_ids.iter().any(|id| id == RULE_ID))
        .unwrap_or_else(|| panic!("no source item links to {RULE_ID}"));
    assert_eq!(item.source_text_zh_hans, SOURCE_TEXT);
}

// ---- QuanShu-only provenance --------------------------------------------

/// The QuanShu source inventory validates against QuanShu rules only: every
/// rule in `rule-corpus/quan-shu/rules.toml` must carry the QuanShu `work`.
/// Pattern/格局-derived rules live in `rule-corpus/patterns/` instead.
#[test]
fn quan_shu_corpus_rules_are_all_quan_shu_work() {
    let rules = rules_corpus();
    for rule in &rules.rule {
        assert_eq!(
            rule.work, QUAN_SHU_WORK,
            "rule {} in the QuanShu corpus has non-QuanShu work {:?}",
            rule.id, rule.work
        );
    }
}

/// The project pattern catalog is kept separate from the QuanShu source
/// inventory. Pattern rules exist, never carry the QuanShu `work`, and their
/// `source_id`s are project `pattern.*` ids rather than QuanShu inventory ids —
/// so they are not required to appear in the inventory. Conversely, the 羊陀夹命 /
/// 昌曲夹命 rules that were moved out must not re-enter the inventory: neither
/// their old `quan_shu.pending.*` source ids nor their rule ids may appear.
#[test]
fn pattern_catalog_is_separate_from_quan_shu_source_inventory() {
    const REMOVED_SOURCE_IDS: [&str; 2] = [
        "quan_shu.pending.yang_tuo_jia_ming",
        "quan_shu.pending.chang_qu_jia_ming",
    ];
    const PATTERN_RULE_IDS: [&str; 2] = [
        "life.yang_tuo_clamp_life.constraint_damage",
        "life.chang_qu_clamp_life.literary_reputation",
    ];

    let inventory = source_inventory();
    let patterns = pattern_rules_corpus();
    assert!(
        !patterns.rule.is_empty(),
        "pattern rule corpus should contain the moved 夹宫 rules"
    );

    let inventory_source_ids: HashSet<&str> = inventory
        .source_item
        .iter()
        .map(|item| item.source_id.as_str())
        .collect();

    // Pattern rules are not QuanShu-work and not QuanShu source-inventory ids.
    for rule in &patterns.rule {
        assert_ne!(
            rule.work, QUAN_SHU_WORK,
            "pattern rule {} must not carry the QuanShu work",
            rule.id
        );
        assert!(
            !inventory_source_ids.contains(rule.source_id.as_str()),
            "pattern rule {} source_id {} must not be a QuanShu source-inventory id",
            rule.id,
            rule.source_id
        );
    }

    // The moved 夹宫 rules must not re-enter the QuanShu source inventory, by
    // either their old pending source ids or their pattern rule ids.
    for item in &inventory.source_item {
        assert!(
            !REMOVED_SOURCE_IDS.contains(&item.source_id.as_str()),
            "removed pattern source_id {} is still in the QuanShu source inventory",
            item.source_id
        );
        for linked in &item.linked_rule_ids {
            assert!(
                !PATTERN_RULE_IDS.contains(&linked.as_str()),
                "source item {} still links pattern rule {linked} in the QuanShu source inventory",
                item.source_id
            );
        }
    }
}

// ---- Tai Wei Fu normalization-map completeness --------------------------

/// Every 太微赋 rule-candidate source item is linked to at least one runtime
/// rule.
///
/// "Complete 太微赋" means no useful source item is left unlinked: each item
/// either links to a normalized/ambiguous/executable rule, or—when it is not a
/// runtime rule candidate (e.g. the section's closing remark)—links to a
/// `rejected` rule that documents the exclusion. This test fails if any 太微赋
/// source item regresses to an empty `linked_rule_ids`.
#[test]
fn tai_wei_fu_source_items_are_all_linked() {
    let inventory = source_inventory();
    let mut unlinked = Vec::new();
    for item in &inventory.source_item {
        if item.volume != 1 || item.section != "太微赋" {
            continue;
        }
        if item.linked_rule_ids.is_empty() {
            unlinked.push(item.source_id.clone());
        }
    }
    assert!(
        unlinked.is_empty(),
        "太微赋 normalization map is incomplete; unlinked source items: {unlinked:?}"
    );
}

/// Every non-executable rule (`normalized` / `ambiguous` / `rejected`) carries a
/// `normalized_note_zh_hans` explaining what the source unit means and why it is
/// not executable yet (or, for `rejected`, why it is not a runtime rule
/// candidate).
#[test]
fn non_executable_rules_have_normalized_notes() {
    let rules = rules_corpus();
    for rule in &rules.rule {
        if !matches!(
            rule.status.as_str(),
            "normalized" | "ambiguous" | "rejected"
        ) {
            continue;
        }
        let note = rule.normalized_note_zh_hans.as_deref().unwrap_or("");
        assert!(
            !note.trim().is_empty(),
            "rule {} (status {}) must have a non-empty normalized_note_zh_hans",
            rule.id,
            rule.status
        );
    }
}

/// Every `executable` QuanShu rule must be one the evaluator actually wires a
/// predicate for. The evaluator lives in `src/` and cannot be called from this
/// corpus-governance test, so we pin the wired set explicitly: marking a rule
/// `executable` in the corpus without adding (and listing) its evaluator branch
/// fails here, keeping the `executable` status honest.
#[test]
fn executable_quan_shu_rules_are_wired_in_the_evaluator() {
    // Rule ids with a hand-coded predicate branch in
    // `src/rules/classical/evaluator.rs`. Pattern-catalog executables
    // (羊陀夹命 / 昌曲夹命) are validated separately and are not QuanShu rules.
    const WIRED_EXECUTABLE: [&str; 5] = [
        "migration.tian_ma_void.restless_movement",
        "life.ri_yue_fan_bei.hardship_pressure",
        "relationship.tan_ju_hai_zi.water_romance",
        "relationship.xing_yu_tan_lang.romance_with_penalty",
        "fortune.shan_fu_ju_kong.monastic_life",
    ];
    let rules = rules_corpus();
    for rule in &rules.rule {
        if rule.status == "executable" {
            assert!(
                WIRED_EXECUTABLE.contains(&rule.id.as_str()),
                "rule {} is marked executable but is not wired in the evaluator; \
                 add an evaluator branch and list it in WIRED_EXECUTABLE",
                rule.id
            );
        }
    }
}
