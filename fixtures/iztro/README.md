# iztro compatibility fixtures

Reference fixtures used to check `iztro-rs` chart output against the upstream
JavaScript implementation.

## Pinned target

- Package: `npm:iztro`
- Version: `2.5.8`

## Upstream reference workspace

Use the pinned local npm workspace when inspecting or regenerating upstream
behavior:

```bash
npm ci --prefix tools/iztro-reference
npm run check:version --prefix tools/iztro-reference
npm run dump:by-lunar --prefix tools/iztro-reference
npm run dump:adjective --prefix tools/iztro-reference
npm run dump:e2e-supported --prefix tools/iztro-reference
npm run dump:e2e-supported-by-solar --prefix tools/iztro-reference
npm run dump:leap-month --prefix tools/iztro-reference
```

The workspace targets `npm:iztro` version `2.5.8` and keeps
`tools/iztro-reference/node_modules/` gitignored. The committed fixture JSON
files remain the compatibility source of truth; these scripts are helper tools
for inspecting and regenerating upstream behavior. The reference workspace does
not change Rust chart-generation logic.

## Generation

Each fixture records its own `metadata.generation_command`. The current command
(from the JSON metadata) is:

```bash
npm install iztro@2.5.8 --prefix /tmp/iztro-fixture && \
cd /tmp/iztro-fixture && \
node --input-type=module -e "import { astro } from 'iztro'; const a = astro.bySolar('1990-5-17', 4, '女', true, 'zh-CN'); console.log(JSON.stringify(a, null, 2));"
```

The raw upstream output is stored under `iztro_output`. The normalized
`supported_fields` block is what the compatibility test asserts against.

## Supported-field-only policy

Fixtures compare **only** fields currently implemented by `iztro-rs`:

- birth time;
- gender;
- life/body palace branches;
- palace branches;
- palace names;
- palace heavenly stems;
- the five-element bureau (五行局);
- fourteen major-star facts in the major-star fixture;
- fourteen supported minor-star facts in the minor-star fixtures;
- the full default-algorithm set of 38 natal adjective/helper-star facts in the
  adjective-star fixtures;
- the four decorative runtime families in runtime-family fixtures;
- typed temporal flow-star facts in flow-star fixtures;
- the current combined supported `by_lunar` fields in the E2E fixture.

`metadata.supported_fields_only` is `true`.

## Facade coverage

`iztro-core::by_lunar` and `iztro-core::by_solar` are the iztro-compatible public
facade entry points. They mirror iztro's `astro.byLunar(...)` and
`astro.bySolar(...)` conceptually through the typed `LunarChartRequest` and
`SolarChartRequest`, not JavaScript-style positional arguments.

`by_lunar` records the provided lunar date and delegates to the natal chart with
supported stars builder. It now carries explicit `is_leap_month` / `fix_leap`
semantics: a leap month with `fix_leap` and lunar day > 15 advances the effective
month used for month-based placement. `by_solar` validates the Gregorian date,
converts it through an internal ICU4X (`icu_calendar`) adapter, derives the
birth-year stem/branch from the cyclic year, and delegates to `by_lunar`; ICU4X
types are not exposed in the public API.

`by_lunar` still does not perform year-to-ganzhi derivation, so its fixtures
provide `birth_year_stem` and `birth_year_branch` explicitly; `by_solar` derives
them from the conversion. Rat-hour variants remain deferred.

## Supported by_lunar E2E fixture

`e2e_supported_by_lunar.json` is a compact supported-field-only regression
fixture for the current public `by_lunar` facade. It contains six ordinary
non-leap lunar chart cases, each generated under the default and Zhongzhou
algorithms, for 12 by_lunar E2E cases total.

Each case records:

- the explicit lunar facade inputs used by Rust;
- `algorithm` (`default` or `zhongzhou`);
- life and body palace branches;
- five-element bureau;
- all 12 palace branch/stem/name facts;
- represented typed natal stars with `name`, `kind`, `brightness`, and
  `mutagen`;
- the four decorative runtime families per palace;
- typed temporal flow-star placements for decadal, yearly, monthly, daily, and
  hourly scopes from explicit stem-branch contexts.

