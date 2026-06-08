# Compatibility Policy

`iztro-rs` is inspired by `iztro` and should initially validate chart-generation behavior against `iztro` where applicable.

## What compatibility means

Compatibility means:

- selected chart-generation outputs should match `iztro` golden fixtures;
- differences should be documented;
- public Rust models should preserve the same conceptual chart facts where possible;
- tests should make compatibility explicit rather than implicit.

## What compatibility does not mean

Compatibility does not require:

- identical internal architecture;
- identical public API names;
- identical string-based data representation;
- identical narrative or interpretation output;
- support for every `iztro` feature in the first release.

## Compatibility target

The current compatibility target is:

- `iztro` npm package version `2.5.8`.

Future compatibility fixtures may update this target only when the version
change and expected output differences are documented.

## Public facade compatibility

`by_lunar` is the first iztro-compatible facade entry point in `iztro-rs`. It
mirrors iztro's `astro.byLunar(...)` conceptually, but uses the typed
`LunarChartRequest` request object instead of JavaScript-style positional
arguments.

The facade records the provided lunar date as chart input facts, delegates to
the supported-star natal chart builder, and does not perform solar-to-lunar
conversion. The birth year stem and branch remain explicit because
Gregorian/lunar year-to-ganzhi derivation is deferred.

`by_solar`, leap-month handling, rat-hour variants, and full calendar behavior
remain deferred.

## Horoscope layer models

`iztro-core` defines model-only horoscope overlays: `HoroscopeChart` wraps an
immutable natal `Chart` and holds zero or more `TemporalLayer`s, each with a
non-natal `Scope`, a typed `TemporalContext`, scoped `StarPlacement`s, and
`MutagenActivation`s. These models carry only temporal facts supplied explicitly
by the caller, and a layer never duplicates natal placements.

Temporal star placement, decadal/yearly derivation, year-to-ganzhi conversion,
and calendar derivation remain out of scope. These models are not yet validated
against `iztro` horoscope fixtures.

## Current fixtures

The fixtures are:

- `fixtures/iztro/minimal_natal_1990_05_17_chen_female.json`
- `fixtures/iztro/major_stars_1990_05_17_chen_female.json`
- `fixtures/iztro/minor_stars_1990_05_17_chen_female.json`
- `fixtures/iztro/minor_stars_1988_03_14_zi_male.json`
- `fixtures/iztro/minor_stars_1991_08_09_hai_female.json`
- `fixtures/iztro/adjective_stars_fourth_subset_1990_05_17_chen_female.json`
- `fixtures/iztro/adjective_stars_fourth_subset_1988_03_14_zi_male.json`
- `fixtures/iztro/adjective_stars_fourth_subset_1991_08_09_hai_female.json`

Only the latest/current adjective-star fixture subset is kept in-tree. Earlier,
smaller adjective-star subsets are available through git history.

The minimal-natal fixture compares only fields currently implemented by
`iztro-rs`:

- birth time;
- gender;
- life palace branch;
- body palace branch;
- palace branches;
- palace names;
- palace heavenly stems;
- five-element bureau (五行局).

Palace heavenly stems are generated from the birth year stem via the classical
起五行寅例 and compared against iztro's per-palace `heavenlyStem`. The
five-element bureau is compared against iztro's `fiveElementsClass` (`火六局`
maps to the `fire6` bureau).

The birth year stem is currently provided explicitly in the fixture input
(`birth_year_stem`) because Gregorian-to-ganzhi year conversion is deferred.

It intentionally does not compare stars, brightness, mutagens, decadal scopes,
yearly scopes, or narrative output.

### Fourteen major stars

The `major_stars_1990_05_17_chen_female.json` fixture compares represented
facts for the fourteen major stars (主星) against iztro's per-palace
`majorStars`:

- the major-star name in each palace;
- the palace branch each star occupies;
- each major star's brightness;
- supported birth-year mutagens for represented major stars.

