//! Provides traits for statistical computation.

pub use self::iter_statistics_ext::*;
pub use self::statistics_ext::*;
pub use self::ranks::*;

mod iter_statistics_ext;
mod statistics_ext;
mod ranks;
