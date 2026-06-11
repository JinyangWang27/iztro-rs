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

For local upstream inspection, use the pinned npm reference workspace under
`tools/iztro-reference`:
`npm ci --prefix tools/iztro-reference`.
The committed fixture JSON files remain the compatibility source of truth.

## Star-name inventory

`iztro-core` now keeps two separate star metadata surfaces:

- `represented_star_metadata_table().len() == 70` remains strict: it covers only
  the stars currently represented by chart facts, placed by Rust code, and
  validated by fixtures. Four represented adjective stars are algorithm-gated
  and appear only under `ChartAlgorithmKind::Zhongzhou`.
- `known_star_metadata_table().len() == 170` inventories the broader upstream
  `iztro@2.5.8` runtime star-name universe spanning the represented stars,
  decorative runtime arrays
  (`changsheng12`, `boshi12`, `suiqian12`, `jiangqian12`), and horoscope flow
  star names for decadal, yearly, monthly, daily, and hourly scopes.

`represented_star_metadata_table()` stays **natal-only (70)**. Two further
runtime surfaces are now placed without changing that table:

- The four decorative families (长生/博士/岁前/将前十二神) are placed as **untyped**
  `DecorativeStarPlacement`s. They have no upstream `FunctionalStar` type and
  therefore no `StarKind`, so they are never typed `StarPlacement`s and never
  appear in `Chart::stars()`.
- The horoscope flow stars (`Yun*`/`Liu*`/`Yue*`/`Ri*`/`Shi*`) are placed as
  **typed, branch-tagged** `ScopedStarPlacement`s inside a `TemporalLayer`. They
  carry a concrete `StarKind` from the known inventory, but because they are
  temporal they remain outside the natal `represented_star_metadata_table()` —
  they are *known/typed-but-temporal*.

