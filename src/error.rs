use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MusicXmlError {
    Generic(String),
    UnknownElement(String),
    UnknownAttribute(String),
    TextfieldEmpty(String),
}

impl std::fmt::Display for MusicXmlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MusicXmlError::Generic(s) => write!(f, "MusicXmlError::Generic error: {}", s),
            MusicXmlError::UnknownElement(s) => write!(f, "MusicXmlError::UnknownElement: {}", s),
            MusicXmlError::UnknownAttribute(s) => {
                write!(f, "MusicXmlError::UnknownAttribute: {}", s)
            }
            MusicXmlError::TextfieldEmpty(s) => write!(f, "MusicXmlError::TextfieldEmpty: {}", s),
        }
    }
}
