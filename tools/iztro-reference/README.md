# iztro reference workspace

Pinned local npm workspace for inspecting upstream `iztro@2.5.8` behavior.

Target package: `npm:iztro`
Pinned version: `2.5.8`

This workspace is tooling only. It does not change Rust chart-generation logic,
does not vendor `iztro`, and does not replace committed compatibility fixtures
as the source of truth.

## Install

```bash
npm ci --prefix tools/iztro-reference
```

`tools/iztro-reference/node_modules/` is intentionally gitignored.

## Commands

```bash
npm run check:version --prefix tools/iztro-reference
npm run dump:by-lunar --prefix tools/iztro-reference
npm run dump:adjective --prefix tools/iztro-reference
npm run dump:e2e-supported --prefix tools/iztro-reference
npm run dump:e2e-supported-by-solar --prefix tools/iztro-reference
npm run dump:leap-month --prefix tools/iztro-reference
npm run dump:time-index-rat-hour --prefix tools/iztro-reference
npm run dump:horoscope --prefix tools/iztro-reference
```

The dump commands use the canonical lunar fixture case:

- lunar date: `1990-5-17`
- time index: `4`
- gender: `女`
- leap month: `false`
- fix leap: `true`
- language: `zh-CN`

`dump:by-lunar` prints compact chart JSON with palace branches and upstream
star arrays. `dump:adjective` prints `palaces[].adjectiveStars` plus the total
default-algorithm natal adjective/helper-star count.

### Supported by_lunar E2E fixture

```bash
# inspect
npm run dump:e2e-supported --prefix tools/iztro-reference

# regenerate crates/iztro/fixtures/iztro/e2e_supported_by_lunar.json from repo root
npm run dump:e2e-supported --prefix tools/iztro-reference -- --write
```

`dump:e2e-supported` emits one compact supported-field-only fixture with six
ordinary lunar chart cases under both the default and Zhongzhou algorithms (12
by_lunar E2E cases). It normalizes only the current Rust-supported facts:
life/body palace branches, five-element bureau, palace branch/stem/name facts,
typed natal stars, the four decorative runtime families, and typed temporal
flow-star placements for explicit stem-branch contexts. It preserves raw
upstream labels next to normalized keys for diagnosis, but intentionally does
not snapshot full facade serialization parity, calendar conversion, leap-month
behavior, rat-hour variants, horoscope palace-name derivation, temporal
decorative arrays, features, rules, or narrative output.

### Supported by_solar E2E fixture

```bash
# inspect
npm run dump:e2e-supported-by-solar --prefix tools/iztro-reference

# regenerate crates/iztro/fixtures/iztro/e2e_supported_by_solar.json from repo root
npm run dump:e2e-supported-by-solar --prefix tools/iztro-reference -- --write
```

`dump:e2e-supported-by-solar` emits one supported-field-only `bySolar` fixture
with seven solar cases under both algorithms (14 cases): Chinese New Year
boundaries, ordinary non-leap dates, a date converting into a leap lunar month,
a date after a leap month, and the leap second-half date under both `fix_leap=true`
and `fix_leap=false` (`fix_leap` is per-case input, not hardcoded). Each case adds
a `converted_lunar` block (lunar year/month/day, leap flag, birth-year ganzhi)
derived from upstream `rawDates.lunarDate` and `rawDates.chineseDate.yearly`, so
calendar mismatches are diagnosable. It preserves raw upstream labels beside
normalized keys and excludes temporal flow stars, full facade serialization
parity, rat-hour variants, horoscope palace-name derivation, temporal decorative
arrays, features, rules, and narrative.

### Leap-month by_lunar fixture

```bash
# inspect
npm run dump:leap-month --prefix tools/iztro-reference

# regenerate crates/iztro/fixtures/iztro/leap_month_by_lunar.json from repo root
npm run dump:leap-month --prefix tools/iztro-reference -- --write
```

`dump:leap-month` emits a `byLunar` fixture that characterizes leap-month behavior
using real 2020 闰四月 lunar dates across the `isLeapMonth` and `fixLeap` toggles.
The leap fourth-month day > 15 pair (`fixLeap` true vs false) is the discriminator
for the effective-month advance. It also includes invalid leap requests
(`isLeapMonth=true` for a non-leap month/year), which upstream resolves back to
ordinary months; each case records the upstream `resolved_lunar` block.

Shared normalization maps/helpers for these two generators live in
`scripts/lib/normalize.mjs`.

### Time-index rat-hour fixture

