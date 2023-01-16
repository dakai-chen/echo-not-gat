use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;
use futures_util::Stream;
use pin_project_lite::pin_project;

use super::{Body, Frame, SizeHint};

pin_project! {
    #[derive(Debug, Clone, Copy, Default)]
    pub struct StreamBody<S> {
        #[pin]
        stream: S,
    }
}

impl<S> StreamBody<S> {
    pub fn new(stream: S) -> Self {
        Self { stream }
    }

    pub fn get_ref(&self) -> &S {
        &self.stream
    }

    pub fn get_mut(&mut self) -> &mut S {
        &mut self.stream
    }

    pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut S> {
        self.project().stream
    }

    pub fn into_inner(self) -> S {
        self.stream
    }
}

impl<S, E> Body for StreamBody<S>
where
    S: Stream<Item = Result<Frame<Bytes>, E>>,
{
    type Error = E;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Bytes>, Self::Error>>> {
        self.project().stream.poll_next(cx)
    }
}

impl<S: Stream> Stream for StreamBody<S> {
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().stream.poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.stream.size_hint()
    }
}

pin_project! {
    #[derive(Debug, Clone, Copy, Default)]
    pub struct BodyStream<B> {
        #[pin]
        body: B,
    }
}

impl<B> BodyStream<B> {
    pub fn new(body: B) -> Self {
        Self { body }
    }

    pub fn get_ref(&self) -> &B {
        &self.body
    }

    pub fn get_mut(&mut self) -> &mut B {
        &mut self.body
    }

    pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut B> {
        self.project().body
    }

    pub fn into_inner(self) -> B {
        self.body
    }
}

impl<B: Body> Stream for BodyStream<B> {
    type Item = Result<Frame<Bytes>, B::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.as_mut().project().body.poll_frame(cx)
    }
}

impl<B: Body> Body for BodyStream<B> {
    type Error = B::Error;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Bytes>, Self::Error>>> {
        self.project().body.poll_frame(cx)
    }

    fn size_hint(&self) -> SizeHint {
        self.body.size_hint()
    }
}
