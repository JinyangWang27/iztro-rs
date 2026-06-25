//! Loading of the Chinese-first classical rule corpus.
//!
//! The corpus TOML is the authoring source for rule metadata. It is embedded at
//! compile time with `include_str!` (mirroring how `iztro-i18n` embeds its Fluent
//! resources), so loading is deterministic and never touches the filesystem at
//! runtime. Parse failures would only arise from a malformed committed corpus and
//! are caught by the corpus deserialization test.

use std::sync::OnceLock;

use serde::Deserialize;

use crate::rules::classical::rule::ClassicalRule;

/// The embedded 《紫微斗数全书》 pilot corpus.
const QUAN_SHU_TOML: &str = include_str!("../../../rule-corpus/quan-shu/rules.toml");
/// The embedded project pattern/格局/夹宫 corpus. These rules are derived from
/// modeled chart structures, not from a cited QuanShu passage.
const PATTERNS_TOML: &str = include_str!("../../../rule-corpus/patterns/rules.toml");

#[derive(Deserialize)]
struct Corpus {
    #[serde(default)]
    rule: Vec<ClassicalRule>,
}

fn parse(toml_src: &str) -> Vec<ClassicalRule> {
    let corpus: Corpus =
        toml::from_str(toml_src).expect("embedded classical rule corpus must deserialize");
    corpus.rule
}

/// Returns only the 《紫微斗数全书》 rules, parsing the embedded corpus once.
///
/// # Panics
///
/// Panics if the embedded corpus TOML fails to deserialize. This can only happen
/// if the committed corpus is malformed, which the corpus test guards against.
pub fn quan_shu_rules() -> &'static [ClassicalRule] {
    static RULES: OnceLock<Vec<ClassicalRule>> = OnceLock::new();
    RULES.get_or_init(|| parse(QUAN_SHU_TOML)).as_slice()
}

/// Returns only the project pattern/格局-derived rules.
///
/// # Panics
///
/// Panics if the embedded corpus TOML fails to deserialize (see
/// [`quan_shu_rules`]).
pub fn pattern_rules() -> &'static [ClassicalRule] {
    static RULES: OnceLock<Vec<ClassicalRule>> = OnceLock::new();
    RULES.get_or_init(|| parse(PATTERNS_TOML)).as_slice()
}

/// Returns all classical rules used by the engine: QuanShu source rules
/// followed by project pattern-derived rules.
///
/// # Panics
///
/// Panics if either embedded corpus TOML fails to deserialize (see
/// [`quan_shu_rules`]).
pub fn classical_rules() -> &'static [ClassicalRule] {
    static RULES: OnceLock<Vec<ClassicalRule>> = OnceLock::new();
    RULES
        .get_or_init(|| {
            let mut all = quan_shu_rules().to_vec();
            all.extend(pattern_rules().iter().cloned());
            all
        })
        .as_slice()
}

/// Returns the rule with the given id, searching all classical rules.
pub fn rule_by_id(id: &str) -> Option<&'static ClassicalRule> {
    classical_rules().iter().find(|rule| rule.id.as_str() == id)
}
