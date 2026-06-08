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
