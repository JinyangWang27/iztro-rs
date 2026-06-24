//! Strongly typed Zi Wei Dou Shu domain models.
//!
//! This module groups the immutable domain facts: value objects
//! ([`calendar`], [`nayin`], [`bureau`], [`profile`]), star facts ([`star`]),
//! and chart facts ([`chart`]). Low-level stem/branch and sexagenary-cycle
//! primitives ([`ganzhi`]) are owned by `iztro-rs`. Deterministic placement
//! algorithms live in [`crate::core::placement`]; these modules carry only data.

pub mod bureau;
pub mod calendar;
pub mod chart;
pub mod ganzhi;
pub mod master;
pub mod nayin;
pub mod profile;
pub mod star;
pub mod zodiac;
