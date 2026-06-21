//! Deterministic natal-chart placement.
//!
//! Inputs ([`input`]) feed the layered builders: [`minimal`] computes the empty
//! and minimal natal charts (palace layout, life/body palace, palace stems, and
//! the five-element bureau), and [`supported`] places the major, minor, and
//! adjective stars on top. The per-family placers live in [`major`], [`minor`],
//! and [`adjective`]; the supporting rules in [`life_body`] and [`palace_stems`].
//!
//! Above the per-family placers, [`strategy`] defines the high-level
//! [`strategy::NatalStarPlacementStrategy`] orchestration: it owns the full
//! supported-star pipeline and is the extension point for future Zhongzhou
//! 中州地盘 / 中州人盘 algorithms.

pub mod adjective;
pub mod decorative;
pub mod input;
pub mod life_body;
pub mod major;
pub mod minimal;
pub mod minor;
pub mod palace_stems;
pub mod strategy;
pub mod supported;
