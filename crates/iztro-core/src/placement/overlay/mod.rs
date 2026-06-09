//! Temporal overlay placement/activation builders (流年, 大限, …).
//!
//! These layer onto the model-only horoscope overlays in
//! [`crate::model::chart::horoscope`]. The `mutagen` module holds the shared
//! Heavenly Stem mutagen-activation builder reused by [`yearly`] and
//! [`decadal`].

pub mod decadal;
pub mod yearly;

pub(crate) mod mutagen;
