//! Shared helpers for the fixture-backed tests. Split into focused submodules
//! and re-exported flat so call sites keep using `common::{...}`:
//!
//! - [`fixtures`] — fixture constants and JSON loading helpers;
//! - [`builders`] — fixture `input` block → built `Chart` through the facade;
//! - [`targets`] — temporal-layer target/scope parsing helpers;
//! - [`normalize`] — key parsing and `supported_fields` assertion helpers.

#![allow(dead_code, unused_imports)]

pub mod builders;
pub mod fixtures;
pub mod normalize;
pub mod targets;

pub use builders::*;
pub use fixtures::*;
pub use normalize::*;
pub use targets::*;
