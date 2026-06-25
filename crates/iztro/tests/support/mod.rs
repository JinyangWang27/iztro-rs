//! Shared, test-only support code for the 《紫微斗数全书》 source-inventory tests.
//!
//! This module is compiled into each integration-test binary that declares
//! `mod support;` (currently `classical_source_inventory.rs` and
//! `classical_source_coverage.rs`). It is **not** a runtime module: nothing in
//! `src/` references it, and it never enters the chart-evaluation path. Keeping
//! the deserialization shapes here avoids duplicating them across test files
//! while preserving the layer boundary (corpus-tracking data stays test-only).

pub mod classical_source;
