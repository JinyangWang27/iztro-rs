//! Fluent-based localization for `iztro-rs` user interfaces.
//!
//! This crate owns locale parsing, compile-time Fluent resource loading,
//! English-fallback behavior, and typed label helpers that map `iztro` domain
//! enums to their localized display strings. It performs no chart calculation:
//! it only resolves already-derived, language-neutral facts into display text at
//! the presentation boundary.
//!
//! ```
//! use iztro_i18n::{I18n, Locale};
//!
//! let en = I18n::new(Locale::EnUs);
//! assert_eq!(en.text("button-save"), "Save");
//!
//! let zh = I18n::new(Locale::ZhHans);
//! assert_eq!(zh.text("button-save"), "保存");
//! ```

mod keys;
mod locale;
mod localizer;
#[cfg(test)]
mod star_table;

use iztro::LunarDateProjection;
use iztro::core::labels::chinese_date;
use iztro::core::{
    Brightness, EarthlyBranch, FiveElementBureau, Gender, HeavenlyStem, Mutagen, PalaceName,
    PatternPolarity, PatternStatus, Scope, StarName, StemBranch, WesternZodiac,
};
use iztro::rules::classical::{Claim, ClaimDomain, ClaimPolarity, ClaimTheme};

pub use fluent_bundle::FluentArgs;
pub use locale::{Locale, UnsupportedLocale};
pub use localizer::I18n;

/// English double-hour (时辰) branch pinyin and clock range, by `iztro`
/// `timeIndex` (`0..=12`). Early Zi (`0`) and late Zi (`12`) share the Zi branch.
const HOUR_EN: [(&str, &str); 13] = [
    ("Zi", "00:00–01:00"),
    ("Chou", "01:00–03:00"),
    ("Yin", "03:00–05:00"),
    ("Mao", "05:00–07:00"),
    ("Chen", "07:00–09:00"),
    ("Si", "09:00–11:00"),
    ("Wu", "11:00–13:00"),
    ("Wei", "13:00–15:00"),
    ("Shen", "15:00–17:00"),
    ("You", "17:00–19:00"),
    ("Xu", "19:00–21:00"),
    ("Hai", "21:00–23:00"),
    ("Zi", "23:00–24:00"),
];

impl I18n {
    /// Localized natal palace name (宫位).
    pub fn palace_name(&self, name: PalaceName) -> String {
        self.text(&keys::palace_key(name))
    }

    /// Localized star name (星耀). Covers major, minor, adjective, decorative,
    /// and flow stars.
    pub fn star_name(&self, name: StarName) -> String {
        self.text(&keys::star_key(name))
    }

    /// Localized mutagen / four-transformation label (四化).
    pub fn mutagen(&self, mutagen: Mutagen) -> String {
        self.text(&keys::mutagen_key(mutagen))
    }

    /// Localized horoscope scope / temporal layer label (运限).
    pub fn temporal_label(&self, scope: Scope) -> String {
        self.text(&keys::scope_key(scope))
    }

    /// Short temporal label for the compact stepper arrows.
    pub fn temporal_short(&self, scope: Scope) -> String {
        self.text(&format!("{}-short", keys::scope_key(scope)))
    }

    /// Localized scope label for the analysis inspector group headers
    /// (本命 / 大限 / 小限 / 流年 / 流月 / 流日 / 流时).
    ///
    /// These intentionally use dedicated `rules-scope-*` keys rather than
    /// [`temporal_label`](Self::temporal_label): the inspector wants a compact,
    /// uniform set of scope captions (e.g. `本命` / `Natal`) shared by the
    /// 全书规则 and 格局 tabs, independent of the stepper's `大限`/`Decade` wording.
    pub fn analysis_scope_label(&self, scope: Scope) -> String {
        self.text(match scope {
            Scope::Natal => "rules-scope-natal",
            Scope::Decadal => "rules-scope-decadal",
            Scope::Age => "rules-scope-age",
            Scope::Yearly => "rules-scope-yearly",
            Scope::Monthly => "rules-scope-monthly",
            Scope::Daily => "rules-scope-daily",
            Scope::Hourly => "rules-scope-hourly",
        })
    }

