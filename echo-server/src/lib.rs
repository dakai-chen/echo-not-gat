#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]

mod compat;
mod graceful_shutdown;
use graceful_shutdown::GracefulShutdown;

use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;
use std::time::Duration;

use echo_core::response::IntoResponse;
use echo_core::service::{Service, ServiceExt};
use echo_core::{BoxError, Request};
use hyper::server::conn::http1;
use tokio::net::{TcpListener, TcpStream};

#[derive(Debug, Clone)]
struct Options {
    addr: SocketAddr,
}

#[derive(Debug, Clone)]
pub struct Server {
    options: Options,
    http1: http1::Builder,
}

impl Server {
    pub fn bind(addr: SocketAddr) -> Self {
        Self {
            options: Options { addr },
            http1: http1::Builder::new(),
        }
    }

    pub fn cfg_http1<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut http1::Builder),
    {
        f(&mut self.http1);
        self
    }

    pub async fn serve<S>(self, service: S) -> Result<(), BoxError>
    where
        S: Service<Request, Error = Infallible> + Send + Sync + 'static,
        S::Response: IntoResponse,
        S::Future: Send,
    {
        self.serve_with_graceful_shutdown(service, std::future::pending())
            .await
    }

    pub async fn serve_with_graceful_shutdown<S, G>(
        self,
        service: S,
        signal: G,
    ) -> Result<(), BoxError>
    where
        S: Service<Request, Error = Infallible> + Send + Sync + 'static,
        S::Response: IntoResponse,
        S::Future: Send,
        G: Future<Output = Option<Duration>> + Send + 'static,
    {
        tokio::pin!(signal);

        let service = service.boxed_arc();

        let graceful = GracefulShutdown::new();
        let listener = TcpListener::bind(self.options.addr).await?;

        let timeout = loop {
            tokio::select! {
                timeout = signal.as_mut() => {
                    break timeout;
                }
                conn = listener.accept() => {
                    let (conn, _) = conn?;

                    let service = service.clone();
                    let service = extract_addr(service, &conn);
                    let service = compat::context_switch(service);
                    let service = hyper::service::service_fn(move |req| service.call(req));

                    let conn = self.http1.serve_connection(conn, service).with_upgrades();
                    let conn = graceful.watch(conn);

                    tokio::spawn(conn);
                }
            }
        };

        graceful.shutdown(timeout).await;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocalAddr(pub SocketAddr);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RemoteAddr(pub SocketAddr);

fn extract_addr<S, R>(
    service: S,
    conn: &TcpStream,
) -> impl Service<
    Request,
    Response = R,
    Error = Infallible,
    Future = impl Future<Output = Result<R, Infallible>> + Send,
>
where
    S: Service<Request, Response = R, Error = Infallible> + Send + Sync + 'static,
    S::Future: Send,
{
    let raddr = conn.peer_addr().ok().map(RemoteAddr);
    let laddr = conn.local_addr().ok().map(LocalAddr);

    service.map_request(move |mut request: Request| {
        let extensions = request.extensions_mut();
        raddr.map(|addr| extensions.insert(addr));
        laddr.map(|addr| extensions.insert(addr));
        request
    })
}
