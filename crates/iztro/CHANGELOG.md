# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.10.0](https://github.com/JinyangWang27/iztro-rs/compare/v0.9.0...v0.10.0) - 2026-07-02

### Added

- *(classical)* evaluate chang-qu clamp in overlay ([#149](https://github.com/JinyangWang27/iztro-rs/pull/149))
- *(rules)* add classical rule engine foundation (schema + corpus + engine) ([#113](https://github.com/JinyangWang27/iztro-rs/pull/113))

### Fixed

- *(pattern)* align normalized detectors with maintained conditions ([#143](https://github.com/JinyangWang27/iztro-rs/pull/143))
- font/LFS bug, rule-engine guardrails, and doc realignment ([#138](https://github.com/JinyangWang27/iztro-rs/pull/138))

### Other

- *(pattern)* move detectors to named-pattern layout ([#155](https://github.com/JinyangWang27/iztro-rs/pull/155))
- Centralize pattern metadata and document registry ([#154](https://github.com/JinyangWang27/iztro-rs/pull/154))
- add source inventory for upcoming pattern expansion ([#153](https://github.com/JinyangWang27/iztro-rs/pull/153))
- *(release)* add Windows iztro-gui installer ([#152](https://github.com/JinyangWang27/iztro-rs/pull/152))
- *(release)* build iztro-gui artifacts and installers ([#151](https://github.com/JinyangWang27/iztro-rs/pull/151))
- *(architecture)* record domain model first principles ([#150](https://github.com/JinyangWang27/iztro-rs/pull/150))
- *(rules)* move pattern engine under rules namespace ([#148](https://github.com/JinyangWang27/iztro-rs/pull/148))
- introduce shared RuleEvaluationContext ([#147](https://github.com/JinyangWang27/iztro-rs/pull/147))
- Refactor pattern context and clarify query helpers ([#146](https://github.com/JinyangWang27/iztro-rs/pull/146))
- Add effective temporal state enhancements ([#145](https://github.com/JinyangWang27/iztro-rs/pull/145))
- introduce frame-relative static chart projections ([#144](https://github.com/JinyangWang27/iztro-rs/pull/144))
- Enhance theme support and improve PatternPolarity semantics ([#141](https://github.com/JinyangWang27/iztro-rs/pull/141))
- Enhance pattern queries and document overlay pattern analysis ([#139](https://github.com/JinyangWang27/iztro-rs/pull/139))
- Add segmented pattern-rule source groups and metadata for Volume 1 ([#136](https://github.com/JinyangWang27/iztro-rs/pull/136))
- Refactor calendar boundary adapter for improved robustness ([#135](https://github.com/JinyangWang27/iztro-rs/pull/135))
- migrate back to lunar-lite with datetime-level LiChun ([#134](https://github.com/JinyangWang27/iztro-rs/pull/134))
- Expand QuanShu rules and update documentation ([#133](https://github.com/JinyangWang27/iztro-rs/pull/133))
- Add classical rule guardrail tests and authoring guide ([#132](https://github.com/JinyangWang27/iztro-rs/pull/132))
- add selected-view batch facade and per-key scope helper ([#130](https://github.com/JinyangWang27/iztro-rs/pull/130))
- add layer-level rule and pattern detection APIs ([#128](https://github.com/JinyangWang27/iztro-rs/pull/128))
- add classical rule panel view model ([#127](https://github.com/JinyangWang27/iztro-rs/pull/127))
- execute Tan Lang water-romance rules ([#126](https://github.com/JinyangWang27/iztro-rs/pull/126))
- Update terminology for classical rule provenance ([#125](https://github.com/JinyangWang27/iztro-rs/pull/125))
- Refactor QuanShu source inventory to grouped TOML format ([#124](https://github.com/JinyangWang27/iztro-rs/pull/124))
- complete Tai Wei Fu rule normalization map ([#122](https://github.com/JinyangWang27/iztro-rs/pull/122))
- remove placeholder scaffold modules ([#121](https://github.com/JinyangWang27/iztro-rs/pull/121))
- Enhance classical rules with source hits and claim metadata ([#120](https://github.com/JinyangWang27/iztro-rs/pull/120))
- Complete Tai Wei Fu source segmentation and update coverage report ([#119](https://github.com/JinyangWang27/iztro-rs/pull/119))
- Segment Tai Wei Fu example clauses and add coverage report ([#118](https://github.com/JinyangWang27/iztro-rs/pull/118))
- *(rules)* introduce clause-level QuanShu source inventory ([#117](https://github.com/JinyangWang27/iztro-rs/pull/117))
- Align Tian Ma Void Source ID and Validate Source Inventory ([#116](https://github.com/JinyangWang27/iztro-rs/pull/116))
- *(rules)* add QuanShu source volumes ([#115](https://github.com/JinyangWang27/iztro-rs/pull/115))

### Changed

- Migrate the calendar engine back to `lunar-lite` (1.2.1) and remove the
  `tyme4rs` dependency. The duplicated `core/model/ganzhi` GanZhi model is
  removed; `lunar-lite`'s `HeavenlyStem`/`EarthlyBranch`/`StemBranch`/
  `FourPillars` are used directly. `YearBoundary::LiChun` stays datetime-level,
  now powered by `lunar_lite::li_chun_datetime`: a birth before the exact 立春
  instant on the 立春 day keeps the previous Ganzhi year. This intentionally
  diverges from upstream date-level `iztro@2.5.8`; the
  `year_divide_exact_2000_02_04` case (08:00, before the 20:40:24 instant) keeps
  the corrected `己卯` result.

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
