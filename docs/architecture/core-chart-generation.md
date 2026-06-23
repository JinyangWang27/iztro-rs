# Core Chart Generation Architecture

This document describes the current natal chart-generation design in
`iztro-rs`. It covers structural responsibilities and extension boundaries,
not TypeScript compatibility details or interpretation behavior.

## 1. Core axes

Chart generation separates two independent axes:

```rust
ChartAlgorithmKind::{QuanShu, Zhongzhou, Placeholder}
ChartPlane::{Heaven, Earth, Human}
```

`ChartAlgorithmKind` identifies the algorithm family. `ChartPlane` identifies
the plane variant within that family. Zhongzhou Heaven is therefore not
equivalent to QuanShu.

Valid combinations:

```text
QuanShu + Heaven
Zhongzhou + Heaven
Zhongzhou + Earth
Zhongzhou + Human
Placeholder + Heaven
```

Invalid combinations:

```text
QuanShu + Earth/Human
Placeholder + Earth/Human
```

Invalid combinations fail with `ChartError::UnsupportedChartPlane` at the
facade boundary. `ChartProfile` combines the method profile, including its
algorithm family, with the selected plane. Generated `Chart` values retain this
profile and are self-describing.

## 2. Generation pipeline

The lunar and solar facades share one deterministic generation path:

```text
request validation
  -> calendar/date resolution
  -> effective lunar month/day derivation
  -> minimal chart construction
  -> optional Zhongzhou chart-plane re-anchoring
  -> deterministic natal star placement strategy
  -> final Chart with ChartProfile metadata
```

`by_solar` converts its input into lunar facts and delegates to the lunar path;
it does not own separate star-placement behavior.

The facade validates the algorithm and plane before calendar work. It derives
normalized natal inputs, resolves the Life Palace anchor, invokes deterministic
construction, and attaches the final `ChartProfile`.

This keeps request adaptation, plane dispatch, minimal chart structure, and
star placement as separate responsibilities.

## 3. Zhongzhou Heaven, Earth, and Human semantics

Current Zhongzhou plane rules:

```text
Zhongzhou Heaven:
  existing Zhongzhou generation without re-anchoring
Zhongzhou Earth:
  re-anchor Life Palace to the Heaven chart's Body Palace branch
Zhongzhou Human:
  re-anchor Life Palace to the Heaven chart's Spirit / 福德宫 branch
```

Earth and Human first derive the relevant branch from a Heaven-plane minimal
chart. That branch becomes an explicit `NatalChartAnchor`.

Generation then rebuilds an anchor-aware minimal chart from deterministic
inputs. It does not mutate palace names or other facts on a completed chart.
Palace names, palace stems, and the five-element bureau are consequently
calculated from the selected Life Palace anchor.

The Body Palace branch remains the original calculated Body Palace fact.
Re-anchoring changes the Life Palace anchor, not the birth-time-derived Body
Palace branch.

## 4. Minimal chart versus star placement

Minimal chart construction owns the twelve palace branches, canonical palace
names, palace stems, Life Palace, Body Palace branch, and five-element bureau.

Star placement adds major, minor, adjective, and decorative runtime stars on top
of that complete structure.

Star placers consume chart facts; they do not decide the chart plane or
reinterpret the Life Palace anchor.

Placement orchestration goes through `NatalStarPlacementStrategy`.
`DeterministicNatalStarPlacementStrategy` is the default composition for the
supported natal star families. Future algorithm-specific behavior can use new
strategy implementations instead of scattering algorithm checks across
individual placers.

## 5. Diagnostics and invariants

`Chart` exposes a compact structural diagnostic API:

```rust
Chart::diagnostic_snapshot()
ChartDiagnosticSnapshot
PalaceDiagnosticSnapshot
```

The chart snapshot records the algorithm, plane, palace count, Life and Body
Palace branches, five-element bureau, and per-palace summaries. Each palace
summary records its name, branch, stem, typed star count, and decorative star
count.

Diagnostic snapshots are owned, serializable structural debugging views derived
only from existing `Chart` facts. They are useful for tests and troubleshooting.

They are not semantic or golden fixtures, TypeScript parity data, a stable
compatibility export, a rendered UI schema, or an interpretation model.

Invariant tests generate charts through both `by_lunar` and `by_solar` for
QuanShu Heaven and Zhongzhou Heaven, Earth, and Human. They assert:

```text
- exactly 12 palaces
- every PalaceName appears once
- every EarthlyBranch appears once
- Life Palace exists
- Body Palace branch exists and resolves
- five-element bureau exists
- requested algorithm and plane match ChartProfile metadata
- palace lookup helpers round-trip every palace
- typed and decorative star contexts resolve to existing palaces
- diagnostic snapshots match generated chart structure
```

These checks lock down internal structure without claiming upstream placement
parity.

## 6. What this is not

This work does not introduce:

```text
- TypeScript iztro fixture parity
- golden chart snapshots
- new star placement logic
- new Zhongzhou rules
- rendering or UI behavior
```

If an invariant fails, first determine whether the assertion is too strong, the
test setup is wrong, or an existing generation defect was exposed.

## 7. Future extension points

Potential follow-up work:

```text
- TypeScript parity fixtures for Zhongzhou chart planes
- broader invariant helpers when concrete needs emerge
- richer diagnostic snapshots when troubleshooting justifies them
- more algorithm-specific strategy implementations
- architecture documents for individual star-placement families
```

Extensions should preserve the separation between algorithm family, chart
plane, minimal structure, star placement, diagnostics, and presentation.
