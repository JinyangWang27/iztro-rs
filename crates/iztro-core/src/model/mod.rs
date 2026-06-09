//! Strongly typed Zi Wei Dou Shu domain models.
//!
//! This module groups the immutable domain facts: value objects
//! ([`calendar`], [`ganzhi`], [`sexagenary`], [`bureau`], [`profile`]), star
//! facts ([`star`]), and chart facts ([`chart`]). Deterministic placement
//! algorithms live in [`crate::placement`]; these modules carry only data.

pub mod bureau;
pub mod calendar;
pub mod chart;
pub mod ganzhi;
pub mod profile;
pub mod sexagenary;
pub mod star;
