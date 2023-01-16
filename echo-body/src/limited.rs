use std::fmt;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::{Buf, Bytes};
use pin_project_lite::pin_project;

use super::{Body, BoxError, Frame, SizeHint};

pin_project! {
    #[derive(Debug, Clone, Copy)]
    pub struct Limited<B> {
        remaining: usize,
        #[pin]
        inner: B,
    }
}

impl<B> Limited<B> {
    pub fn new(inner: B, limit: usize) -> Self {
        Self {
            remaining: limit,
            inner,
        }
    }
}

impl<B> Body for Limited<B>
where
    B: Body,
    B::Error: Into<BoxError>,
{
    type Error = BoxError;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Bytes>, Self::Error>>> {
        let this = self.project();
        let res = match this.inner.poll_frame(cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(None) => None,
            Poll::Ready(Some(Ok(frame))) => {
                if let Some(data) = frame.data_ref() {
                    if data.remaining() > *this.remaining {
                        *this.remaining = 0;
                        Some(Err(LengthLimitError.into()))
                    } else {
                        *this.remaining -= data.remaining();
                        Some(Ok(frame))
                    }
                } else {
                    Some(Ok(frame))
                }
            }
            Poll::Ready(Some(Err(err))) => Some(Err(err.into())),
        };
        Poll::Ready(res)
    }

    fn size_hint(&self) -> SizeHint {
        match u64::try_from(self.remaining) {
            Ok(n) => {
                let mut hint = self.inner.size_hint();
                if hint.lower() >= n {
                    hint.set_exact(n)
                } else if let Some(max) = hint.upper() {
                    hint.set_upper(n.min(max))
                } else {
                    hint.set_upper(n)
                }
                hint
            }
            Err(_) => self.inner.size_hint(),
        }
    }
}

#[derive(Debug)]
pub struct LengthLimitError;

impl fmt::Display for LengthLimitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("length limit exceeded")
    }
}

impl std::error::Error for LengthLimitError {}
