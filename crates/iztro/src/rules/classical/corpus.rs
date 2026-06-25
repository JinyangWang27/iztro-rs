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

#[derive(Deserialize)]
struct Corpus {
    #[serde(default)]
    rule: Vec<ClassicalRule>,
}

/// Returns the parsed 全书 pilot rules, parsing the embedded corpus once.
///
/// # Panics
///
/// Panics if the embedded corpus TOML fails to deserialize. This can only happen
/// if the committed corpus is malformed, which the corpus test guards against.
pub fn quan_shu_rules() -> &'static [ClassicalRule] {
    static RULES: OnceLock<Vec<ClassicalRule>> = OnceLock::new();
    RULES
        .get_or_init(|| {
            let corpus: Corpus = toml::from_str(QUAN_SHU_TOML)
                .expect("embedded classical rule corpus must deserialize");
            corpus.rule
        })
        .as_slice()
}

/// Returns the rule with the given id, if present in the 全书 corpus.
pub fn rule_by_id(id: &str) -> Option<&'static ClassicalRule> {
    quan_shu_rules().iter().find(|rule| rule.id.as_str() == id)
}
