use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use echo_core::service::{Middleware, Service};
use futures_core::ready;
use pin_project_lite::pin_project;

#[derive(Clone, Copy)]
pub struct CatchErrorMiddleware<F> {
    f: F,
}

impl<F> CatchErrorMiddleware<F> {
    pub fn new(f: F) -> Self {
        Self { f }
    }
}

impl<S, F> Middleware<S> for CatchErrorMiddleware<F> {
    type Service = CatchError<S, F>;

    fn transform(self, service: S) -> Self::Service {
        CatchError {
            inner: service,
            f: self.f,
        }
    }
}

impl<F> fmt::Debug for CatchErrorMiddleware<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CatchErrorMiddleware")
            .field("f", &std::any::type_name::<F>())
            .finish()
    }
}

#[derive(Clone, Copy)]
pub struct CatchError<S, F> {
    inner: S,
    f: F,
}

impl<S, F, Req, Fut, Err> Service<Req> for CatchError<S, F>
where
    S: Service<Req>,
    F: FnOnce(S::Error) -> Fut + Clone,
    Fut: Future<Output = Result<S::Response, Err>>,
{
    type Response = S::Response;
    type Error = Err;
    type Future = CatchErrorFuture<S::Future, Fut, F>;

    fn call(&self, request: Req) -> Self::Future {
        CatchErrorFuture::Incomplete {
            fut: self.inner.call(request),
            f: self.f.clone(),
        }
    }
}

impl<S, F> fmt::Debug for CatchError<S, F>
where
    S: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CatchError")
            .field("inner", &self.inner)
            .field("f", &std::any::type_name::<F>())
            .finish()
    }
}

pin_project! {
    #[project = CatchErrorFutureProj]
    #[project_replace = CatchErrorFutureProjReplace]
    pub enum CatchErrorFuture<Fut1, Fut2, F> {
        Incomplete {
            #[pin]
            fut: Fut1,
            f: F,
        },
        Error {
            #[pin]
            fut: Fut2,
        },
        Complete,
    }
}

impl<Fut1, Fut2, F, Res, Err1, Err2> Future for CatchErrorFuture<Fut1, Fut2, F>
where
    Fut1: Future<Output = Result<Res, Err1>>,
    Fut2: Future<Output = Result<Res, Err2>>,
    F: FnOnce(Err1) -> Fut2,
{
    type Output = Result<Res, Err2>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            return match self.as_mut().project() {
                CatchErrorFutureProj::Incomplete { fut, .. } => {
                    let output = ready!(fut.poll(cx));
                    match self.as_mut().project_replace(CatchErrorFuture::Complete) {
                        CatchErrorFutureProjReplace::Incomplete { f, .. } => match output {
                            Ok(res) => Poll::Ready(Ok(res)),
                            Err(err) => {
                                self.as_mut().set(CatchErrorFuture::Error { fut: f(err) });
                                continue;
                            }
                        },
                        CatchErrorFutureProjReplace::Error { .. } => unreachable!(),
                        CatchErrorFutureProjReplace::Complete => unreachable!(),
                    }
                }
                CatchErrorFutureProj::Error { fut } => {
                    let res = ready!(fut.poll(cx));
                    self.set(CatchErrorFuture::Complete);
                    Poll::Ready(res)
                }
                CatchErrorFutureProj::Complete => {
                    panic!("polled after completion")
                }
            };
        }
    }
}