The fixture preserves raw upstream labels next to normalized keys for diagnosis,
for example `raw_branch`, `raw_name`, `raw_kind`, `raw_brightness`, and
family-specific decorative raw fields such as `raw_suiqian12`. Rust tests assert
the normalized fields only.

The Rust E2E test builds each case through
`iztro_core::by_lunar(LunarChartRequest::builder()...)`, maps `default` to
`ChartAlgorithmKind::QuanShu` and `zhongzhou` to
`ChartAlgorithmKind::Zhongzhou`, then compares the normalized supported fields
branch by branch. It also asserts the current inventory and count boundaries:
66 default typed natal stars, 68 Zhongzhou typed natal stars, 48 decorative
runtime entries, yearly-only `NianJieYearly` outside `FlowStarBase`,
`known_star_metadata_table().len() == 170`, and
`represented_star_metadata_table().len() == 70`.

Regenerate it from the repo root with:

```bash
npm ci --prefix tools/iztro-reference
npm run dump:e2e-supported --prefix tools/iztro-reference -- --write
```

This fixture intentionally excludes full facade serialization parity, raw
upstream `iztro_output`, calendar conversion, leap-month behavior, rat-hour
variants, horoscope palace-name derivation, temporal decorative arrays, feature
extraction, rule-engine output, and narrative output.

### Palace heavenly stems

Palace stems are generated by `iztro-rs` from `birth_year_stem` using the
classical 起五行寅例 (the rule that fixes the Yin palace stem from the birth
year stem), then counting forward one Heavenly Stem per Earthly Branch. The
fixture stores the expected stems both as upstream `heavenlyStem` characters
under `iztro_output.palaces` and as normalized snake_case keys under
`supported_fields.palaces[].stem`.

### Five-element bureau

`supported_fields.five_element_bureau` is compared against iztro's
`fiveElementsClass`. For this fixture iztro reports `火六局`, which maps to the
`iztro-rs` `fire6` bureau. The Life Palace stem-branch pair (`己丑`, Ji-Chou)
has the NaYin element Fire, and Fire maps to the Fire 6 bureau.

### Explicit birth year stem

`input.birth_year_stem` is supplied explicitly because Gregorian-to-ganzhi year
conversion (and full solar/lunar calendar conversion) is deferred. For 1990 the
ganzhi year is `庚午`, so the year stem is `geng`. Once year-stem derivation
from a Gregorian date exists, this field can be derived instead of provided.

### Explicit birth year branch

`input.birth_year_branch` is supplied explicitly in supported-star fixtures
because Gregorian-to-ganzhi year conversion is deferred. Minor-star placement
uses the year branch for 天马 and 火星/铃星.

### Explicitly excluded fields

- stars
- brightness
- mutagens
- decadal scopes
- yearly scopes
- narrative output

## Supported by_solar E2E fixture

`e2e_supported_by_solar.json` is a supported-field-only regression fixture for the
public `by_solar` facade. It contains seven solar chart cases, each generated
under the default and Zhongzhou algorithms (14 cases total), spanning Chinese New
Year boundaries, ordinary non-leap dates, a date that converts into a leap lunar
month (second half, so the effective month advances), and a date after a leap
month. The leap second-half date appears twice — once with `fix_leap=true` and
once with `fix_leap=false` — and the two produce different month-based placement.

Each case records:

- the solar facade inputs used by Rust (`solar_year`/`solar_month`/`solar_day`,
  `birth_time`, `gender`, `fix_leap`), with `fix_leap` read from the fixture (not
  hardcoded);
- a `converted_lunar` block with the lunar year/month/day, leap flag, and
  birth-year stem/branch that upstream derived (from `rawDates.lunarDate` and
  `rawDates.chineseDate.yearly`), to diagnose calendar mismatches;
- the supported chart fields (life/body palace branches, five-element bureau,
  palace branch/stem/name facts, typed natal stars, and the four decorative
  runtime families), with counts.

The Rust E2E test (`crates/iztro-core/tests/e2e_supported_by_solar.rs`) builds
each case through `iztro_core::by_solar(SolarChartRequest::builder()...)`, asserts
the ICU-converted lunar year/month/day recorded on the chart equals
`converted_lunar`, and compares the supported fields. The converted leap flag and
birth-year ganzhi are covered by the ICU adapter's own unit tests and, end to
end, by the palace-stem and minor-star comparisons that depend on them.

