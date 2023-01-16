use std::convert::Infallible;

use echo::response::IntoResponse;
use echo::server::Server;
use echo::service::service_fn;
use echo::Request;

#[tokio::main]
async fn main() {
    Server::bind("127.0.0.1:3000".parse().unwrap())
        .serve(service_fn(hello))
        .await
        .unwrap();
}

async fn hello(_: Request) -> Result<impl IntoResponse, Infallible> {
    Ok("Hello, World!")
}
