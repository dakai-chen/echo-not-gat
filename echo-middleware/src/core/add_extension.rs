use std::fmt;

use echo_core::service::{Middleware, Service};
use echo_core::Request;

#[derive(Clone, Copy)]
pub struct AddExtensionMiddleware<F> {
    f: F,
}

impl<F> AddExtensionMiddleware<F> {
    pub fn new(f: F) -> Self {
        Self { f }
    }
}

impl<S, F> Middleware<S> for AddExtensionMiddleware<F> {
    type Service = AddExtension<S, F>;

    fn transform(self, service: S) -> Self::Service {
        AddExtension {
            inner: service,
            f: self.f,
        }
    }
}

impl<F> fmt::Debug for AddExtensionMiddleware<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AddExtensionMiddleware")
            .field("f", &std::any::type_name::<F>())
            .finish()
    }
}

#[derive(Clone, Copy)]
pub struct AddExtension<S, F> {
    inner: S,
    f: F,
}

impl<S, F, B, T> Service<Request<B>> for AddExtension<S, F>
where
    S: Service<Request<B>>,
    F: Fn() -> T,
    T: Send + Sync + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn call(&self, mut request: Request<B>) -> Self::Future {
        request.extensions_mut().insert((self.f)());
        self.inner.call(request)
    }
}

impl<S, F> fmt::Debug for AddExtension<S, F>
where
    S: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AddExtension")
            .field("inner", &self.inner)
            .field("f", &std::any::type_name::<F>())
            .finish()
    }
}
