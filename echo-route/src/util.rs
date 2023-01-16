use echo_core::http::uri::{Parts, Uri};
use echo_core::Request;

pub fn try_downcast<T: 'static, K: 'static>(k: K) -> Result<T, K> {
    let mut k = Some(k);
    if let Some(k) = <dyn std::any::Any>::downcast_mut::<Option<T>>(&mut k) {
        Ok(k.take().unwrap())
    } else {
        Err(k.unwrap())
    }
}

pub fn replace_request_path(request: &mut Request, path: &str) {
    let uri = request.uri_mut();

    let path = if path.starts_with('/') {
        path[1..].as_ref()
    } else {
        path
    };

    let path_and_query = if let Some(query) = uri.query() {
        format!("/{path}?{query}")
    } else {
        format!("/{path}")
    };

    let mut parts = Parts::default();

    parts.scheme = uri.scheme().cloned();
    parts.authority = uri.authority().cloned();
    parts.path_and_query = Some(path_and_query.parse().unwrap());

    *uri = Uri::from_parts(parts).unwrap();
}
