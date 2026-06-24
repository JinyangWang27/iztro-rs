//! Input calculation policy layer.
//!
//! This layer sits *before* chart generation. It models how a raw birth date
//! and civil clock time become a resolved local date/time and 时辰:
//!
//! ```text
//! raw birth date + civil clock time
//!   -> optional apparent solar time adjustment
//!   -> resolved local date/time
//!   -> derive time branch / time index
//!   -> existing natal chart generation
//! ```
//!
//! [`ChartAlgorithmKind`](crate::core::model::profile::ChartAlgorithmKind),
//! [`ChartPlane`](crate::core::model::profile::ChartPlane), and
//! [`ChartCalculationConfig`] are separate axes.
//!
//! Apparent solar time is an input calculation policy. It normalises birth clock
//! time before the chart is generated. It does not define a new algorithm and
//! does not define a new chart plane.

mod config;
mod resolve;

pub use config::{
    ApparentSolarTimeConfig, ChartCalculationConfig, ClockBirthTime, EquationOfTimePolicy,
    Longitude, SolarTimePolicy, UtcOffset,
};
pub use resolve::ResolvedBirthDateTime;
pub(crate) use resolve::resolve_birth_datetime;
