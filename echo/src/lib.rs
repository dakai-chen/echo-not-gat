#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]

pub mod extract;
pub mod response;

pub use echo_core::*;

pub mod middleware {
    pub use echo_middleware::core::*;
    pub use echo_middleware::util::*;
}

pub mod route {
    pub use echo_route::*;
}

#[cfg(feature = "server")]
pub mod server {
    pub use echo_server::*;
}

#[cfg(feature = "macros")]
pub use echo_macros::route;

#[cfg(feature = "ws")]
pub mod ws {
    pub use echo_ws::*;
}
