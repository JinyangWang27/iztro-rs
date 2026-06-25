# Rule Engine

The rule engine converts extracted features into structured claims. It should not directly generate final prose.

> The **active, concrete implementation is the classical rule engine**
> (Chinese-first 《紫微斗数全书》). It lives under `rules::classical` and is the
> sole rule engine. See [`rules/rule-engine.md`](./rules/rule-engine.md) and
> [ADR 0007](./adr/0007-classical-rule-engine.md) for the implemented design,
> corpus format, and pipeline.
>
> The rest of this document is **conceptual design vocabulary** for the
> feature → claim vision. It is not a second implemented engine, and the
> generic condition/effect sketch below is illustrative only — the classical
> engine deliberately uses data-driven metadata plus hand-coded predicates
> rather than a generic condition DSL.

## Rule shape (conceptual)

Conceptually a rule has three parts:

1. Metadata.
2. Conditions.
3. Effects.

Illustrative TOML sketch (not the implemented corpus format — see
[`rules/rule-engine.md`](./rules/rule-engine.md) for the real classical corpus
shape):

```toml
id = "career.wuqu_huaquan.in_career"
domain = "career"
source = "seed"
school = "basic"
priority = 50

[condition]
palace = "career_palace"
has_star = "wuqu"
star_mutagen = "quan"

[effect]
themes = ["resource_control", "responsibility", "management_pressure"]
polarity = "mixed_positive"
strength = 0.75
evidence_template = "Career Palace contains Wu Qu with Quan transformation."
```

## Claims

A matched rule emits a claim:

- domain;
- theme or themes;
- polarity;
- strength;
- evidence;
- counter-evidence;
- source metadata.

Claims are intermediate artifacts. They can be tested, aggregated, filtered, translated, or rendered.

## Aggregation

Multiple claims about the same domain should be aggregated before narrative rendering.

For example:

- career execution;
- career authority;
- career pressure;
- career support;
- career volatility.

These should synthesize into a domain-level assessment, such as `pressure-driven career growth`, rather than being printed as unrelated sentences.

## Conflict resolution

The engine must support qualifying evidence. A chart can contain supportive and obstructive signals at the same time.

Examples:

- strong career palace plus Ji transformation;
- auspicious stars plus malefic stars;
- strong natal pattern but weak temporal activation;
- favorable palace but damaged opposite palace.

The output should distinguish advantage, risk, condition, and timing.

## Rule sources

Rules may come from:

- classical texts;
- school-specific traditions;
- expert notes;
- existing readings used for weight calibration;
- empirical case feedback.

Source metadata should be preserved so rules can be audited.

## Weighting

Initial weights may be hand-authored. Later versions may tune weights from existing readings or real-world case labels.

Weights should not be treated as truth. They are model parameters for prioritizing claims.

## Narrative separation

Rules should not contain full paragraphs of final reading text. They may contain evidence templates and short labels. Final human-readable reports belong to the Narrative Layer.
