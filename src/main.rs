#[macro_use]
extern crate rocket;

mod db;
mod oidc;
mod rest;

use std::{fs, io};

use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;

pub struct App {
    seaorm_pool: sea_orm::DatabaseConnection,
}

const LOG_PATH: &str = "development.log";
fn write_to_log() -> impl io::Write {
    let pwd = std::env::current_dir().expect("couldn't get current dir");
    fs::File::options()
        .append(true)
        .write(true)
        .open(pwd.join(LOG_PATH))
        .expect("failed to create file")
}

fn initialize_tracing() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_writer(write_to_log)
        .with_span_events(FmtSpan::CLOSE)
        .with_target(true)
        .with_max_level(Level::DEBUG)
        .with_level(false)
        .with_line_number(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

#[launch]
async fn rocket() -> _ {
    initialize_tracing();

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
