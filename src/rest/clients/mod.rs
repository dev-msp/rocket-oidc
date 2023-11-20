use rocket::{http::Status, serde::json::Json, State};

use entity::clients::{self, GrantTypes, RedirectUris, ResponseTypes, Scope, Uuid};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::Deserialize;

use crate::App;

mod rotate_secret;

pub use rotate_secret::rotate_client_secret;

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Database error: {0}")]
    DbErr(#[from] sea_orm::DbErr),

    #[error("Not authorized")]
    NotAuthorized,
}

impl ClientError {
    pub fn not_found() -> Self {
        ClientError::DbErr(sea_orm::DbErr::RecordNotFound(
            "Client not found".to_string(),
        ))
    }
}

impl From<ClientError> for (Status, String) {
    fn from(err: ClientError) -> Self {
        match err {
            ClientError::DbErr(e) => (Status::InternalServerError, e.to_string()),
            ClientError::NotAuthorized => (Status::Unauthorized, "Unauthorized".to_string()),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateClientPayload {
    name: String,
    description: Option<String>,
    redirect_uris: RedirectUris,
    grant_types: GrantTypes,
    response_types: ResponseTypes,
    scope: Scope,
}

pub fn generate_secret(size: usize) -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}

#[get("/")]
pub async fn get_clients(app: &State<App>) -> Result<Json<Vec<clients::Model>>, (Status, String)> {
    let clients = clients::Entity::find()
        .all(&app.seaorm_pool)
        .await
        .map_err(ClientError::from)?;
    Ok(Json(clients))
}

#[post("/", data = "<payload>")]
pub async fn create_client(
    app: &State<App>,
    payload: Json<CreateClientPayload>,
) -> Result<Json<clients::Model>, (Status, String)> {
    let client = clients::ActiveModel {
        name: Set(payload.name.clone()),
        uuid: Set(Uuid::default()),
        secret: Set(generate_secret(64)),
        description: Set(payload.description.clone()),
        redirect_uris: Set(payload.redirect_uris.clone()),
        grant_types: Set(payload.grant_types.clone()),
        response_types: Set(payload.response_types.clone()),
        scope: Set(payload.scope.clone()),
        ..Default::default()
    };

    let client = client.insert(&app.seaorm_pool).await;
    match client {
        Ok(client) => Ok(Json(client)),
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}
