#[macro_use]
extern crate rocket;

mod db;
mod oidc;
mod rest;

use oidc::authorize::AuthorizePayload;
use rocket::{form::Form, http::Status, State};

use entity::clients::Entity as Client;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

pub struct App {
    seaorm_pool: sea_orm::DatabaseConnection,
}

async fn handle_authorize(
    app: &State<App>,
    payload: AuthorizePayload,
) -> Result<Option<String>, sea_orm::DbErr> {
    let q = Client::find().filter(entity::clients::Column::Uuid.eq(payload.client_id()));
    let Some(client) = q.one(&app.seaorm_pool).await? else {
        return Ok(None);
    };
    Ok(Some(format!(
        "Hello, world! {}, {}",
        payload.to_string(),
        client.uuid
    )))
}

#[get("/authorize?<payload..>")]
async fn authorize_get(app: &State<App>, payload: AuthorizePayload) -> Result<String, Status> {
    match handle_authorize(app, payload).await {
        Ok(Some(result)) => Ok(result),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/authorize", data = "<payload>")]
async fn authorize_post(
    app: &State<App>,
    payload: Form<AuthorizePayload>,
) -> Result<String, Status> {
    match handle_authorize(app, payload.into_inner()).await {
        Ok(Some(result)) => Ok(result),
        Ok(None) | Err(sea_orm::DbErr::RecordNotFound(_)) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![authorize_get, authorize_post])
        .mount("/clients", routes![rest::create_client])
        .manage(App {
            seaorm_pool: db::get_seaorm_pool().await.unwrap(),
        })
}
