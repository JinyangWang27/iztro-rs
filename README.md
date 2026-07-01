# iztro-rs

[![Crates.io Version](https://img.shields.io/crates/v/iztro)](https://crates.io/crates/iztro)
[![Crates.io Downloads](https://img.shields.io/crates/d/iztro)](https://crates.io/crates/iztro)
[![docs.rs](https://docs.rs/iztro/badge.svg)](https://docs.rs/iztro)
[![CI](https://github.com/JinyangWang27/iztro-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/JinyangWang27/iztro-rs/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/JinyangWang27/iztro-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/JinyangWang27/iztro-rs)
[![License](https://img.shields.io/crates/l/iztro)](https://crates.io/crates/iztro)

A Rust implementation for Zi Wei Dou Shu (紫微斗数) chart generation, feature extraction, and eventually rule-based interpretation.

> Status: pre-1.0. Chart generation is implemented and fixture-backed against
> `iztro@2.5.8`, with a renderer-neutral snapshot, a desktop GUI prototype, and
> runtime i18n. Feature extraction, the classical rule engine, and narrative
> interpretation are partial and still evolving. APIs may change before 1.0.

中文说明见 [README.zh-CN.md](README.zh-CN.md).

## Installation

The public API ships as a single crate:

```
cargo add iztro
```

The crate keeps clear internal domain boundaries as modules — `core`,
`features`, `rules`, `reading`, and `render` — while the stable user-facing
core API is also re-exported from the crate root.

## Quick demo

The current supported natal chart fact surface can flow from a typed solar input through `by_solar` into a
renderer-neutral stack snapshot, then into the `iztro::render` plain text demo. The excerpt below is from
fixture-backed supported fields.

```rust
use iztro::render::render_chart_stack_text;
use iztro::{
    ChartAlgorithmKind, EarthlyBranch, Gender, MethodProfile, SolarChartRequest, SolarDay,
    SolarMonth, by_solar,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = SolarChartRequest::builder()
        .solar_year(1990)
        .solar_month(SolarMonth::new(5)?)
        .solar_day(SolarDay::new(17)?)
        .birth_time(EarthlyBranch::Chen)
        .gender(Gender::Female)
        .method_profile(MethodProfile::new(
            "readme_demo",
            ChartAlgorithmKind::QuanShu,
            "README plain text demo",
        ))
        .build()?;

    let chart = by_solar(request)?;
    let snapshot = chart.stack_snapshot();

    println!("{}", render_chart_stack_text(&snapshot));
    Ok(())
}
```

Run the example with:

```bash
cargo run -p iztro --example plain_text
```

Abbreviated real output excerpt:

```text
Chart Stack
birth: Lunar 1990-4-23, time Chen, gender Female
method: readme_demo / QuanShu
life_palace_branch: Chou
body_palace_branch: You
five_element_bureau: Fire6

Layer 0: Natal
[Si] Career / Xin
roles: NatalPalace(Career)
typed: TianLiang, HuoXing, SanTai, TianGui, PoSui
decorative: Jue, XiaoHaoBoshi, BingFuSuiqian, WangShen
...
[Chou] Life / Ji
roles: NatalPalace(Life)
typed: TaiYang, TaiYin, TianKui
decorative: MuYu, XiShenBoshi, LongDeSuiqian, TianSha
...
```

See the [demo page](docs/en/demo.md) and the
[full captured output](docs/examples/plain_text_1990_05_17_chen_female.txt).

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

The project is designed around layered boundaries:

1. **Core Chart Layer** — deterministic chart facts and domain models.
2. **Snapshot / Read Model Layer** — renderer-neutral chart and GUI/API read models.
3. **Runtime Localization Layer** — presentation-boundary i18n for labels and UI text.
4. **Render / Application Layer** — text, GUI, future TUI/MCP/3D consumers.
5. **Feature / Rule / Narrative Layers** — interpretation-facing layers that consume structured facts rather than parsing rendered text.

See [docs/en/architecture.md](docs/en/architecture.md) for the layer model and
[ADR 0009](docs/en/adr/0009-domain-model-first-principles.md) for the domain
model first principles.

## Compatibility with iztro

This project is inspired by [`iztro`](https://github.com/SylarLong/iztro), a lightweight Zi Wei Dou Shu astrolabe generation library. Early chart-generation behavior should be validated against `iztro` where applicable, while internal Rust APIs may diverge to favor stronger typing and long-term extensibility.

See [docs/en/compatibility.md](docs/en/compatibility.md).

## Documentation

English documentation is canonical for engineering specifications. Chinese documentation is maintained as a first-class translation and is canonical for Zi Wei Dou Shu terminology.

- [Project specification](docs/en/project-spec.md)
- [Architecture](docs/en/architecture.md)
- [Domain model first principles](docs/en/adr/0009-domain-model-first-principles.md)
- [Core chart generation architecture](docs/architecture/core-chart-generation.md)
- [Roadmap](docs/en/roadmap.md)
- [Compatibility](docs/en/compatibility.md)
- [Terminology](docs/en/terminology.md)
- [Rule engine overview](docs/en/rule-engine.md)
- [Classical rule engine](docs/en/rules/rule-engine.md)
- [Multilingual documentation](docs/en/i18n.md)

## Acknowledgements

This project is inspired by [`iztro`](https://github.com/SylarLong/iztro), licensed under the MIT License. The early compatibility target of `iztro-rs` is to reproduce compatible chart-generation behavior where applicable.

## License

MIT.
