# Contributing

Thank you for contributing to `iztro-rs`.

This project is still in early design and scaffolding. Contributions should preserve the architecture documented under `docs/`.

## Development principles

Please read:

- [Project specification](docs/en/project-spec.md)
- [Architecture](docs/en/architecture.md)
- [Engineering principles](docs/en/engineering-principles.md)
- [Rule engine](docs/en/rule-engine.md)
- [Compatibility policy](docs/en/compatibility.md)

## TDD expectation

For deterministic logic, use test-driven development where practical:

1. add or update a failing test;
2. implement the smallest deterministic change;
3. refactor while keeping tests green.

This applies especially to:

- chart-generation logic;
- calendar and boundary behavior;
- index arithmetic;
- star placement;
- feature extraction;
- rule matching;
- claim aggregation;
- deterministic report rendering.

## Rust-oriented design

Rust does not use class inheritance, so this project interprets SOLID through Rust idioms:

- small modules;
- small traits;
- explicit contracts;
- composition over inheritance;
- enums for closed sets;
- traits for extensible strategies;
- dependency inversion through traits and method profiles.

Avoid large all-purpose traits or modules that mix chart generation, feature extraction, rule evaluation, and report rendering.

## Documentation policy

Major documentation should be bilingual when applicable.

- English is canonical for engineering specifications.
- Chinese is canonical for Zi Wei Dou Shu terminology.

If a PR updates a major document in only one language, explain why.

## Pull request checklist

- [ ] The change respects the documented layer boundaries.
- [ ] Deterministic behavior has tests or a documented reason for not adding tests yet.
- [ ] Public-facing terminology is consistent with the glossary.
- [ ] Documentation is updated in English and Chinese where applicable.
- [ ] New architectural decisions are captured in ADRs if needed.
