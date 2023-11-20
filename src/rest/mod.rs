use rocket::{http::Status, serde::json::Json, State};

use entity::clients;
use sea_orm::{ActiveModelTrait, Set};
use serde::Deserialize;

use crate::App;

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
    app: &State<App>,
    payload: Json<CreateClientPayload>,
) -> Result<Json<clients::Model>, Status> {
    let client = clients::ActiveModel {
        name: Set(payload.name.clone()),
        description: Set(payload.description.clone()),
        redirect_uris: Set(payload.redirect_uris.join(",")),
        grant_types: Set(payload.grant_types.join(",")),
        response_types: Set(payload.response_types.join(",")),
        scope: Set(payload.scope.join(",")),
        ..Default::default()
    };

    let client = client.insert(&app.seaorm_pool).await;
    match client {
        Ok(client) => Ok(Json(client)),
        Err(_) => Err(Status::InternalServerError),
    }
}
