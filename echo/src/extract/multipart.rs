use echo_core::Request;
pub use echo_multipart::{Field, Multipart, MultipartError};

pub fn multipart(request: &mut Request) -> Result<Multipart, MultipartError> {
    Multipart::from_request(request)
}
