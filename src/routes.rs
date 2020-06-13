use super::controller;
use rocket::Rocket;

pub fn build() -> Rocket {
    rocket::ignite()
        .mount("/", routes![controller::auth_controller::register])
}
