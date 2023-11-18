#![allow(dead_code)]

use base64::Engine;
use rand::Rng;
use serde::Serialize;

#[async_trait]
pub trait Model: Sized + Send + Sync {
    async fn find_all(conn: impl sqlx::Executor<'_, Database = sqlx::Sqlite>) -> Vec<Self>;
    async fn find_by_uuid(
        conn: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
        uuid: &str,
    ) -> Result<Option<Self>, sqlx::Error>;

    fn uuid(&self) -> String;
    async fn save(&self, conn: impl sqlx::Executor<'_, Database = sqlx::Sqlite>) -> bool;
}

fn datetime_to_epoch<S>(
    datetime: &chrono::DateTime<chrono::Utc>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_i64(datetime.timestamp())
}

#[derive(Debug, Serialize)]
pub struct Client {
    id: i64,
    #[serde(serialize_with = "datetime_to_epoch")]
    created_at: chrono::DateTime<chrono::Utc>,

    #[serde(serialize_with = "datetime_to_epoch")]
    updated_at: chrono::DateTime<chrono::Utc>,
    uuid: String,
    name: String,
    description: Option<String>,
    secret: String,
    redirect_uris: Vec<String>,
    grant_types: Vec<String>,
    response_types: Vec<String>,
    scope: Vec<String>,
}

#[async_trait]
impl Model for Client {
    async fn find_all(conn: impl sqlx::Executor<'_, Database = sqlx::Sqlite>) -> Vec<Self> {
        let results = sqlx::query!(
			"SELECT id, created_at, updated_at, uuid, name, description, secret, redirect_uris, grant_types, response_types, scope FROM clients"
		)
		.fetch_all(conn)
		.await
		.unwrap();

        results
            .into_iter()
            .map(|result| Client {
                id: result.id,
                created_at: result.created_at.and_utc(),
                updated_at: result.updated_at.and_utc(),
                uuid: result.uuid,
                name: result.name,
                description: result.description,
                secret: result.secret,
                redirect_uris: result.redirect_uris.split(',').map(String::from).collect(),
                grant_types: result.grant_types.split(' ').map(String::from).collect(),
                response_types: result.response_types.split(' ').map(String::from).collect(),
                scope: result.scope.split(' ').map(String::from).collect(),
            })
            .collect()
    }

    async fn find_by_uuid(
        conn: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
        uuid: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        let Some(result) = sqlx::query!("SELECT * FROM clients WHERE uuid = ?", uuid)
            .fetch_optional(conn)
            .await?
        else {
            return Ok(None);
        };

        Ok(Some(Client {
            id: result.id,
            created_at: result.created_at.and_utc(),
            updated_at: result.updated_at.and_utc(),
            uuid: result.uuid,
            name: result.name,
            description: result.description,
            secret: result.secret,
            redirect_uris: result.redirect_uris.split(',').map(String::from).collect(),
            grant_types: result.grant_types.split(' ').map(String::from).collect(),
            response_types: result.response_types.split(' ').map(String::from).collect(),
            scope: result.scope.split(' ').map(String::from).collect(),
        }))
    }

    fn uuid(&self) -> String {
        self.uuid.clone()
    }

    async fn save(&self, conn: impl sqlx::Executor<'_, Database = sqlx::Sqlite>) -> bool {
        let redirect_uris = self.redirect_uris.join(",").to_string();
        let grant_types = self.grant_types.join(" ").to_string();
        let response_types = self.response_types.join(" ").to_string();
        let scope = self.scope.join(" ").to_string();
        let result = sqlx::query!(
			// upsert
			"INSERT INTO clients (id, uuid, name, description, redirect_uris, grant_types, response_types, scope)
			VALUES (NULL, ?, ?, ?, ?, ?, ?, ?)
			ON CONFLICT(uuid) DO UPDATE SET
				name = ?,
				description = ?,
				redirect_uris = ?,
				grant_types = ?,
				response_types = ?,
				scope = ?
			",
			self.uuid,
			self.name,
			self.description,
			redirect_uris,
			grant_types,
			response_types,
			scope,
			self.name,
			self.description,
			redirect_uris,
			grant_types,
			response_types,
			scope,
		)
		.execute(conn)
		.await;
        result.is_ok()
    }
}

#[derive(Default)]
pub struct ClientBuilder {
    name: Option<String>,
    description: Option<String>,
    redirect_uris: Vec<String>,
    grant_types: Vec<String>,
    response_types: Vec<String>,
    scope: Vec<String>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn description(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }

    pub fn redirect_uris(mut self, redirect_uris: Vec<String>) -> Self {
        self.redirect_uris = redirect_uris;
        self
    }

    pub fn grant_types(mut self, grant_types: Vec<String>) -> Self {
        self.grant_types = grant_types;
        self
    }

    pub fn response_types(mut self, response_types: Vec<String>) -> Self {
        self.response_types = response_types;
        self
    }

    pub fn scope(mut self, scope: Vec<String>) -> Self {
        self.scope = scope;
        self
    }

    pub async fn save(
        self,
        conn: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    ) -> Result<Client, sqlx::Error> {
        let uuid = uuid::Uuid::new_v4().to_string();
        let name = self.name.unwrap_or_else(|| "New Client".to_string());
        let description = self.description;
        let secret = generate_secret();
        let redirect_uris = self.redirect_uris.join(",");
        let grant_types = self.grant_types.join(" ");
        let response_types = self.response_types.join(" ");
        let scope = self.scope.join(" ");

        let result = sqlx::query!(
			"INSERT INTO clients (id, uuid, name, description, secret, redirect_uris, grant_types, response_types, scope)
 			VALUES (NULL, ?, ?, ?, ?, ?, ?, ?, ?)
			RETURNING id, created_at, updated_at
			",
			uuid,
			name,
			description,
			secret,
			redirect_uris,
			grant_types,
			response_types,
			scope,
		).fetch_one(conn).await?;

        Ok(Client {
            id: result.id,
            created_at: result.created_at.and_utc(),
            updated_at: result.updated_at.and_utc(),
            uuid,
            name,
            description,
            secret,
            redirect_uris: self.redirect_uris,
            grant_types: self.grant_types,
            response_types: self.response_types,
            scope: self.scope,
        })
    }
}

impl Client {
    pub fn uuid(&self) -> &str {
        &self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn redirect_uris(&self) -> &[String] {
        &self.redirect_uris
    }

    pub fn grant_types(&self) -> &[String] {
        &self.grant_types
    }

    pub fn response_types(&self) -> &[String] {
        &self.response_types
    }

    pub fn scope(&self) -> &[String] {
        &self.scope
    }
}

fn generate_secret() -> String {
    let mut secret = [0u8; 64];
    rand::thread_rng().fill(&mut secret);
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(secret)
}
