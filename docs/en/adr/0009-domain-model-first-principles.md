# ADR 0009: Domain Model First Principles

Status: Accepted

## Context

The rule-engine refactoring ending with the shared `rules::query` layer made the domain boundaries explicit enough to document as first principles rather than isolated implementation notes.

The important changes are:

- selected-state semantics were made explicit through `EffectiveChartState` and `RuleEvaluationContext`;
- pattern detection moved from `core::pattern` to `rules::pattern` because patterns are rules, not core chart facts;
- classical source rules and pattern rules became sibling rule engines under `rules`;
- generic read-only rule queries moved into `rules::query` so `rules::pattern` and `rules::classical` no longer depend on each other for chart lookups;
- 昌曲夹命 became the first selected-state classical rule slice, proving that classical rules can evaluate a temporal frame without mutating natal facts.

This ADR records the domain model boundaries that should guide future work.

## Decision

The project should preserve the following first principles.

### 1. Core owns chart facts, not interpretation

`core` owns deterministic facts and value objects:

- birth context and calculation policy;
- stems, branches, zodiac, NaYin, bureau, palaces, stars, mutagens;
- natal chart facts;
- temporal overlay facts;
- `EffectiveChartState` as the selected effective chart state.

`core` must not own rule engines, GUI state, renderer layout, localized prose, or narrative interpretation.

### 2. There is one natal chart; time adds overlays

A `Chart` is the immutable natal fact aggregate. Temporal state is additive:

```text
Chart
  + TemporalLayer[]
  -> HoroscopeChart
```

Temporal layers may contribute scoped star placements, decorative facts, palace-name layouts, and mutagen activations. They must not rewrite natal palaces or duplicate natal facts.

### 3. Branch is the stable coordinate; palace name is frame-relative

`EarthlyBranch` is the stable palace-cell coordinate. `PalaceName` is assigned by a palace-name frame.

The natal chart and each temporal layer may supply a different palace-name frame over the same branch ring:

```text
branch = stable coordinate
palace name = meaning in a selected frame
```

A selected Yearly view changes the active palace frame; it does not change the natal chart.

### 4. Effective state is selected frame plus active fact stack

`EffectiveChartState` represents the selected state a rule should usually read:

```text
selected palace frame
+ active scopes
+ natal facts
+ visible ancestor/current temporal overlays
+ provenance for every effective fact
```

This is the default semantic surface for ordinary selected-view rules. Source/layer-specific rules may still read one source layer explicitly, but that should be visible in the helper name and rule intent.

Selected temporal chart state controls the palace frame and which facts are visible. It does **not** alter star identity matching. A selected Yearly view may make 流曲 (`LiuQu`) visible, but 流曲 is not 文曲 (`WenQu`): star matching is exact by default, so a base-star condition never silently matches a same-scope flow star. 文曲 and 流曲 may share `StarFamily::Qu`, but family membership is taxonomy, not equality. A detector that wants family-level matching, or the exact flow blade of a specific layer, must ask for it explicitly (`StarSelector::Family`, `StarFamily::member_in_scope`); the palace-frame scope is never overloaded to imply base↔flow equivalence.

### 5. Context describes chart state; rule metadata describes rule identity

`RuleEvaluationContext` describes what chart state is being evaluated. It should not classify the rule.

Do not put rule identity into the context, for example:

```text
is_pattern = true
```

That is the wrong axis. A pattern is a kind of rule output or rule facet, not a different chart-state context.

Current wrappers:

```text
RuleEvaluationContext
├── PatternContext
└── ClassicalRuleContext
```

The wrappers exist for rule-engine-specific API compatibility and request/query surfaces. They should not diverge in chart-state semantics.

### 6. Patterns are rules

A pattern (格局) is a structural rule over chart state. It currently emits `PatternDetection` rather than `Claim`, but it is still rule-engine code.

Therefore:

```text
rules::pattern = pattern rule engine
rules::classical = classical source/claim rule engine
```

`core::pattern` must not return as a rule engine namespace because it would make `core` own rule logic.

### 7. Classical source text is provenance, not prose generation

For classical rules, verbatim Chinese source text is authoritative provenance and must remain separate from interpretation.

`source_text_zh_hans` must quote the cited source clause. Interpretive content belongs in normalized notes, claim metadata, i18n strings, commentary metadata, or future narrative layers.

### 8. Rule queries belong under `rules::query` when shared

Generic read-only rule queries belong in `rules::query` when they are used by more than one rule engine.

Pattern-specific wrappers may remain in `rules::pattern::query`; classical-specific predicates may remain in `rules::classical::predicates`. But neither engine should depend on the other engine's query module for generic star, palace, clamp, selected-state, or brightness lookup.

Target dependency shape:

```text
rules::query
├── rules::pattern
└── rules::classical
```

Not:

```text
rules::pattern::query
└── rules::classical
```

### 9. Analysis coordinates rule engines; it does not interpret

`analysis` coordinates selected-view, per-layer rule evaluation:

```text
TemporalAnalysisContext
+ AnalysisLayerKey
+ AnalysisLayerRequest
-> AnalysisLayerResult
```

It may build the right context, truncate active scopes for caching, and compose `rules::pattern` with `rules::classical`. It should not generate prose, mutate charts, or become a second rule engine.

### 10. Projection and facade are read-model boundaries

Projection/facade code turns facts and analysis results into renderer-friendly read models. GUI, CLI, TUI, MCP, and future 3D surfaces should consume these read models instead of re-deriving chart or rule logic.

A renderer may decide layout, labels, colors, and interaction. It must not own placement, effective-state, or rule-evaluation semantics.

## Current module map

```text
core/
  calculation/         input normalization and calculation policy
  model/               chart facts and value objects
  placement/           deterministic placement builders
  rule_context.rs      shared selected-state context, transitional home

rules/
  query.rs             shared read-only rule queries
  pattern/             pattern / 格局 detection
  classical/           source hits, claims, diagnostics

analysis/              per-layer selected-view coordination
projection/            serializable GUI/API read models
facade/                orchestration around projections and core facts
render/                deterministic rendering consumers
reading/               report structures and narrative-facing contracts
```

`RuleEvaluationContext` currently lives in `core` because it was introduced while pattern code still lived under `core`. Moving it to `rules::context` may be a future cleanup, but it is not required for the current architecture to be coherent.

## Anti-patterns

Avoid these unless a future ADR explicitly changes the boundary:

- putting `is_pattern`, `is_classical`, or similar rule identity flags into `RuleEvaluationContext`;
- making `core` depend on `rules`;
- restoring `core::pattern` as a forwarding shim;
- silently treating temporal palace names as natal palace names;
- evaluating selected-view rules by guessing the deepest active scope instead of using an explicit selected frame;
- duplicating generic query helpers inside both pattern and classical engines;
- storing interpretation or claim prose in `source_text_zh_hans`;
- letting GUI/rendering code derive chart facts or rule results.

## Consequences

This keeps the model boring and testable:

- chart generation is deterministic and fixture-friendly;
- temporal overlays remain additive and inspectable;
- selected-frame bugs are easier to catch;
- pattern and classical rules can evolve independently while sharing query helpers;
- GUI and reporting layers can consume structured facts without becoming astrology engines.

The cost is a slightly larger number of boundary types (`EffectiveChartState`, `RuleEvaluationContext`, `PatternContext`, `ClassicalRuleContext`, analysis keys). That cost is intentional: these names encode real domain distinctions that were previously easy to conflate.
