use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use echo_core::BoxError;
use pin_project_lite::pin_project;

pin_project! {
    #[project = RouteFutureProj]
    pub enum RouteFuture<F> {
        Future {
            #[pin]
            fut: F,
        },
        Error {
            err: Option<BoxError>,
        },
        Complete,
    }
}

impl<F, R, E> Future for RouteFuture<F>
where
    F: Future<Output = Result<R, E>>,
    E: Into<BoxError>,
{
    type Output = Result<R, BoxError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let output = match self.as_mut().project() {
            RouteFutureProj::Future { fut } => fut.poll(cx).map_err(Into::into),
            RouteFutureProj::Error { err } => {
                Poll::Ready(Err(err.take().expect("polled after completion")))
            }
            RouteFutureProj::Complete => {
                panic!("polled after completion")
            }
        };
        if output.is_ready() {
            self.as_mut().set(RouteFuture::Complete);
        }
        output
    }
}
