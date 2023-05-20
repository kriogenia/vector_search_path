

use axum::{extract::{Path, State}, response::IntoResponse, Json};

use crate::model;

use super::error::ApiError;

pub const ENDPOINT: &str = "/text/:text_to_embed";

pub(crate) async fn text(Path(text_to_embed): Path<String>, State(model): State<model::Model>) -> Result<impl IntoResponse, ApiError> {
    let encoded = model.encode(text_to_embed).await?;
    Ok(Json(encoded))
}
