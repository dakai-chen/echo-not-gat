#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]

mod body;
mod boxed;
mod collect;
mod ext;
mod limited;
mod map_err;
mod next;
mod stream;

pub use body::Body;
pub use boxed::BoxBody;
pub use bytes::Bytes;
pub use collect::Collect;
pub use ext::BodyExt;
pub use http_body::{Frame, SizeHint};
pub use limited::{LengthLimitError, Limited};
pub use map_err::MapErr;
pub use next::{Data, Next};
pub use stream::{BodyStream, StreamBody};

type BoxError = Box<dyn std::error::Error + Send + Sync>;

fn try_downcast<T: 'static, K: 'static>(k: K) -> Result<T, K> {
    let mut k = Some(k);
    if let Some(k) = <dyn std::any::Any>::downcast_mut::<Option<T>>(&mut k) {
        Ok(k.take().unwrap())
    } else {
        Err(k.unwrap())
    }
}
