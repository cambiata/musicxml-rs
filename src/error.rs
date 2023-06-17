#[derive(thiserror::Error, Debug)]

pub enum Error {
    #[error("Generic: {0}")]
    Generic(String),

    #[error("UnknownElement: {0}")]
    UnknownElement(String),
    #[error("UnknownAttribute: {0}")]
    UnknownAttribute(String),

    #[error(transparent)]
    Roxml(#[from] roxmltree::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}
