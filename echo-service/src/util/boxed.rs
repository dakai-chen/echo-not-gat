use core::{fmt, future::Future, pin::Pin};

use alloc::{boxed::Box, sync::Arc};

use crate::{Service, ServiceExt};

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub struct BoxService<Req, Res, Err> {
    inner: Box<
        dyn Service<Req, Response = Res, Error = Err, Future = BoxFuture<'static, Result<Res, Err>>>
            + Send
            + Sync,
    >,
}

impl<Req, Res, Err> BoxService<Req, Res, Err> {
    pub fn new<S>(inner: S) -> Self
    where
        S: Service<Req, Response = Res, Error = Err> + Send + Sync + 'static,
        S::Future: Send + 'static,
    {
        Self {
            inner: Box::new(inner.map_future(|f| Box::pin(f) as _)),
        }
    }
}

impl<Req, Res, Err> Service<Req> for BoxService<Req, Res, Err> {
    type Response = Res;
    type Error = Err;
    type Future = BoxFuture<'static, Result<Res, Err>>;

    fn call(&self, request: Req) -> BoxFuture<'static, Result<Res, Err>> {
        self.inner.call(request)
    }
}

impl<Req, Res, Err> fmt::Debug for BoxService<Req, Res, Err> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("BoxService").finish()
    }
}

pub struct BoxCloneService<Req, Res, Err> {
    inner: Box<
        dyn CloneService<
                Req,
                Response = Res,
                Error = Err,
                Future = BoxFuture<'static, Result<Res, Err>>,
            > + Send
            + Sync,
    >,
}

impl<Req, Res, Err> Clone for BoxCloneService<Req, Res, Err> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone_box(),
        }
    }
}

impl<Req, Res, Err> BoxCloneService<Req, Res, Err> {
    pub fn new<S>(inner: S) -> Self
    where
        S: Service<Req, Response = Res, Error = Err> + Clone + Send + Sync + 'static,
        S::Future: Send + 'static,
    {
        BoxCloneService {
            inner: Box::new(inner.map_future(|f| Box::pin(f) as _)),
        }
    }
}

impl<Req, Res, Err> Service<Req> for BoxCloneService<Req, Res, Err> {
    type Response = Res;
    type Error = Err;
    type Future = BoxFuture<'static, Result<Res, Err>>;

    fn call(&self, request: Req) -> BoxFuture<'static, Result<Res, Err>> {
        self.inner.call(request)
    }
}

impl<Req, Res, Err> fmt::Debug for BoxCloneService<Req, Res, Err> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("BoxCloneService").finish()
    }
}

pub struct ArcService<Req, Res, Err> {
    inner: Arc<
        dyn Service<Req, Response = Res, Error = Err, Future = BoxFuture<'static, Result<Res, Err>>>
            + Send
            + Sync,
    >,
}

impl<Req, Res, Err> Clone for ArcService<Req, Res, Err> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<Req, Res, Err> ArcService<Req, Res, Err> {
    pub fn new<S>(inner: S) -> Self
    where
        S: Service<Req, Response = Res, Error = Err> + Send + Sync + 'static,
        S::Future: Send + 'static,
    {
        Self {
            inner: Arc::new(inner.map_future(|f| Box::pin(f) as _)),
        }
    }
}

impl<Req, Res, Err> Service<Req> for ArcService<Req, Res, Err> {
    type Response = Res;
    type Error = Err;
    type Future = BoxFuture<'static, Result<Res, Err>>;

    fn call(&self, request: Req) -> BoxFuture<'static, Result<Res, Err>> {
        self.inner.call(request)
    }
}

impl<Req, Res, Err> fmt::Debug for ArcService<Req, Res, Err> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("ArcService").finish()
    }
}

trait CloneService<R>: Service<R> {
    fn clone_box(
        &self,
    ) -> Box<
        dyn CloneService<R, Response = Self::Response, Error = Self::Error, Future = Self::Future>
            + Send
            + Sync,
    >;
}

impl<S, R> CloneService<R> for S
where
    S: Service<R> + Clone + Send + Sync + 'static,
{
    fn clone_box(
        &self,
    ) -> Box<
        dyn CloneService<R, Response = Self::Response, Error = Self::Error, Future = Self::Future>
            + Send
            + Sync,
    > {
        Box::new(self.clone())
    }
}
