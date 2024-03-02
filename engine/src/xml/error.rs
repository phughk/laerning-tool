use std::fmt::{Debug, Display, Formatter};

// Renaming Error elements to match Rust Clippy suggestion: https://rust-lang.github.io/rust-clippy/master/index.html#/enum_variant_names
#[derive(Debug)]
pub enum Error {
    Io {},
    ListModule { error: String, path: String },
    Serde {},
    ServerError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io {} => write!(f, "IO error occurred"),
            Error::ListModule { error, path } => write!(
                f,
                "Error occurred while listing modules: {} (Path: {})",
                error, path
            ),
            Error::Serde {} => write!(f, "Serde serialization/deserialization error"),
            Error::ServerError(error) => write!(f, "Server unable to start: {error}"),
        }
    }
}

impl std::error::Error for Error {}
