//! Shared brightness discovery helpers for 日月并明 and 日月反背.
//!
//! Re-exports the canonical bright/dim classification so brightness-driven
//! detectors share one source. The predicates classify a brightness state; the
//! named pattern decides whether bright or dim is the formation it wants.

pub(crate) use crate::rules::pattern::query::{is_bright, is_dim};
