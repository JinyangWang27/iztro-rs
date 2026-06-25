# Engineering Principles

`iztro-rs` favors deterministic, testable, strongly typed, and modular design. These principles are intended to guide both human contributors and coding agents.

## Core values

- Determinism before expressiveness.
- Strong typing before stringly typed logic.
- Small modules before monolithic engines.
- Explicit contracts before implicit conventions.
- Evidence-first interpretation.
- Tests as executable specifications.

## Test-driven development

TDD is strongly encouraged for deterministic domain logic.

For every new chart-generation rule, feature extractor, rule evaluator, or report-rendering behavior:

1. add or update a failing test first;
2. implement the smallest deterministic change;
3. refactor while keeping tests green.

Exploratory rule knowledge may begin as draft data, but once a rule is accepted into a maintained rule set, it should have tests that show how it is matched and what claim it emits.

## Rust-oriented SOLID

This project uses Rust-oriented modular design principles inspired by SOLID. The goal is not to imitate class inheritance from Java or C#, but to preserve the useful design constraints through Rust idioms.

### Single Responsibility

Each module should have one reason to change.

Examples:

- Core chart models do not contain interpretation prose.
- Feature extraction does not perform rule matching.
- Rule evaluation does not render final reports.
- Narrative rendering does not recalculate chart facts.

### Open/Closed

New methods, schools, mutagen strategies, feature extractors, rule sets, and narrative styles should be added through method profiles, traits, enums, or data files rather than scattered edits across large functions.

Use traits for plugin-like extensibility and enums for small closed sets of known options.

### Substitutability

Rust has no class inheritance, but trait implementations still have contracts. Implementations of a trait should be behaviorally substitutable within the trait's documented expectations.

For example, two chart-generation strategies may differ in placement rules, but both should return a valid chart with complete palace structure, valid star placements, and consistent error semantics.

### Interface Segregation

Prefer small traits over one large engine trait.

Good examples:

- `ChartGenerator`;
- `FeatureExtractor`;
- `ReportRenderer`.

Avoid a single trait that combines chart generation, feature extraction, rule evaluation, localization, rule loading, and report rendering.

### Dependency Inversion

High-level orchestration should depend on traits, method profiles, and stable data contracts rather than concrete implementations.

This makes it possible to test the pipeline with fixtures, mock generators, or alternative strategies.

## Testing categories

Expected test categories include:

- unit tests for pure helpers, index arithmetic, enum conversion, and small deterministic functions;
- golden tests for compatibility with selected `iztro` fixtures;
- snapshot tests for serialized chart outputs and structured claims;
- rule tests for feature input to matched claim output;
- integration tests for chart to features to claims to deterministic report.

## Coding-agent expectations

Coding agents should not introduce large abstractions without tests or documentation. When scaffolding, they should preserve layer boundaries and add minimal interfaces that can be validated by tests.

## Documentation expectations

Engineering decisions that affect architecture, compatibility, testing policy, or public API should be captured in ADRs.
