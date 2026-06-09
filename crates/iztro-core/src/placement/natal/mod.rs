//! Deterministic natal-chart placement.
//!
//! Inputs ([`input`]) feed the layered builders: [`minimal`] computes the empty
//! and minimal natal charts (palace layout, life/body palace, palace stems, and
//! the five-element bureau), and [`supported`] places the major, minor, and
//! adjective stars on top. The per-family placers live in [`major`], [`minor`],
//! and [`adjective`]; the supporting rules in [`life_body`] and [`palace_stems`].

pub mod adjective;
pub mod input;
pub mod life_body;
pub mod major;
pub mod minimal;
pub mod minor;
pub mod palace_stems;
pub mod supported;
