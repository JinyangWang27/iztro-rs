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

# regenerate fixtures/iztro/e2e_supported_by_lunar.json from repo root
npm run dump:e2e-supported --prefix tools/iztro-reference -- --write
```

`dump:e2e-supported` emits one compact supported-field-only fixture with six
ordinary lunar chart cases under both the default and Zhongzhou algorithms (12
by_lunar E2E cases). It normalizes only the current Rust-supported facts:
life/body palace branches, five-element bureau, palace branch/stem/name facts,
typed natal stars, and the four decorative runtime families. It intentionally
does not snapshot full facade serialization parity, calendar conversion,
leap-month behavior, rat-hour variants, horoscope derivation, features, rules,
or narrative output.

### Runtime star families

```bash
# inspect (prints all fixtures to stdout)
npm run dump:runtime-star-families --prefix tools/iztro-reference

# regenerate fixtures under fixtures/iztro/ (run from the repo root so the
# relative fixtures/iztro/ path resolves)
node tools/iztro-reference/scripts/dump-runtime-star-families.mjs --write
```

`dump:runtime-star-families` emits the decorative natal families
(长生/博士/岁前/将前十二神) per palace for the default and Zhongzhou algorithms,
and the scoped flow stars (流耀) from `getHoroscopeStar` for every scope across
all ten stems and twelve branches. With `--write` it regenerates
`fixtures/iztro/runtime_decorative_*.json` and `fixtures/iztro/flow_stars.json`.
