# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Migrate the calendar engine back to `lunar-lite` (1.1.0) and remove the
  `tyme4rs` dependency. The duplicated `core/model/ganzhi` GanZhi model is
  removed; `lunar-lite`'s `HeavenlyStem`/`EarthlyBranch`/`StemBranch`/
  `FourPillars` are used directly. `YearBoundary::LiChun` returns to
  `lunar-lite`'s date-level 立春 boundary, restoring `iztro@2.5.8` parity for
  the `year_divide_exact_2000_02_04` case (no more datetime-level divergence).

## [0.9.0](https://github.com/JinyangWang27/iztro-rs/compare/v0.8.0...v0.9.0) - 2026-06-24

### Added

- *(core)* wire chart plane through chart requests ([#102](https://github.com/JinyangWang27/iztro-rs/pull/102))

### Other

- Refactor calendar to use tyme4rs adapter ([#112](https://github.com/JinyangWang27/iztro-rs/pull/112))
- Add calculation generation reports and diagnostics documentation ([#111](https://github.com/JinyangWang27/iztro-rs/pull/111))
- Add year, leap-month, and nominal-age boundary policies ([#110](https://github.com/JinyangWang27/iztro-rs/pull/110))
- Add input calculation policy for apparent solar time ([#109](https://github.com/JinyangWang27/iztro-rs/pull/109))
- Refactor natal plane resolver ([#107](https://github.com/JinyangWang27/iztro-rs/pull/107))
- Add chart diagnostic snapshot and invariants ([#106](https://github.com/JinyangWang27/iztro-rs/pull/106))
- Add typed palace lookup helpers ([#105](https://github.com/JinyangWang27/iztro-rs/pull/105))
- Make Chart self-describing with explicit ChartProfile ([#104](https://github.com/JinyangWang27/iztro-rs/pull/104))
- Add anchor-aware support for Zhongzhou chart planes ([#103](https://github.com/JinyangWang27/iztro-rs/pull/103))
- *(core)* add chart plane foundation ([#100](https://github.com/JinyangWang27/iztro-rs/pull/100))

## [0.8.0](https://github.com/JinyangWang27/iztro-rs/compare/v0.7.0...v0.8.0) - 2026-06-21

### Added

- *(gui)* migrate iztro-gui to Fluent i18n with locale switching
- *(core)* expose typed center fields for localization
- *(pattern)* add four conservative clamp/brightness pattern rules
- *(core)* prepare iztro-style center, palace-limit, and temporal labels ([#87](https://github.com/JinyangWang27/iztro-rs/pull/87))

### Fixed

- *(pattern)* reject empty PatternScope::Combined in scope guard

### Other

- Add NatalStarPlacementStrategy orchestration layer ([#98](https://github.com/JinyangWang27/iztro-rs/pull/98))
- compact 小限 rendering and propagate Minor Limit errors
- document 小限 vs 流年 and cover the active Minor Limit
- derive active 小限 / Minor Limit in static temporal view
- apply rustfmt line-wrapping
- *(pattern)* cover the four new clamp/brightness rules
- Address PR #90 review: scope filtering, rule guards, evidence clarity
- Add read-only pattern (格局) detection layer

## [0.7.0](https://github.com/JinyangWang27/iztro-rs/compare/v0.6.0...v0.7.0) - 2026-06-18

### Added

- *(gui)* refine static chart interactions ([#84](https://github.com/JinyangWang27/iztro-rs/pull/84))
- *(gui)* refine static chart window ([#83](https://github.com/JinyangWang27/iztro-rs/pull/83))
- *(core)* expose natal four pillars in facade snapshots ([#82](https://github.com/JinyangWang27/iztro-rs/pull/82))
- *(core)* add natal four pillars

### Fixed

- *(core)* avoid let-chain in chart constructor

### Other

- raise coverage for supported snapshots

## [0.6.0](https://github.com/JinyangWang27/iztro-rs/compare/v0.5.0...v0.6.0) - 2026-06-17

### Added

- *(gui)* add birth input, by_solar generation, and chart cache
- *(core)* add from_horoscope_chart temporal overlays + selectors
- *(core)* add StaticChartViewSnapshot view model + from_chart
- *(labels)* add scope_zh and star_category_zh helpers
- *(core)* expose zh-CN labels on natal facade snapshots
- *(core)* add table-driven zh-CN label module
- *(core)* add natal astrolabe facade snapshot ([#73](https://github.com/JinyangWang27/iztro-rs/pull/73))
- *(core)* retain horoscope target context ([#72](https://github.com/JinyangWang27/iztro-rs/pull/72))
- *(core)* add horoscope runtime helpers ([#69](https://github.com/JinyangWang27/iztro-rs/pull/69))

### Fixed

- *(core)* always keep natal active in horoscope static views

### Other

- rustfmt static chart view fixture test
- *(core)* add static chart view golden fixture + manifest entry
- enhance comments for 紫微 and 天府 series with additional context
- *(facade)* pin deterministic facade star ordering
- *(core)* order facade palace stars deterministically
- *(fixtures)* document CASES.json case registry
- *(fixtures)* add fixture case registry drift tests
- *(fixtures)* add canonical birth case registry CASES.json
- add more tests
- *(fixtures)* add fixture manifest registry and drift test
- *(test)* prefix leap-month and rat-hour boundary tests
- *(test)* centralize compat fixture plumbing and add compat_ prefix
- *(test)* split shared test common into module tree
- *(core)* add horoscope facade reference fixtures ([#71](https://github.com/JinyangWang27/iztro-rs/pull/71))

## [0.5.0](https://github.com/JinyangWang27/iztro-rs/compare/v0.4.0...v0.5.0) - 2026-06-16

### Added

- *(core)* add horoscope supported-fields snapshot ([#67](https://github.com/JinyangWang27/iztro-rs/pull/67))

## [0.4.0](https://github.com/JinyangWang27/iztro-rs/compare/v0.3.1...v0.4.0) - 2026-06-16

### Added

- *(core)* add yearly decorative star facts ([#66](https://github.com/JinyangWang27/iztro-rs/pull/66))
- *(core)* add hourly horoscope layer

### Other

- Merge pull request #64 from JinyangWang27/feat/full-horoscope-stack-assembly
- update comments for clarity on temporal palace derivation
- Merge branch 'main' into feat/hourly-horoscope-layer

## [0.3.1](https://github.com/JinyangWang27/iztro-rs/compare/v0.3.0...v0.3.1) - 2026-06-16

### Added

- *(core)* add daily horoscope layer ([#60](https://github.com/JinyangWang27/iztro-rs/pull/60))

## [0.3.0](https://github.com/JinyangWang27/iztro-rs/compare/v0.2.0...v0.3.0) - 2026-06-16

### Added

- *(core)* add monthly horoscope layer ([#58](https://github.com/JinyangWang27/iztro-rs/pull/58))
- *(core)* add yearly horoscope layer ([#56](https://github.com/JinyangWang27/iztro-rs/pull/56))

### Other

- Add age horoscope scope ([#54](https://github.com/JinyangWang27/iztro-rs/pull/54))

## [0.2.0](https://github.com/JinyangWang27/iztro-rs/compare/v0.1.2...v0.2.0) - 2026-06-16

### Added

- *(core)* add temporal palace-name layout facts

## [0.1.2](https://github.com/JinyangWang27/iztro-rs/compare/v0.1.1...v0.1.2) - 2026-06-15

### Other

- update Cargo.toml dependencies

## [0.1.1](https://github.com/JinyangWang27/iztro-rs/compare/v0.1.0...v0.1.1) - 2026-06-15

### Added

- add README and configure release-plz workflow ([#48](https://github.com/JinyangWang27/iztro-rs/pull/48))
