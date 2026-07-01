# Architecture

`iztro-rs` uses a layered architecture. Each layer has a clear responsibility and should avoid leaking concerns into adjacent layers.

The current implementation separates chart facts, renderer-friendly read models, rendering, feature extraction, rules, localization, application frontends, and narrative. This separation is intentional: chart generation should stay deterministic and fixture-backed, while renderers, GUIs, TUIs, MCP tools, and future 3D views consume read models instead of re-deriving chart layout from core aggregates.

## 1. Core Chart Layer

The Core Chart Layer contains deterministic chart facts. It should not contain interpretation prose, report formatting, CLI output formatting, GUI assumptions, renderer geometry, or runtime language selection.

Examples:

- birth context and calendar options;
- low-level stem/branch and sexagenary-cycle value objects owned by `lunar-lite` and used directly; the `lunar-lite` calendar conversion stays behind the `core/calendar` adapter;
- NaYin and five-element bureau facts owned by `core`;
- palaces;
- typed natal stars;
- untyped decorative runtime star facts;
- brightness;
- natal mutagens;
- model-only horoscope overlays;
- scoped temporal star placements;
- temporal mutagen activations;
- method profile and chart-plane metadata.

The output of this layer is a structured chart object, usually `Chart`, or a model-only overlay wrapper such as `HoroscopeChart`.

### Natal facts and temporal overlays

Natal chart facts are immutable once built. `Chart::stars()` returns typed natal `StarPlacement`s only, while `Palace::decorative_stars()` holds untyped decorative runtime facts such as 长生/博士/岁前/将前十二神. These surfaces must remain separate.

Temporal facts are additive overlays. `HoroscopeChart` wraps a natal `Chart` and zero or more `TemporalLayer`s. A temporal layer records its `Scope`, `TemporalContext`, branch-tagged scoped star placements, scoped decorative star facts, and `MutagenActivation`s. It must not duplicate natal placements or mutate natal palace names. 四化 activations remain activation facts, not fake stars.

## 2. Snapshot / Read Model Layer

The Snapshot Layer converts core chart facts into renderer-neutral read models.

The main read model is `ChartStackSnapshot`:

```text
Chart / HoroscopeChart
  -> ChartStackSnapshot
     -> ChartLayerSnapshot[]
        -> PalaceLayerCellSnapshot[]
```

It represents the conventional 12-palace grid as x/y coordinates and the selected natal/temporal scopes as stack layers. This keeps future renderers free to show the same data as:

- a plain text list;
- a terminal UI grid;
- a 2D palace grid;
- a 文墨天机-style interactive chart;
- a future 3D stacked view where the z-axis is 本命 / 大限 / 流年 / 流月 / 流日 / 流时.

A renderer should consume `ChartStackSnapshot` rather than walking `Chart` directly. A future GUI should change the selected temporal view request and re-render from a snapshot; it should not mutate the natal chart.

### Facade snapshots and GUI projections

`HoroscopeFacadeSnapshot` and related facade DTOs are compatibility/export payloads. They should preserve stable machine-readable fields, deterministic ordering, and additive conventional Chinese labels where useful for compatibility, but they should not become UI layout code or runtime localization infrastructure.

GUI-facing read models live in their own top-level modules rather than in `core`, so `core` owns domain facts and transformations only:

- `projection` owns the serializable static chart read models (the projections);
- `facade` orchestrates them — it builds or receives core charts, resolves the selected temporal layers, and assembles a projection.

The dependency direction is `core <- {analysis, projection} <- facade`: `core` never depends on projections, GUI DTOs, panel state, selectors, or highlights.

A 文墨天机-style static chart is backed by `StaticChartProjection` (in `projection::static_chart`). It is derived from existing chart/facade facts via `StaticChartProjection::from_chart` (natal-only) or `from_horoscope_chart_with` (natal plus selected temporal overlays), and includes:

- the conventional 4x4 palace-grid position for each palace (reusing `palace_grid_position`);
- display-ready labels for branches, stems, palace names, stars, brightness, mutagens, decorative-star families, star categories, and scopes;
- star lists grouped by `StarCategory` (major / minor / adjective, with a reserved `other_typed_stars`) plus decorative stars, all in deterministic facade star order;
- scope-selector state (本命/大限/小限/流年/流月/流日/流时) and the active scopes, so a frontend can render selector controls without owning that logic;
- selected temporal overlays for the current view, kept separate from natal facts;
- reserved highlight annotations (`HighlightProjection`), currently empty unless populated by feature/rule layers.

**Frame-relative palace identity.** A branch is the stable palace-cell coordinate; a palace *name* is assigned by a palace frame. The natal chart and each temporal layer are different palace-name frames over the same branch ring, so each `StaticPalaceProjection` carries both:

