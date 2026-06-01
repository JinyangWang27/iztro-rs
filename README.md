# iztro-rs

A Rust implementation for Zi Wei Dou Shu (紫微斗数) chart generation, feature extraction, and eventually rule-based interpretation.

> Status: early design and scaffolding. The project is not yet a complete charting or interpretation engine.

中文说明见 [README.zh-CN.md](README.zh-CN.md).

## Goals

`iztro-rs` aims to provide:

- a strongly typed Rust core for Zi Wei Dou Shu chart data;
- chart-generation compatibility with `iztro` where applicable;
- a feature extraction layer for palaces, stars, mutagens, relations, patterns, and temporal activations;
- a rule engine that emits structured claims rather than prose;
- deterministic report generation with optional future LLM-assisted narrative rendering;
- future bindings for CLI, Python, and WebAssembly use cases.

## Non-goals for early versions

Early versions will not attempt to be:

- a fortune-telling SaaS product;
- an LLM-first interpretation system;
- a complete multi-school Zi Wei Dou Shu interpretation engine;
- a drop-in clone of every public `iztro` API;
- a replacement for human judgement in classical or modern metaphysical interpretation.

## Initial architecture

The project is designed around four layers:

1. **Core Chart Layer** — deterministic chart facts and domain models.
2. **Feature Extraction Layer** — structured semantic features derived from a chart.
3. **Rule Engine Layer** — rules map features into structured claims with evidence.
4. **Narrative Layer** — claims are rendered into human-readable reports.

See [docs/en/architecture.md](docs/en/architecture.md) for details.

## Compatibility with iztro

This project is inspired by [`iztro`](https://github.com/SylarLong/iztro), a lightweight Zi Wei Dou Shu astrolabe generation library. Early chart-generation behavior should be validated against `iztro` where applicable, while internal Rust APIs may diverge to favor stronger typing and long-term extensibility.

See [docs/en/compatibility.md](docs/en/compatibility.md).

## Documentation

English documentation is canonical for engineering specifications. Chinese documentation is maintained as a first-class translation and is canonical for Zi Wei Dou Shu terminology.

- [Project specification](docs/en/project-spec.md)
- [Architecture](docs/en/architecture.md)
- [Roadmap](docs/en/roadmap.md)
- [Compatibility](docs/en/compatibility.md)
- [Terminology](docs/en/terminology.md)
- [Rule engine](docs/en/rule-engine.md)
- [Multilingual documentation](docs/en/i18n.md)

## Acknowledgements

This project is inspired by [`iztro`](https://github.com/SylarLong/iztro), licensed under the MIT License. The early compatibility target of `iztro-rs` is to reproduce compatible chart-generation behavior where applicable.

## License

MIT.
