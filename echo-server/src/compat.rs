use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use echo_core::body::{Body as EchoBody, BodyExt, BoxBody, Bytes, Frame, SizeHint};
use echo_core::response::IntoResponse;
use echo_core::service::{Service, ServiceExt};
use echo_core::{BoxError, Request, Response};
use hyper::body::{Body as HyperBody, Incoming};
use pin_project_lite::pin_project;

pub fn context_switch<S>(
    service: S,
) -> impl Service<
    Request<Incoming>,
    Response = Response<EchoToHyper>,
    Error = Infallible,
    Future = impl Future<Output = Result<Response<EchoToHyper>, Infallible>> + Send,
>
where
    S: Service<Request, Error = Infallible>,
    S::Response: IntoResponse,
    S::Future: Send,
{
    service
        .map_request(|request: Request<Incoming>| request.map(HyperToEcho::to))
        .map_response(|response: S::Response| response.into_response().map(EchoToHyper::to))
}

pin_project! {
    struct HyperToEcho {
        #[pin]
        body: Incoming,
    }
}

impl HyperToEcho {
    fn to(body: Incoming) -> BoxBody {
        Self { body }.boxed()
    }
}

impl EchoBody for HyperToEcho {
    type Error = BoxError;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Bytes>, Self::Error>>> {
        self.project().body.poll_frame(cx).map_err(From::from)
    }

    fn size_hint(&self) -> SizeHint {
        self.body.size_hint()
    }
}

pin_project! {
    pub struct EchoToHyper {
        #[pin]
        body: BoxBody,
    }
}

impl EchoToHyper {
    fn to(body: BoxBody) -> Self {
        Self { body }
    }
}

impl HyperBody for EchoToHyper {
    type Data = Bytes;
    type Error = BoxError;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        self.project()
            .body
            .poll_frame(cx)
            .map_err(|e| e.to_string().into())
    }

    fn size_hint(&self) -> SizeHint {
        self.body.size_hint()
    }
}
