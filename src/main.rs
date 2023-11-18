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
}

#[get("/authorize?<payload..>")]
fn authorize(payload: AuthorizePayload) -> String {
    format!(
        "Hello, {}! response_type={:?}, redirect_uri={}, scope={}, state={}",
        payload.client_id,
        payload.response_type,
        payload.redirect_uri,
        payload.scope,
        payload.state
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![authorize])
}
