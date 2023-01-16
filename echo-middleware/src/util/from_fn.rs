use std::fmt;
use std::future::Future;

use echo_core::service::{Middleware, Service};

pub fn from_fn<F>(f: F) -> FromFnMiddleware<F> {
    FromFnMiddleware::new(f)
}

#[derive(Clone, Copy)]
pub struct FromFnMiddleware<F> {
    f: F,
}

impl<F> FromFnMiddleware<F> {
    pub fn new(f: F) -> Self {
        Self { f }
    }
}

impl<S, F> Middleware<S> for FromFnMiddleware<F> {
    type Service = FromFn<S, F>;

    fn transform(self, service: S) -> Self::Service {
        FromFn { service, f: self.f }
    }
}

impl<F> fmt::Debug for FromFnMiddleware<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FromFnMiddleware")
            .field("f", &std::any::type_name::<F>())
            .finish()
    }
}

#[derive(Clone, Copy)]
pub struct FromFn<S, F> {
    service: S,
    f: F,
}

impl<S, F, Req, Res, Err, Fut> Service<Req> for FromFn<S, F>
where
    S: Service<Req>,
    F: Fn(Req, &S) -> Fut,
    Fut: Future<Output = Result<Res, Err>>,
{
    type Response = Res;
    type Error = Err;
    type Future = Fut;

    fn call(&self, request: Req) -> Self::Future {
        (self.f)(request, &self.service)
    }
}

impl<S, F> fmt::Debug for FromFn<S, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FromFn")
            .field("service", &std::any::type_name::<S>())
            .field("f", &std::any::type_name::<F>())
            .finish()
    }
}
