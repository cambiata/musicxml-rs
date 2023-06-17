pub use crate::error::Error::{Generic, UnknownAttribute, UnknownElement, IO};

pub type Result<T> = core::result::Result<T, crate::error::Error>;