Known metadata still does not imply brightness tables or full horoscope
palace-name derivation. See [Runtime star-family placement](#runtime-star-family-placement).

The upstream locale key `xunzhong` / `旬中` is intentionally excluded because no
built-in upstream `FunctionalStar` construction or `StarType` assignment was
found for it in `iztro@2.5.8`. 四化 remain `Mutagen` /
`MutagenActivation` facts, not `StarName` variants.

## Public facade compatibility

`by_lunar` and `by_solar` are the iztro-compatible facade entry points in
`iztro-rs`. They mirror iztro's `astro.byLunar(...)` and `astro.bySolar(...)`
conceptually, but use the typed `LunarChartRequest` and `SolarChartRequest`
request objects instead of JavaScript-style positional arguments.

`by_lunar` records the provided lunar date as chart input facts and delegates to
the supported-star natal chart builder. It now carries explicit leap-month
semantics through `is_leap_month` and `fix_leap` (builder defaults `false` and
`true`, preserving prior non-leap behavior). The requested `is_leap_month` is
first resolved against the real calendar through the internal ICU-backed
calendar normalizer; no ICU or calendar-adapter types are exposed from the
public API. The leap flag is honored **only** when the requested month is
actually that year's leap month, mirroring upstream `lunar2solar`. An invalid
leap request — for example `2020-3-20` with `is_leap_month=true`, where 2020's
leap month is the fourth, not the third — is treated as the ordinary month.
After resolution, the second half of an actual leap month (lunar day > 15) with
`fix_leap` advances the effective month used for month-based star placement by
one, matching upstream `iztro@2.5.8` `fixLunarMonthIndex`, except when the birth
time is late Zi (`timeIndex = 12`), where upstream keeps the effective month
unchanged. A leap twelfth month is rejected with
`ChartError::UnsupportedLeapMonthCombination` rather than guessed. The birth
year stem and branch are still supplied explicitly to `by_lunar` because
year-to-ganzhi derivation from a lunar year is not implemented there.

Birth time is represented by `BirthTime`, matching upstream `iztro` `timeIndex`
values `0..=12`. `EarlyZi` (`0`) and `LateZi` (`12`) both project to
`EarthlyBranch::Zi`, while branch-based request setters continue to map `Zi` to
early Zi for backward compatibility. Late Zi is fixture-backed against
`iztro@2.5.8`: time-based formulas wrap it like Zi, major-star placement uses
the next lunar day, daily adjective-star placement uses upstream
`fixLunarDayIndex`, and leap-month second-half adjustment is guarded off for
late Zi.

`by_solar` is a minimal adaptor over the same supported slice: it validates the
Gregorian/solar date, converts it to Chinese-lunisolar facts through the internal
`lunar-lite` adapter, derives the birth-year Heavenly Stem and Earthly Branch
from the converted lunar year, sets `is_leap_month` from the conversion and
`fix_leap` from the request, then delegates to `by_lunar`. It performs no chart
construction of its own, so it produces exactly the `by_lunar` supported slice.
`lunar-lite` is used internally only; calendar-backend types are not part of the
public API. The conversion uses the lunar-new-year boundary, matching iztro's
default `yearDivide: 'normal'`, so the converted year ganzhi agrees with
upstream even across the 立春/正月初一 window.

Full BaZi output, full horoscope assembly, temporal decorative arrays
(`yearlyDecStar`), full facade serialization parity, bindings, feature
extraction, rules, and narrative remain deferred.

## Horoscope layer models

`iztro-core` defines model-only horoscope overlays: `HoroscopeChart` wraps an
immutable natal `Chart` and holds zero or more `TemporalLayer`s, each with a
non-natal `Scope`, a typed `TemporalContext`, scoped `StarPlacement`s, and
`MutagenActivation`s. These models carry only temporal facts supplied explicitly
by the caller, and a layer never duplicates natal placements.

A yearly mutagen overlay builder (`build_yearly_mutagen_layer`) is now available
as the first model-level temporal activation builder. Given a natal `Chart` and
an explicit yearly stem-branch / lunar year (`YearlyMutagenLayerInput`), it
produces a `Scope::Yearly` `TemporalLayer` whose `MutagenActivation`s apply the
yearly Heavenly Stem to the represented stars actually present in the natal
chart, reusing the shared Heavenly Stem mutagen table. It derives no calendar
facts, places no flow stars (流曜), does not mutate natal placements, and does
not perform full horoscope interpretation. Absent or unsupported target stars
are skipped rather than invented. 四化 remain `MutagenActivation` facts, not
independent stars.

A decadal mutagen overlay builder (`build_decadal_mutagen_layer`) is now
available as a model-level temporal activation builder alongside the yearly one.
Given a natal `Chart` and an explicit decadal stem-branch plus starting age
(`DecadalMutagenLayerInput`), it produces a `Scope::Decadal` `TemporalLayer`
whose `MutagenActivation`s apply the decadal Heavenly Stem to the represented
stars actually present in the natal chart, reusing the same shared Heavenly Stem
mutagen table. It accepts explicit decadal stem/context facts only: it derives no
age ranges, no 大限命宫, no decadal palace layout, and no calendar facts, places
no flow stars (流曜), does not mutate natal placements, and does not perform full
horoscope interpretation. Absent or unsupported target stars are skipped rather
than invented. For the same Heavenly Stem and natal chart, the decadal and yearly
builders produce the same target-star / target-branch / mutagen triples,
differing only in `source_scope` and `TemporalContext`. 四化 remain
`MutagenActivation` facts, not independent stars.

A scoped flow-star builder (`build_flow_star_layer`) places the horoscope flow
stars (流曜) for one period. Given a `TemporalContext`, it returns a
`TemporalLayer` of branch-tagged `ScopedStarPlacement`s for that scope. The
mutagen builders above still place no flow stars; flow-star placement is this
separate builder. Decadal/yearly derivation, year-to-ganzhi conversion, and
calendar derivation remain out of scope.

## Runtime star-family placement

Typed stars and decorative runtime entries are **separate fact surfaces**, and
`Chart::stars()` returns typed `StarPlacement`s only.

**Decorative families.** The four "twelve gods" families
(长生/博士/岁前/将前十二神) are emitted by upstream as bare names with no
`StarKind`, so they are modelled as untyped `DecorativeStarPlacement`s
(`name` + `DecorativeStarFamily` + `Scope`) rather than fake-typed
`StarPlacement`s. `DecorativeStarPlacement::try_new` validates each entry against
the known inventory (matching family, no `StarKind`). They live in a separate
`Palace::decorative_stars()` collection (serde-skipped when empty) and are read
through `Chart::decorative_stars()` / `Chart::decorative_star()`. The
supported-star natal builder — and therefore `by_lunar` — places all four
families: 长生 starts from the five-element bureau branch and 博士 from 禄存, both
阳男阴女顺行; 岁前 and 将前 advance forward from the year branch / triad anchor.
岁破 (`SuiPo`) is known as a 岁前 name but is not placed as an additional
thirteenth entry because upstream emits exactly 12 岁前 entries. Under
`ChartAlgorithmKind::Zhongzhou`, 岁破 occupies the seventh 岁前 slot in place of
大耗 (`SuiPo` replaces `DaHaoSuiqian`); otherwise 岁破 is known-but-not-placed.
Because decorative entries are separate facts, default/non-Zhongzhou
`Chart::stars()` remains **66** typed natal `StarPlacement`s and Zhongzhou
`Chart::stars()` remains **68** typed natal `StarPlacement`s.

**Zhongzhou natal adjective stars** (`LongDeAdj`, `JieKong`, `JieShaAdj`,
`DaHaoAdj`) remain typed `StarPlacement`s placed under
`ChartAlgorithmKind::Zhongzhou`, and so stay in the represented (natal) table.

**Flow stars.** Flow-star placement is implemented through the normalized
`FlowStarScope` + `FlowStarBase` identity: `flow_star_name(scope, base)` yields
the `StarName`, and `build_flow_star_layer` runs one scope-generic algorithm for
decadal/yearly/monthly/daily/hourly, placing the ten matrix stars
(魁钺昌曲禄羊陀马鸾喜) from the period stem-branch. Flow 文昌文曲 uses the
stem-based rule (distinct from the natal time-based one). The yearly scope also
places 年解 (`NianJieYearly`), which is intentionally kept outside `FlowStarBase`.
No horoscope palace-name derivation is performed; placement is branch-based.

四化 remain `Mutagen` / `MutagenActivation` facts, never `StarName` variants.
Minimal `by_solar` (`lunar-lite`-backed solar-to-lunar conversion), fixture-backed
leap-month behavior, and `BirthTime`/`timeIndex` `0..=12` rat-hour variants for
the supported `by_lunar`/`by_solar` slice are now implemented (see
[Public facade compatibility](#public-facade-compatibility)). Full BaZi output,
the upstream yearly decorative arrays (`yearlyDecStar`), full horoscope
assembly, bindings, feature extraction, rules, and narrative remain deferred.

## Current fixtures

The fixtures are:

- `fixtures/iztro/minimal_natal_1990_05_17_chen_female.json`
- `fixtures/iztro/major_stars_1990_05_17_chen_female.json`
- `fixtures/iztro/minor_stars_1990_05_17_chen_female.json`
- `fixtures/iztro/minor_stars_1988_03_14_zi_male.json`
- `fixtures/iztro/minor_stars_1991_08_09_hai_female.json`
- `fixtures/iztro/adjective_stars_full_default_1990_05_17_chen_female.json`
- `fixtures/iztro/adjective_stars_full_default_1988_03_14_zi_male.json`
- `fixtures/iztro/adjective_stars_full_default_1991_08_09_hai_female.json`
- `fixtures/iztro/zhongzhou_adjective_stars_1990_05_17_chen_female.json`
- `fixtures/iztro/zhongzhou_adjective_stars_1988_03_14_zi_male.json`
- `fixtures/iztro/zhongzhou_adjective_stars_1991_08_09_hai_female.json`
- `fixtures/iztro/runtime_decorative_default_1990_05_17_chen_female.json`
- `fixtures/iztro/runtime_decorative_default_1988_03_14_zi_male.json`
- `fixtures/iztro/runtime_decorative_default_1991_08_09_hai_female.json`
- `fixtures/iztro/runtime_decorative_zhongzhou_1990_05_17_chen_female.json`
- `fixtures/iztro/runtime_decorative_zhongzhou_1988_03_14_zi_male.json`
- `fixtures/iztro/runtime_decorative_zhongzhou_1991_08_09_hai_female.json`
- `fixtures/iztro/flow_stars.json`
- `fixtures/iztro/e2e_supported_by_lunar.json`
- `fixtures/iztro/e2e_supported_by_solar.json`
- `fixtures/iztro/leap_month_by_lunar.json`
- `fixtures/iztro/time_index_rat_hour.json`

The `runtime_decorative_*` fixtures cover the four decorative families per palace
(default and Zhongzhou); `flow_stars.json` covers the scoped flow stars for every
scope across all ten stems and twelve branches. See
[Runtime star-family placement](#runtime-star-family-placement).

`e2e_supported_by_lunar.json` covers current supported `by_lunar` facade facts
for six chart cases under both default and Zhongzhou algorithms, including typed
temporal flow-star placements for explicit stem-branch contexts and yearly-only
`NianJieYearly`. It remains supported-field-only, preserves raw upstream labels
beside normalized keys for diagnosis, and intentionally excludes full facade
serialization parity, calendar conversion, leap-month behavior, rat-hour
variants, horoscope palace-name derivation, temporal decorative arrays, rules,
and narrative.

`e2e_supported_by_solar.json` covers the supported `by_solar` slice for seven
solar cases under both algorithms (fourteen cases): Chinese New Year boundaries,
ordinary dates, a date converting into a leap lunar month, and a date after a
leap month, plus the leap second-half date under both `fix_leap=true` and
`fix_leap=false` (which yield different month-based placement). Each case adds a
`converted_lunar` block (lunar year/month/day, leap flag, birth-year stem/branch)
so calendar mismatches are diagnosable. Regenerate it from the repo root with:

```bash
npm ci --prefix tools/iztro-reference
npm run dump:e2e-supported-by-solar --prefix tools/iztro-reference -- --write
```

`leap_month_by_lunar.json` covers explicit `by_lunar` leap-month behavior using
real 2020 闰四月 dates across the `is_leap_month` and `fix_leap` toggles: before
the leap month, the regular month with the leap-month number, both halves of the
leap month, and a date after it. The leap fourth-month day > 15 pair (`fix_leap`
true vs false) is the discriminator that shows the effective month advancing. It
also covers **invalid** leap requests (`is_leap_month=true` for a month that is
not that year's leap month — third and fifth months of 2020, and an ordinary
2021 month): upstream ignores the flag, and each case records the upstream
`resolved_lunar` block so the Rust test asserts the same resolution rather than
merely echoing input flags. Regenerate it with:

```bash
npm run dump:leap-month --prefix tools/iztro-reference -- --write
```

`time_index_rat_hour.json` covers upstream `iztro` `timeIndex` `0..=12`
behavior for the supported `by_lunar` slice: early Zi (`0`), late Zi (`12`), one
ordinary non-Zi time, and a real leap-month second-half pair proving the late-Zi
guard on effective-month advancement. Regenerate it with:

```bash
npm ci --prefix tools/iztro-reference
npm run dump:time-index-rat-hour --prefix tools/iztro-reference -- --write
```

Both new fixtures are supported-field-only and exclude temporal flow stars (these
depend only on the year stem/branch and are covered by
`e2e_supported_by_lunar.json`), full facade serialization parity, rat-hour
variants, horoscope palace-name derivation, temporal decorative arrays, features,
rules, and narrative. `lunar-lite` backs `by_solar`'s conversion internally and
calendar-backend types do not appear in the public API.

Only the current full default-algorithm adjective-star fixtures (38 stars each)
and Zhongzhou adjective-star fixtures (40 stars each) are kept in-tree. Earlier,
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

### Default-algorithm natal adjective-star set

The three `adjective_stars_full_default_*` fixtures compare the **complete**
default-algorithm set of 38 natal adjective/helper stars (杂曜) against iztro's
per-palace `adjectiveStars`:

- the adjective-star name in each palace;
- the palace branch each star occupies;
- the upstream iztro star `type`, preserved verbatim (`flower`, `adjective`, or
  `helper`) and mapped to the Rust `StarKind`.

The set has four `flower` stars — 红鸾 (HongLuan), 天喜 (TianXi), 天姚
(TianYao), and 咸池 (XianChi); two `helper` stars — 解神 (JieShen) and 年解
(NianJie); and 32 plain `adjective` stars. Placement reproduces iztro 2.5.8
(`getAdjectiveStar` with `getLuanXiIndex`, `getMonthlyStarIndex`,
`getTimelyStarIndex`, `getDailyStarIndex`, `getHuagaiXianchiIndex`, and
`getYearlyStarIndex`), translated from iztro's 寅-based palace frame into branch
offsets, grouped by placement basis:

- **Birth year branch**: 红鸾/天喜 (天喜 sits opposite 红鸾); 龙池/凤阁 and
  天哭/天虚; 华盖, 咸池 (`getHuagaiXianchiIndex` 三合 family), 孤辰, 寡宿, 蜚廉,
  破碎, 天德, 月德, and 年解 (`getYearlyStarIndex`); and 天空 (year branch + 1).
  年解 is represented only as the natal `origin` helper emitted by
  `getAdjectiveStar`; yearly flow 年解 is a separate `NianJieYearly` temporal
  placement.
- **Lunar month**: 天姚/天刑; and 天巫, 天月, 阴煞, and 解神 from fixed per-month
  branch lookups (`getMonthlyStarIndex`).
- **Birth time branch**: 台辅/封诰 (`getTimelyStarIndex`).
- **Placed minor-star anchors + lunar day** (`getDailyStarIndex`): 三台 from the
  placed 左辅 plus the lunar day offset (初一 = 0); 八座 from the placed 右弼
  minus it; 恩光 from the placed 文昌 and 天贵 from the placed 文曲, each plus the
  lunar day offset minus one.
- **Birth year stem**: 天官, 天厨, 天福 (`tian_fu_adj`), 截路, and 空亡 from fixed
  per-stem branch lookups.
- **Life/Body-palace anchored**: 天才 counts forward from the Life Palace and
  天寿 from the Body Palace by the birth year branch index; 天伤 occupies the
  仆役 palace (Life + 5) and 天使 the 疾厄 palace (Life + 7). The default
  algorithm has no 阴阳/gender swap (that swap is Zhongzhou-only).
- **旬空 (旬中空亡)**: the void branch of the birth year's 甲-旬 (sexagenary
  decade) whose 阴阳 polarity matches the birth year branch. iztro computes a
  base palace index then advances one palace on a polarity mismatch; palace and
  branch indices differ by a fixed even offset (寅 = 2), so the rule translates
  directly to branch space. A focused table test covers the rule across all 60
  jiazi.

天福 uses the `tian_fu_adj` key / `StarName::TianFuAdj` to disambiguate from the
major star 天府 (`tian_fu` / `StarName::TianFu`); both romanize to "Tian Fu".
天月 likewise uses `tian_yue_adj` / `StarName::TianYueAdj` to disambiguate from
the minor star 天钺 (`tian_yue` / `StarName::TianYue`). Every placed adjective
star derives `StarCategory::Adjective` (`StarKind::Flower`, `StarKind::Adjective`,
and `StarKind::Helper` all map to it), carries `Brightness::Unknown` and no
四化, and has natal scope. The `build_natal_chart_with_supported_stars` builder
places this set after major and minor stars, so `by_lunar` now yields 14 major +
14 minor + 38 adjective/helper = **66 natal stars**.

This fixture set still does **not** compare adjective-star brightness, feature
extraction, rule-engine output, narrative output, solar-to-lunar conversion,
leap-month behavior, rat-hour variants, or temporal star scopes (大限, yearly, or
other flowing scopes).

### Zhongzhou natal adjective-star set

The three `zhongzhou_adjective_stars_*` fixtures compare the
`ChartAlgorithmKind::Zhongzhou` natal adjective-star output against iztro
2.5.8's Zhongzhou `getAdjectiveStar` behavior. Zhongzhou output keeps the common
default natal adjective/helper stars, does not place the default 截路 (`JieLu`) or
空亡 (`KongWang`) pair, and adds four Zhongzhou-only natal adjective stars:
龙德 (`LongDeAdj`), 截空 (`JieKong`), 劫杀 (`JieShaAdj`), and 大耗
(`DaHaoAdj`). It also follows iztro `getTianshiTianshangIndex`, including the
Zhongzhou-only 天伤/天使 yin-yang/gender swap when applicable.

For Zhongzhou, `by_lunar` / `build_natal_chart_with_supported_stars` now yields
14 major + 14 minor + 40 adjective/helper = **68 natal stars**. The default and
placeholder/non-Zhongzhou profile output remains unchanged at **66 natal stars**.
The represented metadata table includes both default-only and Zhongzhou-only
algorithm-gated natal adjective stars, so it now has **70** represented stars
total: 14 major + 14 minor + 42 natal adjective/helper stars.

This fixture set still does **not** compare decorative runtime arrays,
adjective-star brightness, feature extraction, rule-engine output, narrative
output, solar-to-lunar conversion, leap-month behavior, rat-hour variants,
horoscope placement, or temporal star scopes. 四化 remain `Mutagen` /
`MutagenActivation` facts, not `StarName` variants.

### Adjective/helper star coverage

iztro 2.5.8 `getAdjectiveStar` emits **38** natal-origin 杂曜 under the default
(non-Zhongzhou) algorithm. `iztro-rs` now places **all 38** of them, so
default-algorithm natal adjective/helper star coverage is **complete**. Every
star is natal-origin (`scope: origin`) and is reached from inputs `by_lunar`
already threads — lunar month, lunar day, birth time, birth-year stem,
birth-year branch, and the Life/Body palace branches. None require temporal
layers, solar-to-lunar conversion, leap-month handling, or rat-hour variants.

The Zhongzhou variant `algorithm: 'zhongzhou'` is now supported for the four
Zhongzhou-only natal adjective stars. 神煞 beyond the supported
`getAdjectiveStar` slice, 流曜, and all temporal/horoscope placement remain
deferred. 四化 remain `mutagen: Option<Mutagen>` facts on placements, not
independent stars.

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
