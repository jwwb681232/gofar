use rocket::request::Form;
use rocket_contrib::json::Json;

#[allow(unused_imports)]
use serde::Serialize;
use jsonwebtoken::{encode, Header, EncodingKey};

use crate::auth;
use auth::jwt::Claims;

#[derive(FromForm, Serialize)]
pub struct RegisterPost {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

#[post("/auth/register", data = "<register_post>")]
pub fn register(register_post: Form<RegisterPost>) -> Json<RegisterPost> {
    let header = Header::default();
    let claims = Claims::new("1".to_string());
    let secret = EncodingKey::from_secret(dotenv!("JWT_SECRET").as_ref());

    let token = encode(&header, &claims, &secret).expect("encode token error");

    println!("{}", token);

    let response_data = RegisterPost {
        first_name: register_post.first_name.to_string(),
        last_name: register_post.last_name.to_string(),
        email: register_post.email.to_string(),
        password: register_post.password.to_string(),
    };

    Json(response_data)
}

#[get("/auth/profile")]
pub fn profile() -> &'static str {
    "profile"
}
