use std::str::FromStr;

use base64::Engine;
use rocket::{http::Status, request, serde::json::Json, State};

use entity::{clients, uuid::Uuid};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryTrait, Set};
use serde_json::Value;

use crate::App;

use super::{generate_secret, ClientError};

pub enum SecretRotator {
    ClientSelf { uuid: Uuid, secret: String },
}

async fn get_client(
    app: &App,
    uuid: &Uuid,
    secret: Option<&str>,
) -> Result<Option<clients::Model>, ClientError> {
    let client = clients::Entity::find()
        .filter(clients::Column::Uuid.eq(uuid.clone()))
        .apply_if(secret, |q, sec| q.filter(clients::Column::Secret.eq(sec)))
        .one(&app.seaorm_pool)
        .await?;

    Ok(client)
}

impl SecretRotator {
    pub async fn rotate(&self, app: &App, uuid: &Uuid) -> Result<String, ClientError> {
        if !(self.may_access(app, uuid).await?) {
            return Err(ClientError::not_found());
        }

        let Some(client) = get_client(app, uuid, None).await? else {
            return Err(ClientError::NotAuthorized);
        };

        let mut client: clients::ActiveModel = client.into();
        let new_secret = generate_secret(64);
        client.secret = Set(new_secret.clone());
        client.update(&app.seaorm_pool).await?;

        Ok(new_secret)
    }

    async fn may_access(&self, app: &App, uuid: &Uuid) -> Result<bool, ClientError> {
        match self {
            SecretRotator::ClientSelf {
                uuid: rotator_uuid,
                secret,
            } => {
                let credentials_valid =
                    get_client(app, rotator_uuid, Some(secret)).await?.is_some();
                Ok(credentials_valid && rotator_uuid == uuid)
            }
        }
    }
}

fn parse_basic_auth(header: &str) -> Option<(String, String)> {
    let (auth_type, auth) = header.split_once(' ')?;
    if auth_type != "Basic" {
        return None;
    }
    let auth = base64::engine::general_purpose::STANDARD
        .decode(auth.as_bytes())
        .ok()?;
    let auth = String::from_utf8(auth).ok()?;
    let mut parts = auth.splitn(2, ':');
    let username = parts.next()?.to_string();
    let password = parts.next()?.to_string();
    Some((username, password))
}

#[rocket::async_trait]
impl<'r> request::FromRequest<'r> for SecretRotator {
    type Error = ();

    async fn from_request(
        request: &'r request::Request<'_>,
    ) -> request::Outcome<Self, Self::Error> {
        let auth = request
            .headers()
            .get_one("Authorization")
            .unwrap_or_default();

        let parsed_auth = parse_basic_auth(auth).and_then(|(username, password)| {
            Uuid::from_str(&username).ok().map(|uuid| (uuid, password))
        });

        let Some((client_uuid, client_secret)) = parsed_auth else {
            return request::Outcome::Error((Status::BadRequest, ()));
        };

        request::Outcome::Success(SecretRotator::ClientSelf {
            uuid: client_uuid,
            secret: client_secret,
        })
    }
}

#[patch("/<uuid>/rotate_secret")]
pub async fn rotate_client_secret(
    app: &State<App>,
    rotator: SecretRotator,
    uuid: Uuid,
) -> Result<Json<Value>, (Status, String)> {
    let new_secret = rotator.rotate(app, &uuid).await?;
    Ok(Json(serde_json::json!({ "secret": new_secret })))
}
