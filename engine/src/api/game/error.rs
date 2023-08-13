/// This is where the return type enums are declared
///
/// All endpoints must have an associated wrapping struct, since an enum cannot be converted
/// to JSON easily.
///
/// The struct therefore implements From<enum> and IntoResponse so that the correct HTTP response
/// is sent.
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

/// NewGameError for POST /game endpoint
#[derive(Debug, Serialize, ToSchema)]
pub enum NewGameError {
    UnspecifiedDataset,
    DatasetNotFound,
    UnrecognisedDataset { dataset: String },
    InternalError { cause: String },
}

#[derive(Debug, Serialize, ToSchema)]
pub struct NewGameErrorResponse {
    pub code: u16,
    pub cause: NewGameError,
}

impl From<NewGameError> for NewGameErrorResponse {
    fn from(value: NewGameError) -> Self {
        match value {
            NewGameError::UnspecifiedDataset => NewGameErrorResponse {
                code: StatusCode::BAD_REQUEST.as_u16(),
                cause: value,
            },
            NewGameError::DatasetNotFound => NewGameErrorResponse {
                code: StatusCode::BAD_REQUEST.as_u16(),
                cause: value,
            },
            NewGameError::InternalError { .. } => NewGameErrorResponse {
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                cause: value,
            },
            NewGameError::UnrecognisedDataset { dataset } => NewGameErrorResponse {
                code: StatusCode::BAD_REQUEST.as_u16(),
                cause: NewGameError::UnrecognisedDataset { dataset },
            },
        }
    }
}

impl IntoResponse for NewGameErrorResponse {
    fn into_response(self) -> Response {
        (StatusCode::from_u16(self.code).unwrap(), Json(self)).into_response()
    }
}

/// GameListingError for GET /game/list endpoint
#[derive(Debug, Serialize, ToSchema)]
pub enum GameListingError {
    InternalError { cause: String },
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GameListingErrorResponse {
    pub code: u16,
    pub cause: GameListingError,
}

impl From<GameListingError> for GameListingErrorResponse {
    fn from(value: GameListingError) -> Self {
        match value {
            GameListingError::InternalError { .. } => GameListingErrorResponse {
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                cause: value,
            },
        }
    }
}

impl IntoResponse for GameListingErrorResponse {
    fn into_response(self) -> Response {
        (StatusCode::from_u16(self.code).unwrap(), Json(self)).into_response()
    }
}
