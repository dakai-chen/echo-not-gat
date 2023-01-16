#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]

pub mod response;
pub use response::Response;

pub mod body {
    pub use echo_body::*;
}

pub mod http {
    pub use http::*;
}

pub mod service {
    pub use echo_service::*;
}

pub type Request<B = echo_body::BoxBody> = http::Request<B>;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
