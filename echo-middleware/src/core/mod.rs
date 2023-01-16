mod add_extension;
mod catch_error;

pub use add_extension::{AddExtension, AddExtensionMiddleware};
pub use catch_error::{CatchError, CatchErrorMiddleware};

pub mod future {
    pub use super::catch_error::CatchErrorFuture;
}
