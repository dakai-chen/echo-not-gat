use bytes::Bytes;
use echo_core::body::BodyExt;
use echo_core::{BoxError, Request};

pub async fn bytes(request: &mut Request) -> Result<Bytes, BoxError> {
    std::mem::take(request.body_mut()).collect().bytes().await
}
