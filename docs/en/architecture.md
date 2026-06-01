# Architecture

`iztro-rs` uses a layered architecture. Each layer has a clear responsibility and should avoid leaking concerns into adjacent layers.

## 1. Core Chart Layer

The Core Chart Layer contains deterministic chart facts. It should not contain interpretation prose.

Examples:

- birth context and calendar options;
- heavenly stems and earthly branches;
- palaces;
- stars;
- brightness;
- mutagens;
- decadal and yearly structures;
- method profile metadata.

The output of this layer is a structured chart object.

## 2. Feature Extraction Layer

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

## 3. Rule Engine Layer

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

## 4. Narrative Layer

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
