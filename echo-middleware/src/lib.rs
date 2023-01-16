#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]

#[cfg(feature = "core")]
pub mod core;

#[cfg(feature = "util")]
pub mod util;