    /// Localized pattern fulfilment-status label (成格 / 减力 / 破格).
    pub fn pattern_status_label(&self, status: PatternStatus) -> String {
        self.text(match status {
            PatternStatus::Fulfilled => "patterns-status-fulfilled",
            PatternStatus::Weakened => "patterns-status-weakened",
            PatternStatus::Broken => "patterns-status-broken",
        })
    }

    /// Localized pattern polarity label (吉 / 凶 / 平).
    pub fn pattern_polarity_label(&self, polarity: PatternPolarity) -> String {
        self.text(match polarity {
            PatternPolarity::Auspicious => "pattern-polarity-auspicious",
            PatternPolarity::Inauspicious => "pattern-polarity-inauspicious",
            PatternPolarity::Neutral => "pattern-polarity-neutral",
        })
    }

    /// Localized star brightness (亮度). [`Brightness::Unknown`] renders as the
    /// empty string, mirroring the convention that an uncalculated brightness
    /// carries no label.
    pub fn brightness(&self, brightness: Brightness) -> String {
        if brightness == Brightness::Unknown {
            return String::new();
        }
        self.text(&keys::brightness_key(brightness))
    }

    /// Localized gender marker.
    pub fn gender(&self, gender: Gender) -> String {
        self.text(&keys::gender_key(gender))
    }

    /// Localized Heavenly Stem (天干).
    pub fn stem(&self, stem: HeavenlyStem) -> String {
        self.text(&keys::stem_key(stem))
    }

    /// Localized Earthly Branch (地支).
    pub fn branch(&self, branch: EarthlyBranch) -> String {
        self.text(&keys::branch_key(branch))
    }

    /// Localized stem-branch pair (干支). Simplified Chinese concatenates with no
    /// separator (`癸酉`); English separates the romanized stem and branch with a
    /// space (`Gui You`).
    pub fn stem_branch(&self, stem: HeavenlyStem, branch: EarthlyBranch) -> String {
        match self.locale() {
            Locale::ZhHans => format!("{}{}", self.stem(stem), self.branch(branch)),
            Locale::EnUs => format!("{} {}", self.stem(stem), self.branch(branch)),
        }
    }

    /// Localized stem-branch pair (干支) from a [`StemBranch`] value.
    pub fn stem_branch_value(&self, sb: StemBranch) -> String {
        self.stem_branch(sb.stem(), sb.branch())
    }

    /// Localized Chinese zodiac animal (生肖) for an Earthly Branch.
    pub fn zodiac_animal(&self, branch: EarthlyBranch) -> String {
        self.text(&keys::zodiac_key(branch))
    }

    /// Localized five-element bureau (五行局).
    pub fn bureau(&self, bureau: FiveElementBureau) -> String {
        self.text(&keys::bureau_key(bureau))
    }

    /// Localized Western zodiac sign (星座).
    pub fn constellation(&self, sign: WesternZodiac) -> String {
        self.text(&keys::constellation_key(sign))
    }

    /// Localized soul/body master star name (命主 / 身主 value).
    pub fn master(&self, star: StarName) -> String {
        self.star_name(star)
    }

    /// Localized nominal age (虚岁), such as `Age 16` / `16岁`.
    pub fn nominal_age(&self, age: u16) -> String {
        let mut args = FluentArgs::new();
        args.set("n", age);
        self.text_args("age-label", &args)
    }