Placement reproduces iztro 2.5.8 (`getStartIndex` and `getMajorStar`): 紫微 is
derived from the five-element bureau and the lunar day, 天府 is its reflection
across the 寅–申 axis, and the 紫微 and 天府 series fan out by fixed offsets.
Every placed major star has `StarKind::Major`, derived `StarCategory::Major`,
and scope `natal` (iztro `origin`).
Brightness reproduces iztro 2.5.8 `STARS_INFO` for the fourteen represented
major stars, preserving `de` (`得`) as `advantage` and `li` (`利`) as
`favourable`. Birth-year mutagens reproduce iztro 2.5.8 Heavenly Stem mutagens
only where the target star is one of the represented fourteen major stars.

Star classification uses a two-level model. `StarKind` stores the
iztro-compatible fine type (`major`, `soft`, `tough`, `lucun`, `tianma`,
`adjective`, `flower`, or `helper`). `StarCategory` is a derived coarse palace
grouping: `major`, `minor`, or `adjective`. 四化 remains separate factual state
as `mutagen: Option<Mutagen>` on a placement; it is not encoded as either a
star kind or a category.

The lunar day is supplied explicitly (`input.lunar_day`) because full calendar
conversion is deferred. The public
`build_natal_chart_with_major_stars` builder path is what the compatibility
test exercises: it first builds the minimal natal chart, then uses the derived
five-element bureau, explicit lunar day, and explicit birth year stem to place
the fourteen major stars and attach supported factual star state. This fixture
still does **not** compare feature extraction, rule-engine output, narrative
output, calendar conversion, minor stars, adjective stars, non-major stars,
non-major mutagens, decadal scopes, yearly scopes, or other temporal scopes.

### Fourteen supported minor stars

The three `minor_stars_*` fixtures compare represented facts for the fourteen
supported natal minor stars (辅星) against iztro's per-palace `minorStars`:

- the minor-star name in each palace;
- the palace branch each star occupies;
- iztro-compatible star kind (`soft`, `tough`, `lucun`, or `tianma`);
- brightness when iztro 2.5.8 has a brightness table for that star;
- supported birth-year mutagens for represented minor targets.

Placement reproduces iztro 2.5.8 Yin-index formulas:

- 左辅/右弼 from the explicit lunar month;
- 文昌/文曲 and 地空/地劫 from the birth time branch;
- 天魁/天钺 and 禄存/擎羊/陀罗 from the birth year stem;
- 天马 and 火星/铃星 from the birth year branch, with 火星/铃星 also using
  the birth time branch.

Every placed supported minor star has derived `StarCategory::Minor` and natal
scope. `StarKind` preserves the iztro-compatible fine kind: `soft`, `tough`,
`lucun`, or `tianma`. iztro has brightness tables for 文昌, 文曲, 火星, 铃星,
擎羊, and 陀罗; the other supported minor stars use `Brightness::Unknown`.
Birth-year mutagens now use a general represented-star table and include minor
targets where iztro has them: 丙文昌科, 戊右弼科, 己文曲忌, 辛文曲科/文昌忌,
and 壬左辅科. The previous major-only mutagen API remains as a wrapper that
returns values only for represented major stars.

The public `build_natal_chart_with_supported_stars` builder path first builds
the minimal natal chart, then places the fourteen major stars, then places the
fourteen supported minor stars. The `by_lunar` facade delegates to this
supported-star builder and requires explicit `birth_year_stem` and
`birth_year_branch`.

These fixtures still do **not** compare adjective stars,
flower/helper/adjective subsets, feature extraction, rule-engine output,
reading or narrative output, solar-to-lunar conversion, leap-month behavior,
rat-hour variants, temporal star scopes, CLI bindings, Python bindings, or
WebAssembly bindings.

### Supported adjective-star subset

