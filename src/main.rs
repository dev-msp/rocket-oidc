#[macro_use]
extern crate rocket;

#[get("/")]
fn root() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![root])
}
