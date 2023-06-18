pub use crate::error::MusicXmlError::{Generic, TextfieldEmpty, UnknownAttribute, UnknownElement};
pub type Result<T> = anyhow::Result<T>;

pub type Node<'a> = roxmltree::Node<'a, 'a>;
pub type NodeType = roxmltree::NodeType;
pub type Document<'a> = roxmltree::Document<'a>;
