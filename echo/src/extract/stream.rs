use echo_core::body::{BodyExt, Bytes};
use echo_core::{BoxError, Request};
use futures_util::{Stream, TryStreamExt};

pub fn stream(request: &mut Request) -> impl Stream<Item = Result<Bytes, BoxError>> {
    std::mem::take(request.body_mut())
        .stream()
        .try_filter_map(|frame| async move { Ok(frame.into_data().ok()) })
}