```bash
# inspect
npm run dump:time-index-rat-hour --prefix tools/iztro-reference

# regenerate crates/iztro/fixtures/iztro/time_index_rat_hour.json from repo root
npm run dump:time-index-rat-hour --prefix tools/iztro-reference -- --write
```

`dump:time-index-rat-hour` emits one supported-field-only `byLunar` fixture for
upstream `timeIndex` `0..=12`. It covers early Zi (`0`), late Zi (`12`), one
ordinary non-Zi time, and a real 2020 leap fourth-month second-half pair proving
the late-Zi guard on effective-month advancement. It preserves raw upstream
labels beside normalized keys and excludes full facade serialization parity,
full horoscope assembly, temporal decorative arrays, features, rules, and
narrative.

### Runtime star families

```bash
# inspect (prints all fixtures to stdout)
npm run dump:runtime-star-families --prefix tools/iztro-reference

# regenerate fixtures under crates/iztro/fixtures/iztro/ (run from the repo root so the
# relative crates/iztro/fixtures/iztro/ path resolves)
node tools/iztro-reference/scripts/dump-runtime-star-families.mjs --write
```

`dump:runtime-star-families` emits the decorative natal families
(长生/博士/岁前/将前十二神) per palace for the default and Zhongzhou algorithms,
and the scoped flow stars (流耀) from `getHoroscopeStar` for every scope across
all ten stems and twelve branches. With `--write` it regenerates
`crates/iztro/fixtures/iztro/runtime_decorative_*.json` and `crates/iztro/fixtures/iztro/flow_stars.json`.

### Full horoscope fixture

```bash
# inspect (prints the fixture to stdout)
npm run dump:horoscope --prefix tools/iztro-reference

# regenerate crates/iztro/fixtures/iztro/horoscope.json
npm run dump:horoscope --prefix tools/iztro-reference -- --write
```

`dump:horoscope` snapshots upstream `FunctionalAstrolabe#horoscope(targetSolarDate,
targetTimeIndex)` output as the contract that later iztro-rs horoscope-assembly
PRs target. The write path resolves relative to the script, so `--write` lands in
`crates/iztro/fixtures/iztro/horoscope.json` whether invoked via `npm run --prefix` or directly
with `node`. It is a tooling-only reference; it does **not** implement or claim
Rust horoscope parity.

The fixture covers a small, representative matrix (not exhaustive): the canonical
lunar female chart (`1990-5-17`, time index `4`, `女`, non-leap, `fix_leap=true`,
`zh-CN`) under the default algorithm at a mid-decade target year (`2026`) and at
`2034` (the start of the next 大限, a decade boundary), the same chart under the
Zhongzhou algorithm at `2026`, and a male chart (`1988-3-14`, time index `0`) at
`2026`. So it spans both genders, both algorithms, two target years, and a
decade-boundary crossing.

Per case it records:

- `input` — the chart input plus the `target` solar date / year / time index;
- `raw_keys` — the exact upstream object keys observed for the top-level
  horoscope result and each scope, as a contract anchor;
- `supported_fields` — normalized facts for each temporal frame.

Each temporal scope (`decadal` 大限, `age` 小限, `yearly` 流年, `monthly` 流月,
`daily` 流日, `hourly` 流时) carries its `index`, period `heavenly_stem` /
`earthly_branch`, the 12-entry `palace_names` layout (index 0 = 寅), and the
four-transform (`四化`) `mutagen` block keyed `lu` / `quan` / `ke` / `ji`. The
flow-bearing scopes additionally carry the flow-star (`流耀`) `flow_stars` array
(base + branch + upstream type). `age` adds `nominal_age` and has no `flow_stars` array. `yearly`
adds `nian_jie_branch` (年解) and the `yearly_dec_stars` families `suiqian12`
(岁前) and `jiangqian12` (将前). Raw upstream Chinese labels are preserved beside
every normalized key for diagnosis. The dumper fails loudly if a scope exposes an
unexpected key or an unmapped star/branch/stem so a future iztro bump cannot
silently drop a field.

Intentionally **not** normalized yet (recorded in the fixture's
`not_normalized_yet` note): the runtime palace projections (`agePalace`, `palace`,
`surroundPalaces`), the full natal `astrolabe` re-embedded in the horoscope
result, the boolean query helpers (`hasHoroscopeStars` and friends), and all
narrative/interpretation text.

Shared normalization maps/helpers are reused from `scripts/lib/normalize.mjs`;
only the horoscope-specific flow-base and scope-prefix maps are local to the
script.
