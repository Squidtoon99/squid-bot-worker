#[macro_use]
extern crate rocket;

pub mod context;
pub mod discord;
pub mod error;
pub use error::Error;

#[get("/")]
fn ping() -> &'static str {
    "I'm alive!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![ping])
        .attach(discord::stage())
}
