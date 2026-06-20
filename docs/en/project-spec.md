# Project Specification

`iztro-rs` is a Rust implementation for Zi Wei Dou Shu chart generation, feature extraction, and eventually rule-based interpretation.

## Product boundary

The project is an engine and library first, not an end-user fortune-telling product. It should expose deterministic data structures and interpretation primitives that can be consumed by CLI tools, TUI tools, MCP/coding-agent integrations, Python bindings, WebAssembly bindings, GUI applications, or external systems.

A future application may reproduce a 文墨天机-style static chart first and later experiment with timeline or 3D views. Those frontends should consume renderer-neutral view models and annotations from the library rather than re-deriving chart facts, mutating natal charts, or embedding astrology logic in UI code.

The current frontend direction is static-chart first:

1. stabilize typed chart facts and fixture-backed compatibility;
2. expose renderer-neutral snapshots and static chart view models;
3. build a local GUI over those view models;
4. add runtime i18n through `crates/iztro-i18n`;
5. add TUI and MCP/tooling consumers once the query/export surface is stable;
6. treat timeline and 3D views as later consumers of reusable static chart frames.

## Core objectives

1. Provide strongly typed chart models for Zi Wei Dou Shu.
2. Reproduce `iztro`-compatible chart-generation behavior where applicable.
3. Separate chart facts from interpretation logic.
4. Represent chart interpretation as structured claims with evidence.
5. Support bilingual documentation and future multilingual output.
6. Leave room for multiple schools, method profiles, and interpretation styles.
7. Provide renderer-neutral chart view models that can support a static palace-grid GUI first and later serve as timeline/3D frames.
8. Provide stable structured outputs suitable for future TUI, MCP, bindings, and application frontends.

## Early scope

The first implementation phase should cover:

- Rust workspace scaffolding;
- core domain enums and structs;
- chart data model serialization;
- fixture-backed compatibility testing;
- renderer-neutral snapshots and view models;
- feature extraction traits and placeholder implementations;
- rule schema and rule matching skeleton;
- deterministic report data structures;
- plain text rendering and local static GUI prototypes.

## Out of scope for early versions

- Full Zi Wei Dou Shu rule knowledge base.
- Complete flying-mutagen or Qin Tian rule systems.
- Automatic empirical weight learning.
- LLM-first chart interpretation.
- Production-quality narrative generation.
- Production-quality GUI, 3D renderer, or SaaS product.
- Full API compatibility with every `iztro` public method.
- MCP tools that expose unstable or prose-only interfaces.

## Design principles

- Determinism before expressiveness.
- Strong typing before stringly typed rule logic.
- Structured claims before prose.
- Evidence-first interpretation.
- Strategy/profile composition for multi-school support.
- Renderer-neutral view models before concrete frontends.
- Runtime localization at presentation boundaries, not inside core chart facts.
- English engineering specs and Chinese terminology maintained together.
