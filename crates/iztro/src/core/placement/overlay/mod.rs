//! Temporal overlay placement/activation builders (流年, 大限, …).
//!
//! These layer onto the model-only horoscope overlays in
//! [`crate::core::model::chart::horoscope`]. The `mutagen` module holds the shared
//! Heavenly Stem mutagen-activation builder reused by [`yearly`] and
//! [`decadal`].

pub mod age;
pub mod daily_horoscope;
pub mod decadal;
pub mod decadal_horoscope;
pub mod flow;
pub mod horoscope_stack;
pub mod hourly_horoscope;
pub mod monthly_horoscope;
pub mod yearly;
pub mod yearly_horoscope;

pub(crate) mod mutagen;
