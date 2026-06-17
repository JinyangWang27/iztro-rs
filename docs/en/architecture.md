# Architecture

`iztro-rs` uses a layered architecture. Each layer has a clear responsibility and should avoid leaking concerns into adjacent layers.

The current implementation separates chart facts, renderer-friendly read models, rendering, feature extraction, rules, and narrative. This separation is intentional: chart generation should stay deterministic and fixture-backed, while renderers and future GUIs consume read models instead of re-deriving chart layout from core aggregates.

## 1. Core Chart Layer

The Core Chart Layer contains deterministic chart facts. It should not contain interpretation prose, report formatting, CLI output formatting, or GUI assumptions.

Examples:

- birth context and calendar options;
- low-level stem/branch and sexagenary-cycle primitives re-exported from `lunar-lite`;
- NaYin and five-element bureau facts owned by `core`;
- palaces;
- typed natal stars;
- untyped decorative runtime star facts;
- brightness;
- natal mutagens;
- model-only horoscope overlays;
- scoped temporal star placements;
- temporal mutagen activations;
- method profile metadata.

The output of this layer is a structured chart object, usually `Chart`, or a model-only overlay wrapper such as `HoroscopeChart`.

### Natal facts and temporal overlays

Natal chart facts are immutable once built. `Chart::stars()` returns typed natal `StarPlacement`s only, while `Palace::decorative_stars()` holds untyped decorative runtime facts such as 长生/博士/岁前/将前十二神. These surfaces must remain separate.

Temporal facts are additive overlays. `HoroscopeChart` wraps a natal `Chart` and zero or more `TemporalLayer`s. A temporal layer records its `Scope`, `TemporalContext`, branch-tagged scoped star placements, and `MutagenActivation`s. It must not duplicate natal placements or mutate natal palace names. 四化 activations remain activation facts, not fake stars.

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
- a 2D palace grid;
- a 文墨天机-style interactive chart;
- a future 3D stacked view where the z-axis is 本命 / 大限 / 流年 / 流月 / 流日 / 流时.

A renderer should consume `ChartStackSnapshot` rather than walking `Chart` directly. A future GUI should change the selected temporal view request and re-render from a snapshot; it should not mutate the natal chart.

### Facade snapshots and GUI view models

`HoroscopeFacadeSnapshot` and related facade DTOs are compatibility/export payloads. They should preserve stable machine-readable fields, deterministic ordering, and additive Chinese labels, but they should not become UI layout code.

A 文墨天机-style static chart should instead be backed by a dedicated GUI-facing read model, for example a future `StaticChartViewSnapshot`. That view model can be derived from existing chart/facade facts and may include:

- the conventional 4x4 palace-grid position for each palace;
- Chinese labels for branches, stems, palace names, stars, brightness, mutagens, and decorative-star families;
- grouped star lists for display, such as major stars, minor/helper stars, adjective/misc stars, and decorative stars;
- selected natal/temporal overlays for the current view;
- empty or populated highlight annotations.

The view model should remain renderer-neutral. It may describe that a palace or star should be highlighted, but it should not choose CSS classes, colors, canvas coordinates, camera position, animation, or 3D geometry.

### Static chart slices before timeline and 3D

The first GUI target is a static palace-grid chart. The same static chart view model should later be reusable as one frame in a temporal sequence:

```text
TimelineFrame
  -> target_context
  -> StaticChartViewSnapshot
  -> HighlightView[]
```

A future 3D view can stack these frames along a time axis. The core should therefore expose facts, selected-scope overlays, and highlight annotations; the frontend decides whether to draw them as a static chart, animation, or 3D scene.

Pattern and 成格 highlighting should be produced by feature/rule layers as structured annotations, not by the renderer. Until the rule engine can identify real patterns, highlight fields should be reserved or empty rather than hard-coded in UI code.

## 3. Render Layer

The Render Layer turns snapshot/read-model data into human-facing display formats.

The first concrete renderer is `render`'s plain text chart-stack renderer. It consumes `ChartStackSnapshot` and produces deterministic text output for demos and debugging. It does not generate chart facts, derive temporal periods, localize terminology, evaluate rules, or produce interpretation.

Future renderers may include CLI, TUI, web, GUI, SVG/HTML, or 3D views. They should remain consumers of snapshot/read-model structures.

## 4. Feature Extraction Layer

The Feature Extraction Layer converts a chart into a semantic feature graph.

Important feature dimensions include:

- calendar and boundary settings;
- twelve-palace features;
- star placement and star semantics;
- mutagen flows;
- palace relations such as opposite palace and triads;
- patterns and combinations;
- temporal activation from decadal/yearly/monthly/daily/hourly scopes;
- strength scores and counter-evidence.

The goal is not to write prose, but to expose features that a rule engine can evaluate.

## 5. Rule Engine Layer

The Rule Engine Layer maps features into structured claims.

Rules should not directly emit final narrative text. A rule should emit:

- domain;
- theme;
- polarity;
- strength;
- evidence;
- counter-evidence;
- source metadata.

This makes rule matching testable and allows multiple rules to be aggregated before generating a report.

## 6. Narrative Layer

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

## Evidence-first interpretation

Every interpretive claim should be traceable to chart evidence. This enables debugging, review, rule tuning, and future empirical validation.
