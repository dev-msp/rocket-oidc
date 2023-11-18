#[macro_use]
extern crate rocket;

mod db;
mod models;
mod oidc;
mod rest;

use oidc::authorize::AuthorizePayload;
use rocket::{form::Form, http::Status, State};

use crate::models::{Client, Model};

pub struct AppState {
    db_pool: sqlx::Pool<sqlx::Sqlite>,
}

async fn handle_authorize(
    app: &State<AppState>,
    payload: AuthorizePayload,
) -> Result<Option<String>, sqlx::Error> {
    let Some(client) = Client::find_by_uuid(&app.db_pool, payload.client_id()).await? else {
        return Ok(None);
    };
    Ok(Some(format!(
        "Hello, world! {}, {}",
        payload.to_string(),
        client.uuid()
    )))
}

#[get("/authorize?<payload..>")]
async fn authorize_get(app: &State<AppState>, payload: AuthorizePayload) -> Result<String, Status> {
    match handle_authorize(app, payload).await {
        Ok(Some(result)) => Ok(result),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/authorize", data = "<payload>")]
async fn authorize_post(
    app: &State<AppState>,
    payload: Form<AuthorizePayload>,
) -> Result<String, Status> {
    match handle_authorize(app, payload.into_inner()).await {
        Ok(Some(result)) => Ok(result),
        Ok(None) | Err(sqlx::Error::RowNotFound) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![authorize_get, authorize_post])
        .mount("/clients", routes![rest::create_client])
        .manage(AppState {
            db_pool: db::get_pool().await.unwrap(),
        })
}