    /// Localized lunisolar (农历) date.
    ///
    /// Simplified Chinese reuses the authoritative almanac form
    /// (`一九九三年四月初七`); English renders `Lunar 1993-04-07`, marking a leap
    /// month explicitly.
    pub fn lunar_date(&self, date: &LunarDateProjection) -> String {
        match self.locale() {
            Locale::ZhHans => {
                chinese_date::lunar_date_label(date.year, date.month, date.day, date.is_leap_month)
            }
            Locale::EnUs => {
                let leap = if date.is_leap_month { " (leap)" } else { "" };
                format!(
                    "Lunar {:04}-{:02}-{:02}{leap}",
                    date.year, date.month, date.day
                )
            }
        }
    }

    /// Compact double-hour (时辰) branch label without its clock range, by
    /// `iztro` `timeIndex` (`0..=12`), for chart names and saved-chart metadata.
    ///
    /// Simplified Chinese preserves the existing GUI form (`辰时`, with early/late
    /// Zi distinguished); English uses the branch pinyin.
    pub fn hour_branch(&self, time_index: u8) -> String {
        let label = match self.locale() {
            Locale::ZhHans => match time_index {
                0 => "早子时",
                1 => "丑时",
                2 => "寅时",
                3 => "卯时",
                4 => "辰时",
                5 => "巳时",
                6 => "午时",
                7 => "未时",
                8 => "申时",
                9 => "酉时",
                10 => "戌时",
                11 => "亥时",
                12 => "晚子时",
                _ => "未知",
            },
            Locale::EnUs => match time_index {
                0 => "Early Zi",
                1 => "Chou",
                2 => "Yin",
                3 => "Mao",
                4 => "Chen",
                5 => "Si",
                6 => "Wu",
                7 => "Wei",
                8 => "Shen",
                9 => "You",
                10 => "Xu",
                11 => "Hai",
                12 => "Late Zi",
                _ => "Unknown",
            },
        };
        label.to_owned()
    }

    /// Localized lunisolar (农历) year only, for a 流年-only run-limit selection
    /// that has no concrete day. English `Lunar 2008`; Chinese `二〇〇八年`.
    pub fn lunar_year(&self, year: i32) -> String {
        match self.locale() {
            Locale::ZhHans => format!("{}年", chinese_date::chinese_year_digits(year)),
            Locale::EnUs => format!("Lunar {year}"),
        }
    }

    /// Localized double-hour (时辰) label with its clock range, by `iztro`
    /// `timeIndex` (`0..=12`).
    pub fn double_hour(&self, time_index: u8) -> String {
        match self.locale() {
            Locale::ZhHans => chinese_date::birth_time_label(time_index),
            Locale::EnUs => match HOUR_EN.get(time_index as usize) {
                Some((branch, range)) => format!("{branch} hour ({range})"),
                None => "Unknown".to_owned(),
            },
        }
    }

    /// Localized classical-claim domain label (领域).
    pub fn claim_domain(&self, domain: ClaimDomain) -> String {
        self.text(&keys::claim_domain_key(domain))
    }

    /// Localized classical-claim theme label (主题).
    pub fn claim_theme(&self, theme: ClaimTheme) -> String {
        self.text(&keys::claim_theme_key(theme))
    }

    /// Localized classical-claim polarity label (吉凶).
    pub fn claim_polarity(&self, polarity: ClaimPolarity) -> String {
        self.text(&keys::claim_polarity_key(polarity))
    }

    /// Localized short text for a claim, resolved from its `claim_key`.
    ///
    /// The core crate emits only the stable key and structured facts; the
    /// localized prose lives here, in the Fluent resources.
    pub fn claim_text(&self, claim: &Claim) -> String {
        self.text(&keys::claim_text_key(claim.claim_key()))
    }

