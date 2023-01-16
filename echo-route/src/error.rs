use std::fmt;

use echo_core::Request;
use sync_wrapper::SyncWrapper;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RouteErrorKind {
    NotFound,
    MethodNotAllowed,
}

#[derive(Debug)]
pub struct RouteError {
    kind: RouteErrorKind,
    request: SyncWrapper<Request>,
}

impl RouteError {
    pub fn new(kind: RouteErrorKind, request: Request) -> Self {
        Self {
            kind,
            request: SyncWrapper::new(request),
        }
    }

    pub fn not_found(request: Request) -> Self {
        Self::new(RouteErrorKind::NotFound, request)
    }

    pub fn method_not_allowed(request: Request) -> Self {
        Self::new(RouteErrorKind::MethodNotAllowed, request)
    }

    pub fn kind(&self) -> RouteErrorKind {
        self.kind
    }

    pub fn request_mut(&mut self) -> &mut Request {
        self.request.get_mut()
    }

    pub fn into_request(self) -> Request {
        self.request.into_inner()
    }
}

impl fmt::Display for RouteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind() {
            RouteErrorKind::NotFound { .. } => f.write_str("Not Found"),
            RouteErrorKind::MethodNotAllowed { .. } => f.write_str("Method Not Allowed"),
        }
    }
}

impl std::error::Error for RouteError {}

#[derive(Debug)]
pub enum RouterError {
    Conflict { path: String, message: String },
    InvalidPath { path: String, message: String },
    TooManyPath,
}

impl RouterError {
    pub(crate) fn from_insert_error(path: String, error: matchit::InsertError) -> Self {
        match error {
            matchit::InsertError::Conflict { .. } => RouterError::Conflict {
                path,
                message: format!("{error}"),
            },
            _ => RouterError::InvalidPath {
                path,
                message: format!("{error}"),
            },
        }
    }
}

impl fmt::Display for RouterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RouterError::Conflict { path, message } => {
                write!(f, "conflict path {path} ({message})")
            }
            RouterError::InvalidPath { path, message } => {
                write!(f, "invalid path {path} ({message})")
            }
            RouterError::TooManyPath => f.write_str("too many path"),
        }
    }
}

impl std::error::Error for RouterError {}
