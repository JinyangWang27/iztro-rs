//! Deterministic 安星 (star placement) algorithms.
//!
//! [`natal`] holds the natal-chart placement pipeline (palace layout,
//! life/body and palace-stem rules, the major/minor/adjective star placers, and
//! the public natal chart builders). [`overlay`] holds the temporal overlay
//! activation builders (流年, 大限, …) layered on top of the model-only
//! horoscope overlays in [`crate::model::chart::horoscope`].

pub mod natal;
pub mod overlay;
