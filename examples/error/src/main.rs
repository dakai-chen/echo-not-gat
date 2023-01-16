use std::convert::Infallible;
use std::fmt;
use std::future::Future;

use echo::extract::ExtractPathError;
use echo::http::StatusCode;
use echo::middleware::CatchErrorMiddleware;
use echo::response::IntoResponse;
use echo::route::{RouteError, RouteErrorKind, Router};
use echo::server::Server;
use echo::service::{Service, ServiceExt};
use echo::{extract, BoxError, Request, Response};

#[tokio::main]
async fn main() {
    Server::bind("127.0.0.1:3000".parse().unwrap())
        .serve(app())
        .await
        .unwrap();
}

fn app() -> impl Service<
    Request,
    Response = Response,
    Error = Infallible,
    Future = impl Future<Output = Result<Response, Infallible>> + Send,
> {
    Router::new()
        .mount(div)
        // 错误处理
        .with(CatchErrorMiddleware::new(handle_error))
}

#[echo::route("/div/:n/:d", method = "GET")]
async fn div(req: Request) -> Result<impl IntoResponse, BoxError> {
    let n = extract::path::<u64>(&req, "n")?;
    let d = extract::path::<u64>(&req, "d")?;

    let r = n.checked_div(d).ok_or_else(|| DivideByZeroError)?;

    Ok(format!("{n} / {d} = {r}"))
}

async fn handle_error(err: BoxError) -> Result<Response, Infallible> {
    // 路由错误
    if let Some(e) = err.downcast_ref::<RouteError>() {
        let status_code = match e.kind() {
            // 自定义404响应
            RouteErrorKind::NotFound => StatusCode::NOT_FOUND,
            // 自定义405响应
            RouteErrorKind::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
        };
        return Ok(status_code.into_response());
    }

    // 提取器错误
    if let Some(e) = err.downcast_ref::<ExtractPathError>() {
        return Ok((StatusCode::BAD_REQUEST, format!("{e}")).into_response());
    }

    // 除以零错误
    if let Some(e) = err.downcast_ref::<DivideByZeroError>() {
        return Ok(format!("{e}").into_response());
    }

    // 其他错误
    Ok((StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")).into_response())
}

#[derive(Debug)]
struct DivideByZeroError;

impl fmt::Display for DivideByZeroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Divide by zero error")
    }
}

impl std::error::Error for DivideByZeroError {}
