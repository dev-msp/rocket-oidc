use rocket::form::Form;

#[macro_use]
extern crate rocket;

#[derive(Debug, FromFormField)]
enum ResponseType {
    Code,
}

#[derive(FromForm)]
struct AuthorizePayload {
    response_type: ResponseType,
    client_id: String,
    redirect_uri: String,
    scope: String,
    state: String,
    nonce: Option<String>,
}

impl ToString for AuthorizePayload {
    fn to_string(&self) -> String {
        format!(
            "response_type={:?}, client_id={}, redirect_uri={}, scope={}, state={}, nonce={:?}",
            self.response_type,
            self.client_id,
            self.redirect_uri,
            self.scope,
            self.state,
            self.nonce
        )
    }
}

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
