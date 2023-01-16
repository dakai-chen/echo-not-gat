use core::fmt;

use crate::Middleware;

pub fn middleware_fn<F>(f: F) -> MiddlewareFn<F> {
    MiddlewareFn::new(f)
}

#[derive(Clone, Copy)]
pub struct MiddlewareFn<F> {
    f: F,
}

impl<F> MiddlewareFn<F> {
    pub fn new(f: F) -> Self {
        Self { f }
    }
}

impl<F, S1, S2> Middleware<S1> for MiddlewareFn<F>
where
    F: FnOnce(S1) -> S2,
{
    type Service = S2;

    fn transform(self, service: S1) -> Self::Service {
        (self.f)(service)
    }
}

impl<F> fmt::Debug for MiddlewareFn<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MiddlewareFn")
            .field("f", &core::any::type_name::<F>())
            .finish()
    }
}
