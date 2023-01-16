use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;

use super::{Body, Frame};

#[must_use = "futures don't do anything unless polled"]
#[derive(Debug)]
pub struct Next<'a, B: ?Sized>(pub(crate) &'a mut B);

impl<'a, B: Body + Unpin + ?Sized> Future for Next<'a, B> {
    type Output = Option<Result<Frame<Bytes>, B::Error>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.0).poll_frame(cx)
    }
}

#[must_use = "futures don't do anything unless polled"]
#[derive(Debug)]
pub struct Data<'a, B: ?Sized>(pub(crate) &'a mut B);

impl<'a, B: Body + Unpin + ?Sized> Future for Data<'a, B> {
    type Output = Option<Result<Bytes, B::Error>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            return match Pin::new(&mut self.0).poll_frame(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(None) => Poll::Ready(None),
                Poll::Ready(Some(Ok(frame))) => {
                    if let Ok(data) = frame.into_data() {
                        Poll::Ready(Some(Ok(data)))
                    } else {
                        continue;
                    }
                }
                Poll::Ready(Some(Err(err))) => Poll::Ready(Some(Err(err))),
            };
        }
    }
}