The three `adjective_stars_fourth_subset_*` fixtures compare the currently
supported subset of twenty-six natal adjective/helper stars (杂曜) against iztro's
per-palace `adjectiveStars`:

- the adjective-star name in each palace;
- the palace branch each star occupies;
- the upstream iztro star `type`, preserved verbatim (`flower`, `adjective`, or
  `helper`) and mapped to the Rust `StarKind`.

The subset is 红鸾 (HongLuan), 天喜 (TianXi), and 天姚 (TianYao) —
peach-blossom `flower` stars; 天刑 (TianXing), 台辅 (TaiFu), 封诰 (FengGao),
三台 (SanTai), 八座 (BaZuo), 龙池 (LongChi), 凤阁 (FengGe), 天哭 (TianKu),
天虚 (TianXu), 恩光 (EnGuang), 天贵 (TianGui), 天巫 (TianWu), 天月
(TianYueAdj), 阴煞 (YinSha), 华盖 (HuaGai), 孤辰 (GuChen), 寡宿 (GuaSu),
蜚廉 (FeiLian), 破碎 (PoSui), 天德 (TianDe), and 月德 (YueDe) — plain
`adjective` stars; and 解神 (JieShen) and 年解 (NianJie), `helper` stars.
Placement reproduces iztro 2.5.8 (`getAdjectiveStar` with
`getLuanXiIndex`, `getMonthlyStarIndex`, `getTimelyStarIndex`,
`getDailyStarIndex`, and `getYearlyStarIndex`), translated from iztro's
寅-based palace frame into branch offsets:

- 红鸾/天喜 from the birth year branch (天喜 sits opposite 红鸾);
- 天姚/天刑 from the lunar month;
- 台辅/封诰 from the birth time branch;
- 三台 from the placed 左辅 plus the lunar day offset (初一 = 0);
- 八座 from the placed 右弼 minus the lunar day offset;
- 龙池/凤阁 and 天哭/天虚 from birth-year-branch offsets;
- 恩光 from the placed 文昌 and 天贵 from the placed 文曲, each plus the lunar
  day offset minus one (`getDailyStarIndex`);
- 天巫, 天月, 阴煞, and 解神 from fixed per-lunar-month branch lookups
  (`getMonthlyStarIndex`).
- 华盖, 孤辰, 寡宿, 蜚廉, 破碎, 天德, 月德, and 年解 from the explicit birth
  year branch (`getYearlyStarIndex`). 年解 is represented only as the natal
  `origin` helper emitted by `getAdjectiveStar`; yearly horoscope flow remains
  deferred.

天月 uses the `tian_yue_adj` key / `StarName::TianYueAdj` to disambiguate from
the minor star 天钺 (`tian_yue` / `StarName::TianYue`); both romanize to
"Tian Yue". Every placed adjective star derives `StarCategory::Adjective`
(`StarKind::Flower`, `StarKind::Adjective`, and `StarKind::Helper` all map to
it), carries `Brightness::Unknown` and no 四化, and has natal scope. 解神 is the
first represented `helper`-kind star, and 年解 is the second. The
`build_natal_chart_with_supported_stars` builder places this subset after major
and minor stars, so `by_lunar` now yields 14 major + 14 minor + 26
adjective/helper = 54 natal stars.

This fixture set still does **not** compare the remaining unsupported adjective stars,
adjective-star brightness, feature extraction, rule-engine output, narrative
output, solar-to-lunar conversion, leap-month behavior, rat-hour variants, or
temporal star scopes (大限, yearly, or other flowing scopes).

## Golden tests

Golden tests should include:

- solar date chart generation;
- lunar date chart generation;
- leap-month behavior;
- early and late rat hour behavior;
- year boundary behavior;
- default algorithm behavior;
- Zhongzhou behavior if supported.

## Acknowledgement

`iztro` is licensed under the MIT License. Any directly adapted logic should keep proper attribution in source comments or documentation where appropriate.