Regenerate it from the repo root with:

```bash
npm ci --prefix tools/iztro-reference
npm run dump:e2e-supported-by-solar --prefix tools/iztro-reference -- --write
```

## Leap-month by_lunar fixture

`leap_month_by_lunar.json` characterizes explicit `by_lunar` leap-month behavior
using real 2020 闰四月 lunar dates across the `is_leap_month` and `fix_leap`
toggles: a date before the leap month, the regular month with the leap-month
number, both halves of the leap month, and a date after it. The leap fourth-month
day > 15 pair (`fix_leap` true vs false) is the discriminator — only `fix_leap`
true advances the effective month, so the life palace and bureau diverge; the day
≤ 15 leap case matches the regular month. It also includes **invalid** leap
requests — `is_leap_month=true` for the third and fifth months of 2020 (whose
leap month is the fourth) and for an ordinary 2021 month — which upstream resolves
back to ordinary, non-leap months.

Each case records the lunar facade inputs (including `is_leap_month`, `fix_leap`,
and the upstream-derived `birth_year_stem`/`birth_year_branch` fed back to Rust),
a `resolved_lunar` block (the lunar date upstream resolved to via `lunar2solar`),
and the supported chart fields. The Rust E2E test
(`crates/iztro-core/tests/leap_month_by_lunar.rs`) builds each case through
`iztro_core::by_lunar(...)` with the leap flags set, compares the supported
fields, and asserts the chart's recorded lunar date reproduces the upstream
`resolved_lunar` block. The resolved leap flag remains covered by internal
calendar unit tests and by fixture parity for palace/star placement.

Regenerate it with:

```bash
npm run dump:leap-month --prefix tools/iztro-reference -- --write
```

Both fixtures are supported-field-only and exclude temporal flow stars (covered by
`e2e_supported_by_lunar.json`), full facade serialization parity, rat-hour
variants, horoscope palace-name derivation, temporal decorative arrays, features,
rules, and narrative.

## Major-star fixture

`major_stars_1990_05_17_chen_female.json` covers the fourteen major stars
(主星) for the same birth case. It is generated by capturing iztro's per-palace
`majorStars`:

```bash
npm install iztro@2.5.8 --prefix /tmp/iztro-fixture && \
cd /tmp/iztro-fixture && \
node --input-type=module -e "import { astro } from 'iztro'; const a = astro.bySolar('1990-5-17', 4, '女', true, 'zh-CN'); console.log(JSON.stringify(a.palaces.map(p => ({ earthlyBranch: p.earthlyBranch, majorStars: p.majorStars })), null, 2));"
```

`supported_fields.major_stars` lists, per palace branch, normalized star fact
objects with:

- `name`: the snake_case `StarName` key, for example `zi_wei`, `tian_ji`, or
  `po_jun`;
- `brightness`: the normalized `Brightness` key, including separate
  `advantage` (`得`) and `favourable` (`利`) values;
- `mutagen`: the normalized birth-year mutagen key (`lu`, `quan`, `ke`, `ji`)
  or `null` when the represented major star has no birth-year mutagen.

In `iztro-rs`, placed stars use a two-level classification model:

- `StarKind` stores the iztro-compatible fine type (`major`, `soft`, `tough`,
  `lucun`, `tianma`, `adjective`, `flower`, or `helper`);
- `StarCategory` is derived from `StarKind` as the coarse palace grouping
  (`major`, `minor`, or `adjective`);
- `mutagen: Option<Mutagen>` stores 四化 state separately and is not represented
  as a star kind or category.

The compatibility test asserts, for every palace, that the major-star names,
palace branches, brightness values, and supported birth-year mutagens placed by
`iztro-rs` equal iztro's. The test builds through the public
`build_natal_chart_with_major_stars` path so the fixture covers integration from
the minimal natal chart builder into deterministic major-star facts.

### Explicit lunar day

`input.lunar_day` (23) is supplied explicitly because full calendar conversion
is deferred. It selects the 紫微 position relative to the five-element bureau.

### Explicit birth year stem

