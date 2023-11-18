use rocket::{http::Status, serde::json::Json, State};
use serde::Deserialize;

use crate::{
    models::{Client, ClientBuilder},
    AppState,
};

#[derive(Deserialize)]
pub struct CreateClientPayload {
    name: String,
    description: Option<String>,
    redirect_uris: Vec<String>,
    grant_types: Vec<String>,
    response_types: Vec<String>,
    scope: Vec<String>,
}

#[post("/", data = "<payload>")]
pub async fn create_client(
    app: &State<AppState>,
    payload: Json<CreateClientPayload>,
) -> Result<Json<Client>, Status> {
    let builder = ClientBuilder::new()
        .name(payload.name.clone())
        .description(payload.description.clone())
        .redirect_uris(payload.redirect_uris.clone())
        .grant_types(payload.grant_types.clone())
        .response_types(payload.response_types.clone())
        .scope(payload.scope.clone());

    let client = builder.save(&app.db_pool).await;

    match client {
        Ok(client) => Ok(Json(client)),
        Err(_) => Err(Status::InternalServerError),
    }
}