    /// Localized short claim text resolved from a bare `claim_key` string.
    ///
    /// A compact rule hit (e.g. `ClassicalRuleHitRef`) carries only the claim
    /// key, not a full [`Claim`]; this resolves its localized prose the same way
    /// [`claim_text`](Self::claim_text) does.
    pub fn claim_text_by_key(&self, claim_key: &str) -> String {
        self.text(&keys::claim_text_key(claim_key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iztro::core::labels::zh_cn;

    #[test]
    fn english_fallback_for_locale_missing_key() {
        // `test-fallback-only` exists only in en-US.
        let zh = I18n::new(Locale::ZhHans);
        assert_eq!(zh.text("test-fallback-only"), "English fallback");
    }

    #[test]
    fn domain_mapping_resolves_in_both_locales() {
        let en = I18n::new(Locale::EnUs);
        let zh = I18n::new(Locale::ZhHans);

        // Palace
        assert_eq!(en.palace_name(PalaceName::Life), "Life");
        assert_eq!(zh.palace_name(PalaceName::Life), "命宫");
        // Star
        assert_eq!(en.star_name(StarName::ZiWei), "Zi Wei");
        assert_eq!(zh.star_name(StarName::ZiWei), "紫微");
        // Mutagen
        assert_eq!(en.mutagen(Mutagen::Lu), "Lu");
        assert_eq!(zh.mutagen(Mutagen::Lu), "禄");
        // Temporal label
        assert_eq!(en.temporal_label(Scope::Decadal), "Decade");
        assert_eq!(zh.temporal_label(Scope::Decadal), "大限");
    }

    #[test]
    fn claim_labels_resolve_in_both_locales() {
        let en = I18n::new(Locale::EnUs);
        let zh = I18n::new(Locale::ZhHans);

        assert_eq!(en.claim_domain(ClaimDomain::Migration), "Migration");
        assert_eq!(zh.claim_domain(ClaimDomain::Migration), "迁移");
        assert_eq!(zh.claim_domain(ClaimDomain::Life), "命身");
        assert_eq!(
            en.claim_theme(ClaimTheme::RestlessMovement),
            "Restless movement"
        );
        assert_eq!(zh.claim_theme(ClaimTheme::RestlessMovement), "奔波迁动");
        assert_eq!(
            en.claim_polarity(ClaimPolarity::MixedNegative),
            "Mixed-negative"
        );
        assert_eq!(zh.claim_polarity(ClaimPolarity::MixedNegative), "偏凶");
    }

    #[test]
    fn every_pilot_claim_text_exists_in_both_locales() {
        let en = I18n::new(Locale::EnUs);
        let zh = I18n::new(Locale::ZhHans);
        for rule in iztro::rules::classical::quan_shu_rules() {
            let Some(claim) = &rule.claim else {
                continue;
            };
            let key = crate::keys::claim_text_key(&claim.claim_key);
            assert!(!en.text(&key).starts_with('!'), "missing en text {key}");
            assert!(!zh.text(&key).starts_with('!'), "missing zh text {key}");
        }
    }

    #[test]
    fn claim_text_renders_from_claim_key() {
        use iztro::rules::classical::{ClaimId, ClaimScope, ClaimStrength, ClassicalRuleId};

        let rule_id = ClassicalRuleId::new("migration.tian_ma_void.restless_movement");
        let claim = Claim {
            id: ClaimId::new(&rule_id, ClaimScope::Natal),
            rule_id,
            domain: ClaimDomain::Migration,
            themes: vec![ClaimTheme::RestlessMovement],
            polarity: ClaimPolarity::MixedNegative,
            strength: ClaimStrength::new(0.6),
            scope: ClaimScope::Natal,
            evidence: Vec::new(),
            counter_evidence: Vec::new(),
            source_refs: Vec::new(),
            claim_key: "claim.migration.tian-ma-void.restless-movement".to_owned(),
        };

        assert_eq!(
            I18n::new(Locale::ZhHans).claim_text(&claim),
            "天马受空亡影响，主奔波迁动之象。"
        );
        assert!(
            I18n::new(Locale::EnUs)
                .claim_text(&claim)
                .starts_with("Tian Ma is affected by a void")
        );
    }

    #[test]
    fn missing_claim_key_returns_visible_placeholder() {
        let zh = I18n::new(Locale::ZhHans);
        assert_eq!(zh.text("claim-nonexistent-key"), "!claim-nonexistent-key!");
    }

    #[test]
    fn composite_helpers_render_per_locale() {
        let en = I18n::new(Locale::EnUs);
        let zh = I18n::new(Locale::ZhHans);
        let date = LunarDateProjection {
            year: 1993,
            month: 4,
            day: 7,
            is_leap_month: false,
        };

        assert_eq!(en.lunar_date(&date), "Lunar 1993-04-07");
        assert_eq!(zh.lunar_date(&date), "一九九三年四月初七");
        assert_eq!(en.double_hour(9), "You hour (17:00–19:00)");
        assert_eq!(zh.double_hour(9), "酉时(17:00~19:00)");
        assert_eq!(en.nominal_age(16), "Age 16");
        assert_eq!(zh.nominal_age(16), "16岁");
        assert_eq!(en.brightness(Brightness::Unknown), "");
        assert_eq!(en.hour_branch(4), "Chen");
        assert_eq!(zh.hour_branch(4), "辰时");
        assert_eq!(en.hour_branch(0), "Early Zi");
        assert_eq!(zh.hour_branch(12), "晚子时");
    }

    #[test]
    fn stem_branch_separates_pinyin_but_not_chinese() {
        // 1993 is the 癸酉 (Gui You) year.
        let gui_you = StemBranch::from_lunar_year(1993);
        assert_eq!(
            I18n::new(Locale::EnUs).stem_branch_value(gui_you),
            "Gui You"
        );
        assert_eq!(I18n::new(Locale::ZhHans).stem_branch_value(gui_you), "癸酉");
        assert_eq!(
            I18n::new(Locale::EnUs).stem_branch(gui_you.stem(), gui_you.branch()),
            "Gui You"
        );
    }

    #[test]
    fn every_star_resolves_and_matches_authoritative_chinese() {
        let en = I18n::new(Locale::EnUs);
        let zh = I18n::new(Locale::ZhHans);
        for (star, en_label, zh_label) in star_table::STARS {
            assert_eq!(en.star_name(*star), *en_label, "en label for {star:?}");
            assert_eq!(zh.star_name(*star), *zh_label, "zh label for {star:?}");
            // zh-Hans must equal the authoritative core table — no drift.
            assert_eq!(zh.star_name(*star), zh_cn::star_name_zh(*star), "{star:?}");
            assert!(!en.star_name(*star).starts_with('!'), "missing en {star:?}");
            assert!(!zh.star_name(*star).starts_with('!'), "missing zh {star:?}");
        }
    }

    #[test]
    fn domain_labels_have_no_drift_against_core_chinese() {
        let zh = I18n::new(Locale::ZhHans);
        for (name, _) in [
            (PalaceName::Life, ()),
            (PalaceName::Siblings, ()),
            (PalaceName::Spouse, ()),
            (PalaceName::Children, ()),
            (PalaceName::Wealth, ()),
            (PalaceName::Health, ()),
            (PalaceName::Migration, ()),
            (PalaceName::Friends, ()),
            (PalaceName::Career, ()),
            (PalaceName::Property, ()),
            (PalaceName::Spirit, ()),
            (PalaceName::Parents, ()),
        ] {
            assert_eq!(
                zh.palace_name(name),
                zh_cn::palace_name_zh(name),
                "{name:?}"
            );
        }
        for mutagen in [Mutagen::Lu, Mutagen::Quan, Mutagen::Ke, Mutagen::Ji] {
            assert_eq!(
                zh.mutagen(mutagen),
                zh_cn::mutagen_zh(mutagen),
                "{mutagen:?}"
            );
        }
        for brightness in [
            Brightness::Temple,
            Brightness::Prosperous,
            Brightness::Advantage,
            Brightness::Favourable,
            Brightness::Flat,
            Brightness::Weak,
            Brightness::Trapped,
        ] {
            assert_eq!(
                zh.brightness(brightness),
                zh_cn::brightness_zh(brightness),
                "{brightness:?}"
            );
        }
        for scope in [
            Scope::Natal,
            Scope::Decadal,
            Scope::Age,
            Scope::Yearly,
            Scope::Monthly,
            Scope::Daily,
            Scope::Hourly,
        ] {
            assert_eq!(
                zh.temporal_label(scope),
                zh_cn::scope_zh(scope),
                "{scope:?}"
            );
        }
    }

    /// Regenerates `locales/{en-US,zh-Hans}/stars.ftl` from [`star_table::STARS`].
    /// Ignored by default; run explicitly after editing the star table:
    /// `cargo test -p iztro-i18n -- --ignored generate_star_ftl`.
    #[test]
    fn pattern_polarity_label_maps_all_variants_in_both_locales() {
        use iztro::core::PatternPolarity;
        let en = I18n::new(Locale::EnUs);
        let zh = I18n::new(Locale::ZhHans);

        assert_eq!(
            en.pattern_polarity_label(PatternPolarity::Auspicious),
            "Auspicious"
        );
        assert_eq!(
            en.pattern_polarity_label(PatternPolarity::Inauspicious),
            "Inauspicious"
        );
        assert_eq!(
            en.pattern_polarity_label(PatternPolarity::Neutral),
            "Neutral"
        );

        assert_eq!(zh.pattern_polarity_label(PatternPolarity::Auspicious), "吉");
        assert_eq!(
            zh.pattern_polarity_label(PatternPolarity::Inauspicious),
            "凶"
        );
        assert_eq!(zh.pattern_polarity_label(PatternPolarity::Neutral), "平");
    }

    #[test]
    fn theme_labels_exist_for_all_three_themes_in_both_locales() {
        let en = I18n::new(Locale::EnUs);
        let zh = I18n::new(Locale::ZhHans);

        assert_eq!(en.text("theme-ink-paper"), "InkPaper");
        assert_eq!(en.text("theme-jade-light"), "JadeLight");
        assert_eq!(en.text("theme-deep-ink"), "DeepInk");

        assert_eq!(zh.text("theme-ink-paper"), "水墨纸笺");
        assert_eq!(zh.text("theme-jade-light"), "青玉明笺");
        assert_eq!(zh.text("theme-deep-ink"), "深墨夜笺");
    }

    #[test]
    fn patterns_detail_polarity_key_exists_in_both_locales() {
        let en = I18n::new(Locale::EnUs);
        let zh = I18n::new(Locale::ZhHans);
        assert_eq!(en.text("patterns-detail-polarity"), "Polarity");
        assert_eq!(zh.text("patterns-detail-polarity"), "吉凶");
    }

    #[test]
    #[ignore]
    fn generate_star_ftl() {
        use std::fmt::Write as _;

        fn write_ftl(header: &str, pick: impl Fn(&(StarName, &str, &str)) -> String) -> String {
            let mut out = String::new();
            out.push_str(header);
            for row in star_table::STARS {
                let _ = writeln!(out, "{} = {}", keys::star_key(row.0), pick(row));
            }
            out
        }

        let en = write_ftl(
            "# Star names (星耀). Generated from star_table.rs — see generate_star_ftl.\n",
            |row| row.1.to_owned(),
        );
        let zh = write_ftl(
            "# 星耀名称。由 star_table.rs 生成——参见 generate_star_ftl。\n",
            |row| row.2.to_owned(),
        );

        let base = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("locales");
        std::fs::write(base.join("en-US/stars.ftl"), en).expect("write en stars.ftl");
        std::fs::write(base.join("zh-Hans/stars.ftl"), zh).expect("write zh stars.ftl");
    }
}
