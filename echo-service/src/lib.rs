#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "util")]
#[macro_use]
mod macros;

#[cfg(feature = "util")]
mod util;
#[cfg(feature = "util")]
pub use util::*;

use core::future::Future;

pub trait Service<Request> {
    type Response;

    type Error;

    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn call(&self, request: Request) -> Self::Future;
}

impl<'a, S, Request> Service<Request> for &'a mut S
where
    S: Service<Request> + ?Sized + 'a,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn call(&self, request: Request) -> Self::Future {
        (**self).call(request)
    }
}

impl<'a, S, Request> Service<Request> for &'a S
where
    S: Service<Request> + ?Sized + 'a,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn call(&self, request: Request) -> Self::Future {
        (**self).call(request)
    }
}

#[cfg(feature = "alloc")]
impl<S, Request> Service<Request> for alloc::boxed::Box<S>
where
    S: Service<Request> + ?Sized,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn call(&self, request: Request) -> Self::Future {
        (**self).call(request)
    }
}

#[cfg(feature = "alloc")]
impl<S, Request> Service<Request> for alloc::rc::Rc<S>
where
    S: Service<Request> + ?Sized,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn call(&self, request: Request) -> Self::Future {
        (**self).call(request)
    }
}

#[cfg(feature = "alloc")]
impl<S, Request> Service<Request> for alloc::sync::Arc<S>
where
    S: Service<Request> + ?Sized,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn call(&self, request: Request) -> Self::Future {
        (**self).call(request)
    }
}

pub trait Middleware<S> {
    type Service;

    fn transform(self, service: S) -> Self::Service;
}
