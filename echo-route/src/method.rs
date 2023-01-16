use std::collections::{HashMap, HashSet};

use echo_core::http::Method;
use echo_core::service::future::BoxFuture;
use echo_core::service::{ArcService, Middleware, Service};
use echo_core::{BoxError, Request, Response};

use crate::future::RouteFuture;
use crate::RouteError;

#[derive(Debug, Default)]
pub struct MethodRouter {
    map: HashMap<Method, ArcService<Request, Response, BoxError>>,
    any: Option<ArcService<Request, Response, BoxError>>,
}

impl MethodRouter {
    pub fn add(&mut self, service: ArcService<Request, Response, BoxError>, method: Method) {
        self.map.insert(method, service);
    }

    pub fn add_any(&mut self, service: ArcService<Request, Response, BoxError>) {
        self.any = Some(service);
    }

    pub fn contains(&self, method: &Method) -> bool {
        self.map.contains_key(method)
    }

    pub fn contains_any(&self) -> bool {
        self.any.is_some()
    }
}

impl Service<Request> for MethodRouter {
    type Response = Response;
    type Error = BoxError;
    type Future = RouteFuture<BoxFuture<'static, Result<Response, BoxError>>>;

    fn call(&self, request: Request) -> Self::Future {
        pub fn match_<'a>(
            router: &'a MethodRouter,
            method: &Method,
        ) -> Option<&'a ArcService<Request, Response, BoxError>> {
            router
                .map
                .get(method)
                .or_else(|| {
                    if method == Method::HEAD {
                        router.map.get(&Method::GET)
                    } else {
                        None
                    }
                })
                .or_else(|| router.any.as_ref())
        }

        match match_(self, request.method()) {
            Some(service) => RouteFuture::Future {
                fut: service.call(request),
            },
            None => RouteFuture::Error {
                err: Some(RouteError::method_not_allowed(request).into()),
            },
        }
    }
}

#[derive(Debug, Clone)]
enum Methods {
    Any,
    One(Method),
    More(HashSet<Method>),
}

impl Methods {
    pub fn add(mut self, method: Method) -> Self {
        match self {
            Methods::Any => Methods::One(method),
            Methods::One(m) => Self::More(HashSet::from([m, method])),
            Methods::More(ref mut s) => {
                s.insert(method);
                self
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct MethodRoute<S> {
    methods: Methods,
    service: S,
}

impl<S> MethodRoute<S> {
    pub fn any(service: S) -> Self {
        Self {
            methods: Methods::Any,
            service,
        }
    }

    pub fn one(service: S, method: Method) -> Self {
        Self {
            methods: Methods::One(method),
            service,
        }
    }

    pub fn more(service: S, methods: HashSet<Method>) -> Self {
        Self {
            methods: Methods::More(methods),
            service,
        }
    }

    pub fn add(mut self, method: Method) -> Self {
        self.methods = self.methods.add(method);
        self
    }

    pub fn with<T>(self, middleware: T) -> MethodRoute<T::Service>
    where
        T: Middleware<S>,
    {
        MethodRoute {
            methods: self.methods,
            service: middleware.transform(self.service),
        }
    }
}

pub fn any<S>(service: S) -> MethodRoute<S> {
    MethodRoute::any(service)
}

pub fn method<S>(service: S, method: Method) -> MethodRoute<S> {
    MethodRoute::one(service, method)
}

macro_rules! impl_method_fn {
    ($name:ident, $method:expr) => {
        pub fn $name<S>(service: S) -> MethodRoute<S> {
            method(service, $method)
        }
    };
}

impl_method_fn!(connect, Method::CONNECT);
impl_method_fn!(delete, Method::DELETE);
impl_method_fn!(get, Method::GET);
impl_method_fn!(head, Method::HEAD);
impl_method_fn!(options, Method::OPTIONS);
impl_method_fn!(patch, Method::PATCH);
impl_method_fn!(post, Method::POST);
impl_method_fn!(put, Method::PUT);
impl_method_fn!(trace, Method::TRACE);

pub trait IntoMethodRoute {
    type Service;

    fn into_method_route(self) -> MethodRoute<Self::Service>;
}

impl<S> IntoMethodRoute for S
where
    S: Service<Request>,
{
    type Service = S;

    fn into_method_route(self) -> MethodRoute<Self::Service> {
        MethodRoute::any(self)
    }
}

impl<S> IntoMethodRoute for MethodRoute<S> {
    type Service = S;

    fn into_method_route(self) -> MethodRoute<S> {
        self
    }
}

pub trait MergeToMethodRouter {
    fn merge_to(self, router: &mut MethodRouter) -> Result<(), Option<Method>>;
}

impl MergeToMethodRouter for MethodRouter {
    fn merge_to(self, router: &mut MethodRouter) -> Result<(), Option<Method>> {
        for method in self.map.keys() {
            if router.contains(method) {
                return Err(Some(method.clone()));
            }
        }
        if let Some(service) = self.any {
            if router.contains_any() {
                return Err(None);
            }
            router.add_any(service);
        }
        for (method, service) in self.map {
            router.add(service, method);
        }
        Ok(())
    }
}

impl MergeToMethodRouter for MethodRoute<ArcService<Request, Response, BoxError>> {
    fn merge_to(self, router: &mut MethodRouter) -> Result<(), Option<Method>> {
        match self.methods {
            Methods::Any => {
                if router.contains_any() {
                    return Err(None);
                }
                router.add_any(self.service);
            }
            Methods::One(method) => {
                if router.contains(&method) {
                    return Err(Some(method));
                }
                router.add(self.service, method);
            }
            Methods::More(methods) => {
                for method in methods.iter() {
                    if router.contains(&method) {
                        return Err(Some(method.clone()));
                    }
                }
                for method in methods {
                    router.add(self.service.clone(), method);
                }
            }
        }
        Ok(())
    }
}
