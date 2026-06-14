# Current Project Status

This document summarizes the current implemented surface after the recent `lunar-lite`, snapshot, renderer, decadal-frame, and demo work.

## Compatibility target

The current chart-generation compatibility target is `iztro@2.5.8`.

Compatibility is fixture-driven and scoped to the supported fact surface. The project does not yet claim full upstream API parity, full horoscope assembly, full serialization parity, or interpretation parity.

## Implemented chart-generation surface

The supported natal chart fact surface currently includes:

- typed request facades: `by_lunar` and `by_solar`;
- `lunar-lite` 0.3.1-backed solar-to-lunar conversion and normal-boundary four-pillar birth-year derivation for `by_solar`;
- leap-month and `fix_leap` handling for the supported slice;
- `BirthTime` / upstream `timeIndex` `0..=12`, including early Zi and late Zi;
- retained `Chart::birth_year()` stem-branch fact;
- twelve palace layout;
- Life Palace and Body Palace branches;
- palace heavenly stems;
- five-element bureau;
- represented typed natal stars;
- supported brightness and birth-year mutagens;
- untyped decorative runtime star families in `Palace::decorative_stars()`;
- branch-tagged typed temporal flow-star placements from explicit temporal contexts;
- decadal and yearly mutagen activation layers from explicit contexts;
- typed `DecadalFrame` derivation with 12 ten-year periods, direction, age ranges, and natal palace stem-branch facts.

Default/non-Zhongzhou natal output remains 66 typed natal stars. Zhongzhou natal output remains 68 typed natal stars. `represented_star_metadata_table().len() == 70` stays natal-only, while `known_star_metadata_table().len() == 170` inventories the broader upstream runtime star-name universe.

## Domain boundary decisions

The following boundaries are deliberate:

- `lunar-lite` owns canonical low-level `HeavenlyStem`, `EarthlyBranch`, and `StemBranch` primitives.
- `core` owns Zi Wei-specific NaYin and five-element bureau logic.
- `Chart` retains birth-year `StemBranch` as a natal identity fact.
- `Chart::stars()` returns typed natal `StarPlacement`s only.
- `Palace::decorative_stars()` contains untyped natal decorative runtime facts.
- `TemporalLayer::placements()` contains branch-tagged typed temporal placements.
- `MutagenActivation` records 四化 activation facts and is not modeled as a fake star.
- `HoroscopeChart` wraps an immutable natal `Chart` and additive temporal layers.

## Snapshot and rendering surface

`ChartStackSnapshot` is the renderer-neutral read model for demos and future frontends.

It preserves:

- chart identity fields such as birth context and method profile;
- birth-year stem-branch;
- natal Life/Body Palace branches and five-element bureau;
- conventional 12-palace visual grid positions;
- one natal layer plus zero or more temporal layers;
- separate cell sections for natal typed stars, decorative stars, scoped temporal stars, and mutagen activations.

`render` currently provides a deterministic plain text renderer over `ChartStackSnapshot`. The top-level README and `docs/en/demo.md` show the current end-to-end flow:

```text
solar input -> by_solar -> ChartStackSnapshot -> render module plain text output
```

## Deferred work

The following remain intentionally out of scope for the current supported surface:

- full BaZi output;
- full horoscope assembly;
- 流年 / 流月 / 流日 / 流时 period derivation;
- attaching derived 大限 frames as temporal layers;
- horoscope palace-name derivation;
- temporal decorative arrays such as upstream `yearlyDecStar`;
- full upstream facade serialization parity;
- bindings;
- richer renderers and GUI/WASM/TUI frontends;
- feature extraction for temporal activation;
- rule evaluation;
- deterministic readings;
- narrative or LLM-assisted prose.

## Near-term direction

The next implementation work should stay incremental:

1. Continue keeping compatibility fixture-backed.
2. Build richer renderers or CLI demos on top of `ChartStackSnapshot`, not directly on `Chart` internals.
3. Add full horoscope assembly in small PRs: attach decadal frames to temporal layers, then add yearly, monthly, daily, and hourly derivation.
4. Only after the fact surface is stable, expand feature extraction, rules, and narrative.
