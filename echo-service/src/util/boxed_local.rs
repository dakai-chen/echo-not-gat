use core::{fmt, future::Future, pin::Pin};

use alloc::{boxed::Box, rc::Rc};

use crate::{Service, ServiceExt};

pub type LocalBoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

pub struct LocalBoxService<Req, Res, Err> {
    inner: Box<
        dyn Service<
            Req,
            Response = Res,
            Error = Err,
            Future = LocalBoxFuture<'static, Result<Res, Err>>,
        >,
    >,
}

impl<Req, Res, Err> LocalBoxService<Req, Res, Err> {
    pub fn new<S>(inner: S) -> Self
    where
        S: Service<Req, Response = Res, Error = Err> + 'static,
        S::Future: 'static,
    {
        Self {
            inner: Box::new(inner.map_future(|f| Box::pin(f) as _)),
        }
    }
}

impl<Req, Res, Err> Service<Req> for LocalBoxService<Req, Res, Err> {
    type Response = Res;
    type Error = Err;
    type Future = LocalBoxFuture<'static, Result<Res, Err>>;

    fn call(&self, request: Req) -> LocalBoxFuture<'static, Result<Res, Err>> {
        self.inner.call(request)
    }
}

impl<Req, Res, Err> fmt::Debug for LocalBoxService<Req, Res, Err> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("LocalBoxService").finish()
    }
}

pub struct LocalBoxCloneService<Req, Res, Err> {
    inner: Box<
        dyn LocalCloneService<
            Req,
            Response = Res,
            Error = Err,
            Future = LocalBoxFuture<'static, Result<Res, Err>>,
        >,
    >,
}

impl<Req, Res, Err> Clone for LocalBoxCloneService<Req, Res, Err> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone_box(),
        }
    }
}

impl<Req, Res, Err> LocalBoxCloneService<Req, Res, Err> {
    pub fn new<S>(inner: S) -> Self
    where
        S: Service<Req, Response = Res, Error = Err> + Clone + 'static,
        S::Future: 'static,
    {
        LocalBoxCloneService {
            inner: Box::new(inner.map_future(|f| Box::pin(f) as _)),
        }
    }
}

impl<Req, Res, Err> Service<Req> for LocalBoxCloneService<Req, Res, Err> {
    type Response = Res;
    type Error = Err;
    type Future = LocalBoxFuture<'static, Result<Res, Err>>;

    fn call(&self, request: Req) -> LocalBoxFuture<'static, Result<Res, Err>> {
        self.inner.call(request)
    }
}

impl<Req, Res, Err> fmt::Debug for LocalBoxCloneService<Req, Res, Err> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("LocalBoxCloneService").finish()
    }
}

pub struct RcService<Req, Res, Err> {
    inner: Rc<
        dyn Service<
            Req,
            Response = Res,
            Error = Err,
            Future = LocalBoxFuture<'static, Result<Res, Err>>,
        >,
    >,
}

impl<Req, Res, Err> Clone for RcService<Req, Res, Err> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<Req, Res, Err> RcService<Req, Res, Err> {
    pub fn new<S>(inner: S) -> Self
    where
        S: Service<Req, Response = Res, Error = Err> + 'static,
        S::Future: 'static,
    {
        Self {
            inner: Rc::new(inner.map_future(|f| Box::pin(f) as _)),
        }
    }
}

impl<Req, Res, Err> Service<Req> for RcService<Req, Res, Err> {
    type Response = Res;
    type Error = Err;
    type Future = LocalBoxFuture<'static, Result<Res, Err>>;

    fn call(&self, request: Req) -> LocalBoxFuture<'static, Result<Res, Err>> {
        self.inner.call(request)
    }
}

impl<Req, Res, Err> fmt::Debug for RcService<Req, Res, Err> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("RcService").finish()
    }
}

trait LocalCloneService<R>: Service<R> {
    fn clone_box(
        &self,
    ) -> Box<
        dyn LocalCloneService<
            R,
            Response = Self::Response,
            Error = Self::Error,
            Future = Self::Future,
        >,
    >;
}

impl<S, R> LocalCloneService<R> for S
where
    S: Service<R> + Clone + 'static,
{
    fn clone_box(
        &self,
    ) -> Box<
        dyn LocalCloneService<
            R,
            Response = Self::Response,
            Error = Self::Error,
            Future = Self::Future,
        >,
    > {
        Box::new(self.clone())
    }
}
