use std::fmt;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;

use super::{Body, BodyExt, BoxError, Frame, SizeHint};

pub struct BoxBody {
    inner: Pin<Box<dyn Body<Error = BoxError> + Send>>,
}

impl BoxBody {
    pub fn new<B>(body: B) -> Self
    where
        B: Body + Send + 'static,
        B::Error: Into<BoxError>,
    {
        Self {
            inner: Box::pin(body.map_err(Into::into)),
        }
    }
}

impl Body for BoxBody {
    type Error = BoxError;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Bytes>, Self::Error>>> {
        self.inner.as_mut().poll_frame(cx)
    }

    fn size_hint(&self) -> SizeHint {
        self.inner.size_hint()
    }
}

impl fmt::Debug for BoxBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoxBody").finish()
    }
}

impl Default for BoxBody {
    fn default() -> Self {
        BoxBody::new(())
    }
}
