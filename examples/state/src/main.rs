use std::convert::Infallible;
use std::future::Future;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use echo::middleware::{AddExtensionMiddleware, CatchErrorMiddleware};
use echo::response::IntoResponse;
use echo::route::Router;
use echo::server::Server;
use echo::service::{Service, ServiceExt};
use echo::{extract, Request, Response};

#[tokio::main]
async fn main() {
    Server::bind("127.0.0.1:3000".parse().unwrap())
        .serve(app(Count::new()))
        .await
        .unwrap();
}

fn app<S: Clone + Send + Sync + 'static>(
    state: S,
) -> impl Service<
    Request,
    Response = Response,
    Error = Infallible,
    Future = impl Future<Output = Result<Response, Infallible>> + Send,
> {
    Router::new()
        .mount(count)
        // 添加状态
        .with(AddExtensionMiddleware::new(move || state.clone()))
        .with(CatchErrorMiddleware::new(|e| async move {
            Ok(format!("{e}").into_response())
        }))
}

#[echo::route("/count", method = "GET")]
async fn count(req: Request) -> Result<impl IntoResponse, Infallible> {
    // 提取状态
    let count = extract::extension::<Count>(&req).unwrap();

    Ok(format!("{}", count.inc().await))
}

#[derive(Clone)]
struct Count(Arc<AtomicU64>);

impl Count {
    fn new() -> Self {
        Self(Arc::new(AtomicU64::new(0)))
    }

    async fn inc(&self) -> u64 {
        self.0.fetch_add(1, Ordering::AcqRel)
    }
}
