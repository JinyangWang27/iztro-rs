# Project Specification

`iztro-rs` is a Rust implementation for Zi Wei Dou Shu chart generation, feature extraction, and eventually rule-based interpretation.

## Product boundary

The project is an engine and library, not an end-user fortune-telling product. It should expose deterministic data structures and interpretation primitives that can be consumed by CLI tools, Python bindings, WebAssembly bindings, or external applications.

## Core objectives

1. Provide strongly typed chart models for Zi Wei Dou Shu.
2. Reproduce `iztro`-compatible chart-generation behavior where applicable.
3. Separate chart facts from interpretation logic.
4. Represent chart interpretation as structured claims with evidence.
5. Support bilingual documentation and future multilingual output.
6. Leave room for multiple schools, method profiles, and interpretation styles.

## Early scope

The first implementation phase should cover:

- Rust workspace scaffolding;
- core domain enums and structs;
- chart data model serialization;
- feature extraction traits and placeholder implementations;
- rule schema and rule matching skeleton;
- deterministic report data structures;
- golden-test infrastructure design for later `iztro` comparison.

## Out of scope for early versions

- Full Zi Wei Dou Shu rule knowledge base.
- Complete flying-mutagen or Qin Tian rule systems.
- Automatic empirical weight learning.
- LLM-first chart interpretation.
- Production-quality narrative generation.
- Full API compatibility with every `iztro` public method.

## Design principles

- Determinism before expressiveness.
- Strong typing before stringly typed rule logic.
- Structured claims before prose.
- Evidence-first interpretation.
- Strategy/profile composition for multi-school support.
- English engineering specs and Chinese terminology maintained together.
