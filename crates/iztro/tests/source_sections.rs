//! Runtime section-metadata (卷/节) lookup for QuanShu source ids.
//!
//! `rules::source::source_section` resolves a source-inventory id to its
//! section metadata through the embedded runtime table
//! `rule-corpus/quan-shu/sections.toml`. The full source inventory
//! (`rule-corpus/quan-shu/source/volume-*.toml`) stays test-only; these tests
//! keep the small runtime table in sync with it, including the assumptions
//! that let the runtime table stay lean: `anchor` always equals `section`, and
//! `doc_path` is derivable from the volume number.

mod support;

use std::collections::HashSet;

use iztro::rules::pattern::metadata::pattern_source_metadata;
use iztro::rules::pattern::model::PatternId;
use iztro::rules::source::{ClassicalWork, source_section};
use iztro::rules::{classical_rules, quan_shu_rules};
use support::classical_source::{QUAN_SHU_WORK, source_inventory};

// ---- A. Direct lookup behavior --------------------------------------------

#[test]
fn known_source_ids_resolve_to_section_metadata() {
    let tai_wei_fu = source_section("quan_shu.v01.tai_wei_fu.ma_yu_kong_wang")
        .expect("known volume-01 source id must resolve");
    assert_eq!(tai_wei_fu.work, ClassicalWork::ZiWeiDouShuQuanShu);
    assert_eq!(tai_wei_fu.volume, 1);
    assert_eq!(tai_wei_fu.section, "太微赋");

    let zhu_xing = source_section("quan_shu.v03.zhu_xing_tong_yuan.zi_fu_jia_ming")
        .expect("known volume-03 source id must resolve");
    assert_eq!(zhu_xing.work, ClassicalWork::ZiWeiDouShuQuanShu);
    assert_eq!(zhu_xing.volume, 3);
    assert_eq!(zhu_xing.section, "论诸星同垣各司所宜分别富贵贫贱夭寿");
}

#[test]
fn unknown_and_pattern_catalog_ids_resolve_to_none() {
    assert!(source_section("quan_shu.v99.no_such_section.key").is_none());
    assert!(source_section("no_dots_at_all").is_none());
    // Project pattern-catalog provenance has no classical section metadata.
    for rule in classical_rules() {
        if rule.work == ClassicalWork::IztroPatternCatalog {
            assert!(
                source_section(&rule.source_id).is_none(),
                "pattern-catalog id {} must not resolve to a QuanShu section",
                rule.source_id
            );
        }
    }
}

// ---- B. Runtime table stays in sync with the source inventory -------------

#[test]
fn every_inventory_item_resolves_to_its_group_section() {
    // Also proves prefix truncation is safe for every real id: if an item key
    // contained a `.`, the truncated prefix would miss and this test fails.
    let inventory = source_inventory();
    assert!(!inventory.source_item.is_empty());
    for item in &inventory.source_item {
        let section = source_section(&item.source_id).unwrap_or_else(|| {
            panic!("inventory id {} must resolve to a section", item.source_id)
        });
        assert_eq!(
            serde_json::to_value(section.work).unwrap(),
            item.work,
            "work mismatch for {}",
            item.source_id
        );
        assert_eq!(section.volume, item.volume, "volume mismatch for {}", item.source_id);
        assert_eq!(section.section, item.section, "section mismatch for {}", item.source_id);
    }
}

#[test]
fn inventory_anchor_and_doc_path_stay_derivable() {
    // The runtime table intentionally stores neither `anchor` nor `doc_path`:
    // `anchor` duplicates `section`, and `doc_path` is derived from the volume.
    // If the inventory ever breaks these invariants, the runtime table must
    // grow the field back — this test is the tripwire.
    for item in source_inventory().source_item {
        if item.is_pending() {
            // Pending units (`anchor = "TODO"` / `section = "待校"`) are not yet
            // located in the Markdown volumes; the invariants apply once located.
            continue;
        }
        assert_eq!(
            item.anchor, item.section,
            "anchor must equal section for {}",
            item.source_id
        );
        assert_eq!(
            item.doc_path,
            format!("docs/zh-CN/sources/quan_shu/volume-{:02}.md", item.volume),
            "doc_path must be derivable from volume for {}",
            item.source_id
        );
    }
}

#[test]
fn runtime_table_has_no_stale_or_duplicate_prefixes() {
    // Reverse direction of the sync guard: every runtime-table prefix must
    // still exist in the inventory. Parsed directly from the corpus file so
    // the runtime crate does not need to expose table iteration.
    #[derive(serde::Deserialize)]
    struct RawSections {
        section: Vec<RawSection>,
    }
    #[derive(serde::Deserialize)]
    struct RawSection {
        source_id_prefix: String,
        work: String,
    }

    let raw: RawSections =
        toml::from_str(include_str!("../rule-corpus/quan-shu/sections.toml"))
            .expect("sections.toml must deserialize");
    assert!(!raw.section.is_empty());

    let inventory_prefixes: HashSet<String> = source_inventory()
        .source_item
        .iter()
        .map(|item| {
            let cut = item.source_id.rfind('.').expect("source ids contain '.'");
            item.source_id[..=cut].to_string()
        })
        .collect();

    let mut seen = HashSet::new();
    for section in &raw.section {
        assert_eq!(section.work, QUAN_SHU_WORK);
        assert!(
            seen.insert(section.source_id_prefix.clone()),
            "duplicate prefix {} in sections.toml",
            section.source_id_prefix
        );
        assert!(
            inventory_prefixes.contains(&section.source_id_prefix),
            "sections.toml prefix {} has no inventory group",
            section.source_id_prefix
        );
    }
}

// ---- C. Every runtime citation consumer can resolve its section -----------

#[test]
fn every_quan_shu_runtime_rule_resolves_to_a_section() {
    for rule in quan_shu_rules() {
        assert_eq!(rule.work, ClassicalWork::ZiWeiDouShuQuanShu);
        assert!(
            source_section(&rule.source_id).is_some(),
            "rule {} source_id {} must resolve to a section",
            rule.id.as_str(),
            rule.source_id
        );
    }
}

#[test]
fn every_source_backed_pattern_resolves_to_a_section() {
    let mut resolved = 0;
    for pattern in PatternId::ALL {
        let Some(metadata) = pattern_source_metadata(pattern) else {
            continue;
        };
        let section = source_section(metadata.source_id).unwrap_or_else(|| {
            panic!(
                "{pattern:?} source_id {} must resolve to a section",
                metadata.source_id
            )
        });
        assert_eq!(section.work, metadata.work);
        resolved += 1;
    }
    assert!(resolved > 0, "expected at least one source-backed pattern");
}
