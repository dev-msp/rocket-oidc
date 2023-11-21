#[macro_use]
extern crate rocket;

mod db;
mod oidc;
mod rest;

use std::{fs, io};


pub struct App {
    seaorm_pool: sea_orm::DatabaseConnection,
}



#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount(
            "/authorize",
            routes![
                oidc::authorize::authorize_get,
                oidc::authorize::authorize_post
            ],
        )
        .mount(
            "/clients",
            routes![
                rest::clients::get_clients,
                rest::clients::create_client,
                rest::clients::rotate_client_secret
            ],
        )
        .manage(App {
            seaorm_pool: db::get_seaorm_pool().await.unwrap(),
        })
}
