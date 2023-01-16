use crate::{Middleware, Service};

use super::{AndThen, MapErr, MapFuture, MapRequest, MapResponse, MapResult, Then};
#[cfg(feature = "alloc")]
use super::{
    ArcService, BoxCloneService, BoxService, LocalBoxCloneService, LocalBoxService, RcService,
};

pub trait ServiceExt<Req>: Service<Req> {
    fn with<T>(self, middleware: T) -> T::Service
    where
        Self: Sized,
        T: Middleware<Self>,
    {
        middleware.transform(self)
    }

    fn and_then<F>(self, f: F) -> AndThen<Self, F>
    where
        Self: Sized,
    {
        AndThen::new(self, f)
    }

    fn then<F>(self, f: F) -> Then<Self, F>
    where
        Self: Sized,
    {
        Then::new(self, f)
    }

    fn map_err<F>(self, f: F) -> MapErr<Self, F>
    where
        Self: Sized,
    {
        MapErr::new(self, f)
    }

    fn map_future<F>(self, f: F) -> MapFuture<Self, F>
    where
        Self: Sized,
    {
        MapFuture::new(self, f)
    }

    fn map_request<F>(self, f: F) -> MapRequest<Self, F>
    where
        Self: Sized,
    {
        MapRequest::new(self, f)
    }

    fn map_response<F>(self, f: F) -> MapResponse<Self, F>
    where
        Self: Sized,
    {
        MapResponse::new(self, f)
    }

    fn map_result<F>(self, f: F) -> MapResult<Self, F>
    where
        Self: Sized,
    {
        MapResult::new(self, f)
    }

    #[cfg(feature = "alloc")]
    fn boxed(self) -> BoxService<Req, Self::Response, Self::Error>
    where
        Self: Sized + Send + Sync + 'static,
        Self::Future: Send + 'static,
    {
        BoxService::new(self)
    }

    #[cfg(feature = "alloc")]
    fn boxed_local(self) -> LocalBoxService<Req, Self::Response, Self::Error>
    where
        Self: Sized + 'static,
        Self::Future: 'static,
    {
        LocalBoxService::new(self)
    }

    #[cfg(feature = "alloc")]
    fn boxed_clone(self) -> BoxCloneService<Req, Self::Response, Self::Error>
    where
        Self: Sized + Clone + Send + Sync + 'static,
        Self::Future: Send + 'static,
    {
        BoxCloneService::new(self)
    }

    #[cfg(feature = "alloc")]
    fn boxed_clone_local(self) -> LocalBoxCloneService<Req, Self::Response, Self::Error>
    where
        Self: Sized + Clone + 'static,
        Self::Future: 'static,
    {
        LocalBoxCloneService::new(self)
    }

    #[cfg(feature = "alloc")]
    fn boxed_arc(self) -> ArcService<Req, Self::Response, Self::Error>
    where
        Self: Sized + Send + Sync + 'static,
        Self::Future: Send + 'static,
    {
        ArcService::new(self)
    }

    #[cfg(feature = "alloc")]
    fn boxed_rc(self) -> RcService<Req, Self::Response, Self::Error>
    where
        Self: Sized + 'static,
        Self::Future: 'static,
    {
        RcService::new(self)
    }
}

impl<S, Req> ServiceExt<Req> for S where S: Service<Req> {}
