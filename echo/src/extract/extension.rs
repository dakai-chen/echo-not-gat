use echo_core::Request;

pub fn extension<T>(request: &Request) -> Option<&T>
where
    T: Send + Sync + 'static,
{
    request.extensions().get::<T>()
}

pub fn extension_mut<T>(request: &mut Request) -> Option<&mut T>
where
    T: Send + Sync + 'static,
{
    request.extensions_mut().get_mut::<T>()
}