`input.birth_year_stem` (`geng`) is supplied explicitly because full
Gregorian-to-ganzhi year conversion is deferred. It selects the supported
built-in birth-year mutagens attached to represented major stars in the natal
chart.

### Explicitly excluded fields

- feature extraction
- rule-engine output
- narrative output
- calendar conversion
- minor stars
- adjective stars
- non-major stars
- non-major mutagens
- decadal scopes
- yearly scopes
- other temporal scopes

## Minor-star fixtures

The minor-star fixtures cover the fourteen supported natal minor stars (辅星):

- `minor_stars_1990_05_17_chen_female.json`
- `minor_stars_1988_03_14_zi_male.json`
- `minor_stars_1991_08_09_hai_female.json`

They are generated from `npm:iztro@2.5.8` in `/tmp` and capture compact raw
`palaces[].minorStars` output alongside normalized
`supported_fields.minor_stars`.

The normalized star fact objects include:

- `name`: the snake_case `StarName` key, for example `zuo_fu`,
  `wen_chang`, or `tian_ma`;
- `kind`: the iztro-compatible fine kind (`soft`, `tough`, `lucun`, or
  `tianma`);
- `brightness`: the normalized `Brightness` key; stars without upstream
  brightness tables use `unknown`;
- `mutagen`: the normalized birth-year mutagen key or `null`.

The compatibility tests assert placement, kind, brightness, and represented
minor birth-year mutagens. The implemented minor-star inputs are explicit
`lunar_month`, `birth_time`, `birth_year_stem`, and `birth_year_branch`. Gender
and five-element bureau are not used by iztro minor-star placement.

### Explicitly excluded fields

- adjective stars
- flower/helper/adjective subsets
- feature extraction
- rule-engine output
- reading or narrative output
- solar-to-lunar conversion
- leap-month behavior
- rat-hour variants
- temporal star scopes
- CLI bindings
- Python bindings
- WebAssembly bindings

## Adjective-star fixtures

The adjective-star fixtures cover the **full default-algorithm** natal
adjective-star (杂曜) set — all 38 natal-origin 杂曜 iztro 2.5.8
`getAdjectiveStar` emits under the default (non-Zhongzhou) algorithm, plus the
Zhongzhou natal adjective-star output (40 stars each). These adjective-star
fixtures are kept in-tree:

- `adjective_stars_full_default_1990_05_17_chen_female.json`
- `adjective_stars_full_default_1988_03_14_zi_male.json`
- `adjective_stars_full_default_1991_08_09_hai_female.json`
- `zhongzhou_adjective_stars_1990_05_17_chen_female.json`
- `zhongzhou_adjective_stars_1988_03_14_zi_male.json`
- `zhongzhou_adjective_stars_1991_08_09_hai_female.json`

Earlier, smaller adjective-star subsets (the six-, twelve-, eighteen-, and
twenty-six-star fixtures) are no longer kept in-tree; their history remains
available through git history. Each fixture's `metadata` records the
`target_version` (`2.5.8`), `algorithm` (`default` or `zhongzhou`), and
`adjective_star_count` (`38` or `40`).

They are generated from `npm:iztro@2.5.8` via `astro.byLunar(...)` and capture
compact raw `palaces[].adjectiveStars` alongside normalized
`supported_fields.adjective_stars`.

The normalized star fact objects include:

- `name`: the snake_case `StarName` key, for example `hong_luan`,
  `tian_yao`, or `feng_gao`. 天福 uses `tian_fu_adj` to disambiguate from the
  major star 天府 (`tian_fu`); 天月 uses `tian_yue_adj` to disambiguate from the
  minor star 天钺 (`tian_yue`);
- `type`: the upstream iztro star type, preserved verbatim (`flower`,
  `adjective`, or `helper`) and mapped to the Rust `StarKind` by the
  compatibility test.

