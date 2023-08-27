use crate::api::ApiError::InvalidRequest;

#[derive(Debug)]
pub(crate) enum ApiError {
    InvalidRequest,
}

#[derive(Debug)]
pub(crate) struct Dataset {
    id: String,
    name: String,
    description: String,
}

pub(crate) async fn get_datasets() -> Result<Vec<Dataset>, ApiError> {
    Err(InvalidRequest)
}
