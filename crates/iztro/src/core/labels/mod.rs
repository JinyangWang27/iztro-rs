//! Localized display labels for facade/export snapshots.
//!
//! Core domain models stay language-neutral: stems, branches, palaces, stars,
//! mutagens, brightness, kinds, and families are modeled as Rust enums and
//! serialized with stable machine-readable keys. This module is a thin,
//! presentation-oriented adapter that maps those neutral identities to Chinese
//! display strings for the facade/export layer, because Zi Wei Dou Shu is
//! primarily consumed in Chinese.
//!
//! This is intentionally *not* a complete multilingual/i18n subsystem: it only
//! exposes deterministic, table-driven `*_zh` lookups consumed by the facade
//! snapshots. Full multilingual support and complete upstream localized-string
//! parity are deferred.

pub mod zh_cn;
