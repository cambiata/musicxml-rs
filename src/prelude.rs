pub use crate::error::MusicXmlError::{Generic, TextfieldEmpty, UnknownAttribute, UnknownElement};
pub type Result<T> = anyhow::Result<T>;
