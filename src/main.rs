#[macro_use]
extern crate rocket;

mod db;
mod oidc;

use oidc::authorize::AuthorizePayload;
use rocket::form::Form;

fn handle_authorize(payload: AuthorizePayload) -> String {
    format!("Hello, world! {}", payload.to_string())
}

#[get("/authorize?<payload..>")]
fn authorize_get(payload: AuthorizePayload) -> String {
    handle_authorize(payload)
}

#[post("/authorize", data = "<payload>")]
fn authorize_post(payload: Form<AuthorizePayload>) -> String {
    handle_authorize(payload.into_inner())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![authorize_get, authorize_post])
}
