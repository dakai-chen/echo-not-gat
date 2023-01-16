#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]

mod error;
mod method;
mod params;
mod router;
mod util;

pub mod future;

pub use error::{RouteError, RouteErrorKind, RouterError};
pub use method::{
    any, connect, delete, get, head, method, options, patch, post, put, trace, IntoMethodRoute,
    MethodRoute,
};
pub use params::PathParams;
pub use router::{Route, Router};
