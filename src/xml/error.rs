use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    IoError {},
    ListModuleError { error: String, path: String },
    SerdeError {},
    Unknown { cause: String },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError {} => write!(f, "IO error occurred"),
            Error::ListModuleError { error, path } => write!(
                f,
                "Error occurred while listing modules: {} (Path: {})",
                error, path
            ),
            Error::SerdeError {} => write!(f, "Serde serialization/deserialization error"),
            Error::Unknown { cause } => write!(f, "Unknown error occurred: {}", cause),
        }
    }
}

impl std::error::Error for Error {}
