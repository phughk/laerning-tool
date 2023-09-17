use crate::BASE_API;
use log::{error, trace};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub(crate) enum ApiError {
    InvalidRequest,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Dataset {
    id: String,
    name: String,
    description: Option<String>,
}

pub(crate) async fn get_datasets() -> Result<Vec<Dataset>, ApiError> {
    let req_url = format!("{}/dataset/list", BASE_API);
    match reqwest::get(req_url).await {
        Ok(response) => {
            if response.status().is_success() {
                let datasets: Vec<Dataset> = response.json().await.unwrap();
                Ok(datasets)
            } else {
                trace!("Request was not successful: {}", response.status());
                Err(ApiError::InvalidRequest)
            }
        }
        Err(e) => {
            error!("An actual error occured: {}", e);
            Err(ApiError::InvalidRequest)
        }
    }
}