- `natal_identity` (`StaticNatalPalaceIdentity`) — the immutable natal meaning of the branch (natal 宫名, 宫干, natal roles);
- `active_frame` (`StaticPalaceFrameIdentity`) — the selected frame's meaning (`frame_scope`, `palace_name`, `is_life_palace`), the ring a GUI renders as the main palace title.

`StaticChartProjectionRequest` makes the two concepts explicit and separate: `visible_scopes` controls which temporal layers are visible as overlays, while `active_frame_scope` selects the primary palace-name frame. A 流年 view has `visible_scopes = [Natal, Decadal, Age, Yearly]` but `active_frame_scope = Yearly` — 小限 (Age) stays visible as auxiliary data yet never becomes the active frame. The facade derives `active_frame_scope` from the navigation selection via the single canonical mapping `StaticTemporalNavigationSelection::active_frame_scope`. A non-natal active frame is built from the selected layer's `TemporalPalaceLayout`; a missing layer/layout fails loudly rather than silently falling back to natal names. Selecting a temporal scope therefore keeps natal facts immutable while changing the active palace frame and the visible overlays — the GUI consumes the projection and never computes temporal 命宫 itself.

The projection remains renderer-neutral. It may describe that a palace or star should be highlighted, but it does not choose CSS classes, colors, canvas coordinates, camera position, animation, or 3D geometry.

`StaticChartProjection` remains language-neutral enough for frontends to localize at presentation boundaries. The desktop GUI uses `crates/iztro-i18n` to render the current surface in English or Simplified Chinese without making localized strings the internal model identity.

### Static chart slices before timeline and 3D

The first GUI target is a static palace-grid chart. The same static chart projection should later be reusable as one frame in a temporal sequence:

```text
TimelineFrame
  -> target_context
  -> StaticChartProjection
  -> HighlightProjection[]
```

A future 3D view can stack these frames along a time axis. The core should therefore expose facts, selected-scope overlays, and highlight annotations; the frontend decides whether to draw them as a static chart, animation, or 3D scene.

Pattern and 成格 highlighting should be produced by feature/rule layers as structured annotations, not by the renderer. Until the rule engine can identify real patterns, highlight fields should be reserved or empty rather than hard-coded in UI code.

## 3. Runtime Localization Layer

Runtime localization is a presentation boundary, not a chart-generation concern.

`crates/iztro-i18n` owns:

- supported runtime locales, currently `en-US` and `zh-Hans`;
- Fluent resource loading and formatting;
- fallback behavior, with `en-US` as the default fallback;
- typed helpers such as `star_name`, `palace_name`, `mutagen`, and `temporal_label`;
- stable key mapping from domain enums/value objects to localized labels.

Core domain types must remain stable enums/value objects. They should not become localized strings. A GUI may call `i18n.star_name(star_name)`, but placement and compatibility tests should still assert `StarName` values, `EarthlyBranch` values, `MutagenActivation` facts, and other typed structures.

Facade/export DTOs may keep additive zh-CN labels for compatibility and readability, but they are not the general runtime i18n mechanism.

## 4. Render Layer

The Render Layer turns snapshot/read-model data into human-facing display formats.

The first concrete renderer is `render`'s plain text chart-stack renderer. It consumes `ChartStackSnapshot` and produces deterministic text output for demos and debugging. It does not generate chart facts, derive temporal periods, localize terminology, evaluate rules, or produce interpretation.

Future renderers may include CLI, TUI, web, GUI, SVG/HTML, or 3D views. They should remain consumers of snapshot/read-model structures.

## 5. Application and Tooling Layer

Application surfaces are consumers of facts, snapshots, features, claims, and annotations. They should not become alternative chart engines.

Recommended ordering:

1. **Static GUI**: validate the 12-palace chart, saved-chart flow, temporal controls, hover/click highlighting, and i18n with real visual feedback.
2. **TUI**: provide a lightweight terminal view and debugging surface over the same snapshots. The TUI should be useful for CI fixtures, SSH workflows, and coding agents, but should not own astrology logic.
3. **MCP server/tooling**: expose stable typed queries to coding agents only after the facade/query surface is stable enough to avoid churn. MCP should return structured facts, snapshots, features, pattern hits, claims, and evidence, not only prose.
4. **Timeline/3D views**: consume reusable static chart frames and structured highlights after the static chart model is stable.

A frontend may choose a different interaction model, but it must not parse rendered text to recover facts or duplicate placement/rule logic.

## 6. Feature Extraction Layer

The Feature Extraction Layer converts a chart into a semantic feature graph.

Important feature dimensions include:

- calendar and boundary settings;
- twelve-palace features;
- star placement and star semantics;
- mutagen flows;
- palace relations such as opposite palace and triads;
- temporal activation from decadal/yearly/monthly/daily/hourly scopes;
- strength scores and counter-evidence.

