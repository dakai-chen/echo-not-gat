mod and_then;
#[cfg(feature = "alloc")]
mod boxed;
#[cfg(feature = "alloc")]
mod boxed_local;
mod ext;
mod map_err;
mod map_future;
mod map_request;
mod map_response;
mod map_result;
mod middleware_fn;
mod service_fn;
mod then;

pub use and_then::AndThen;
#[cfg(feature = "alloc")]
pub use boxed::{ArcService, BoxCloneService, BoxService};
#[cfg(feature = "alloc")]
pub use boxed_local::{LocalBoxCloneService, LocalBoxService, RcService};
pub use ext::ServiceExt;
pub use map_err::MapErr;
pub use map_future::MapFuture;
pub use map_request::MapRequest;
pub use map_response::MapResponse;
pub use map_result::MapResult;
pub use middleware_fn::{middleware_fn, MiddlewareFn};
pub use service_fn::{service_fn, ServiceFn};
pub use then::Then;

pub mod future {
    pub use super::and_then::AndThenFuture;
    #[cfg(feature = "alloc")]
    pub use super::boxed::BoxFuture;
    #[cfg(feature = "alloc")]
    pub use super::boxed_local::LocalBoxFuture;
    pub use super::map_err::MapErrFuture;
    pub use super::map_response::MapResponseFuture;
    pub use super::map_result::MapResultFuture;
    pub use super::then::ThenFuture;
}