The default fixture set has four `flower` stars (红鸾, 天喜, 天姚, 咸池), two
`helper` stars (解神, 年解), and 32 plain `adjective` stars. The compatibility
tests assert placement, upstream type, derived `StarCategory::Adjective`,
`Brightness::Unknown`, and natal scope. The implemented adjective-star inputs are
explicit `lunar_month`, `lunar_day`, `birth_time`, `birth_year_stem`, and
`birth_year_branch`. 三台/八座 derive from the actual placed 左辅/右弼 branches
and 恩光/天贵 from the placed 文昌/文曲 branches plus the lunar-day offset, so
these fixtures are exercised after minor stars have been placed; 天才/天寿/天伤/
天使 read the Life and Body palaces. 年解 is covered only as the natal `origin`
helper emitted by `getAdjectiveStar`, not as a horoscope/yearly flow.

The Zhongzhou fixtures use `astro.config({ algorithm: "zhongzhou" })`. They
assert the upstream Zhongzhou behavior exactly: default 截路/空亡 are absent,
龙德/截空/劫杀/大耗 are present as natal adjective stars, and 天伤/天使 follow
the Zhongzhou yin-yang/gender swap from `getTianshiTianshangIndex` when
applicable. Default/non-Zhongzhou `by_lunar` output remains 66 natal stars; the
Zhongzhou profile outputs 68 natal stars.

### Explicitly excluded fields

- adjective-star brightness
- decorative runtime arrays
- feature extraction
- rule-engine output
- reading or narrative output
- solar-to-lunar conversion
- leap-month behavior
- rat-hour variants
- temporal star scopes
- CLI bindings
- Python bindings
- WebAssembly bindings

## Runtime star-family fixtures

These cover the runtime star families upstream emits beyond the typed natal
stars: the four decorative "twelve gods" families and the scoped flow stars.

Decorative natal families (长生/博士/岁前/将前十二神), per palace, for both
algorithms:

- `runtime_decorative_default_1990_05_17_chen_female.json`
- `runtime_decorative_default_1988_03_14_zi_male.json`
- `runtime_decorative_default_1991_08_09_hai_female.json`
- `runtime_decorative_zhongzhou_1990_05_17_chen_female.json`
- `runtime_decorative_zhongzhou_1988_03_14_zi_male.json`
- `runtime_decorative_zhongzhou_1991_08_09_hai_female.json`

Each `supported_fields.decorative_stars[]` entry records a palace `branch` and
the single snake_case `StarName` key per family (`changsheng12`, `boshi12`,
`suiqian12`, `jiangqian12`). These are **untyped** decorative facts: upstream
emits bare names with no `StarKind`, so the Rust model places them as
`DecorativeStarPlacement`s that never appear in `Chart::stars()`. The seventh
岁前 entry is `da_hao_suiqian` (大耗) under the default algorithm and `sui_po`
(岁破) under Zhongzhou — `岁破` replaces `大耗`, it is not supplemental.

Scoped flow stars (流耀):

- `flow_stars.json`

Generated from `getHoroscopeStar(stem, branch, scope)` for every scope
(decadal/yearly/monthly/daily/hourly) across all ten Heavenly Stems and twelve
Earthly Branches (60 cases). Each case lists the ten matrix stars
(魁钺昌曲禄羊陀马鸾喜) by `base` and `branch` plus the upstream `type`; the yearly
scope additionally records the `nian_jie_branch` for 年解 (`NianJieYearly`), which
is kept outside the flow-star matrix. The Rust model places these as typed,
branch-tagged `ScopedStarPlacement`s inside a `TemporalLayer`. Palace index 0 is
寅.

All runtime star-family fixtures are regenerated by
`tools/iztro-reference/scripts/dump-runtime-star-families.mjs`.

The supported E2E fixture is regenerated by
`tools/iztro-reference/scripts/dump-e2e-supported-fixtures.mjs`.

## Scope

The fixtures cover **minimal natal compatibility** and deterministic
**fourteen-major-star**, **fourteen-supported-minor-star**, the **full
default-algorithm 38 natal-adjective/helper-star facts**, the **Zhongzhou 40
natal-adjective/helper-star facts**, runtime star-family placement, and the
combined supported `by_lunar` E2E fixture. Default `by_lunar` output remains 66
natal stars (14 major + 14 minor + 38 adjective/helper); Zhongzhou profile
output is 68 natal stars (14 major + 14 minor + 40 adjective/helper). The
represented metadata table is 70 because it includes both default-only and
Zhongzhou-only algorithm-gated adjective stars. Typed flow stars remain
known/typed-but-temporal and outside represented natal metadata.
