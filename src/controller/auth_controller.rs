use rocket::request::Form;
use rocket_contrib::json::Json;

#[allow(unused_imports)]
use serde::Serialize;


#[derive(FromForm, Serialize)]
pub struct RegisterPost {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

#[post("/auth/register", data = "<register_post>")]
pub fn register(register_post: Form<RegisterPost>) -> Json<RegisterPost> {
    Json(RegisterPost {
        first_name: register_post.first_name.to_string(),
        last_name: register_post.last_name.to_string(),
        email: register_post.email.to_string(),
        password: register_post.password.to_string(),
    })
}
