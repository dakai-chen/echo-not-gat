use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::{Buf, BufMut, Bytes};
use http_body::{Frame, SizeHint};
use pin_project_lite::pin_project;

use crate::{Body, BodyExt};

pin_project! {
    #[derive(Debug)]
    pub struct Collect<B> {
        #[pin]
        body: B,
    }
}

impl<B: Body> Collect<B> {
    pub fn new(body: B) -> Self {
        Self { body }
    }

    pub async fn bytes(self) -> Result<Bytes, B::Error> {
        let body = self.body;

        futures_util::pin_mut!(body);

        let buf1 = if let Some(buf) = body.data().await {
            buf?
        } else {
            return Ok(Bytes::new());
        };

        let buf2 = if let Some(buf) = body.data().await {
            buf?
        } else {
            return Ok(buf1);
        };

        let cap = buf1
            .remaining()
            .saturating_add(buf2.remaining())
            .saturating_add((body.size_hint().lower() as usize).min(1024 * 16));

        let mut vec = Vec::with_capacity(cap);

        vec.put(buf1);
        vec.put(buf2);

        while let Some(buf) = body.data().await {
            vec.put(buf?);
        }

        Ok(vec.into())
    }
}

impl<B: Body> Body for Collect<B> {
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