The goal is not to write prose, but to expose features that a rule engine can evaluate.

Pattern (格局) recognition is rule-engine code rather than core chart state. The executable pattern engine lives under `rules::pattern`, where it can read core facts without making `core` depend on rules. See [`patterns.md`](patterns.md) for the rule catalog and guarantees.

## 7. Rule Engine Layer

The Rule Engine Layer maps chart facts and extracted features into structured rule outputs.

Rules should not directly emit final narrative text. A rule should emit:

- domain;
- theme;
- polarity;
- strength;
- evidence;
- counter-evidence;
- source metadata.

This makes rule matching testable and allows multiple rules to be aggregated before generating a report.

`rules::pattern` recognizes classical pattern structures as `PatternDetection`
facts. `rules::classical` evaluates classical source rules into source hits,
claims, and diagnostics. Both engines consume chart facts; neither owns chart
generation or placement.

### Layer-level analysis coordination

The `analysis` module is a thin coordinator that composes `rules::pattern` detection and `rules::classical` evaluation for **cacheable, per-layer** detection. It lives outside `core` precisely because `core` must not depend on `rules`, while a layer-level API needs both. It adds no new interpretation: `detect_analysis_layer` returns compact `ClassicalRuleHitRef`s (resolved back to verbatim source text by `classical_rule_metadata`) plus structured `PatternDetection`s for one `AnalysisLayerKey`, leaving grouping, caching, and rendering to the consumer. See [`rules/rule-engine.md`](rules/rule-engine.md) for the API and the deepest-layer cross-layer assignment policy.

## 8. Narrative Layer

The Narrative Layer turns structured claims into human-readable reports.

The first implementation should support deterministic templates. Optional LLM-assisted polishing may be added later, but LLMs should not be responsible for raw chart interpretation.

## Method profiles

Multi-school compatibility should be implemented through composable method profiles rather than one monolithic school enum.

A method profile may specify:

- calendar strategy;
- chart algorithm strategy;
- star placement strategy;
- mutagen strategy;
- feature extraction strategy;
- rule set selection;
- narrative style.

This allows configurations such as `QuanShu chart generation + SanHe features + basic mutagen rules + technical narrative`.

## Input calculation policy

`ChartAlgorithmKind`, `ChartPlane`, and `ChartCalculationConfig` are separate axes and must not be conflated:

- `ChartAlgorithmKind` is the algorithm family (全书 / 中州 / …).
- `ChartPlane` is the plane variant (天盘 / 地盘 / 人盘) within a family.
- `ChartCalculationConfig` is the input calculation policy applied *before* chart generation.

`ChartCalculationConfig` currently includes `SolarTimePolicy`, `YearBoundary`, `LeapMonthBoundary`, and `NominalAgeBoundary`. `YearBoundary` and `LeapMonthBoundary` affect natal input normalization: `YearBoundary::ChineseNewYearEve` means the previous cyclic year lasts through 除夕 and the new cyclic year begins at 正月初一, while `YearBoundary::LiChun` uses 立春, resolved at datetime granularity via `lunar_lite::li_chun_datetime` (intentionally diverging from upstream date-level `iztro@2.5.8`). `LeapMonthBoundary` maps to the legacy `fix_leap` split. `NominalAgeBoundary` is runtime-only: it affects horoscope nominal-age resolution and does not affect natal chart generation.

`Chart` remains the immutable chart fact aggregate. Calculation diagnostics are exposed through generation reports and diagnostic snapshots, not stored inside `Chart`. These reports make resolved clock time, apparent-solar-time corrections, year-boundary effects, leap-month policy mapping, and nominal-age resolution inspectable without changing normal `Chart` serialization.

The user always inputs a birth clock time. The calculation policy decides how that clock time becomes a 时辰:

```text
raw birth date + civil clock time
  -> optional apparent solar time adjustment
  -> resolved local date/time
  -> derive time branch / time index
  -> existing natal chart generation
```

Apparent solar time is an input calculation policy. It normalises birth clock time using time zone and longitude before the chart is generated. It does not define a new algorithm and does not define a new chart plane. The longitude correction is exact:

```text
timezone_meridian_degrees = utc_offset_hours * 15
longitude_correction_minutes = 4 * (longitude_degrees - timezone_meridian_degrees)
resolved_time = clock_time + longitude_correction_minutes + equation_of_time_minutes
```

When the adjusted time crosses midnight, the resolved solar date moves to the adjacent day. These policies run ahead of the existing chart-generation path and never define a new algorithm, chart plane, natal anchor, or star placer.

## Evidence-first interpretation

Every interpretive claim should be traceable to chart evidence. This enables debugging, review, rule tuning, and future empirical validation.
