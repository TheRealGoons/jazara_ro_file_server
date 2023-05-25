use std::env;

use dotenvy::dotenv;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::{form::Form, fs::TempFile};

#[macro_use]
extern crate rocket;

struct SecretKey(String);

#[derive(Debug)]
enum AuthError {
    InvalidKey,
    KeyDoesNotExist,
}

fn is_key_valid(key: &str) -> bool {
    dotenv().ok();
    let correct = env::var("API_KEY").expect("NO API KEY IS SET!");
    if correct == key {
        return true;
    }
    false
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SecretKey {
    type Error = AuthError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        match keys.len() {
            0 => request::Outcome::Failure((Status::BadRequest, AuthError::KeyDoesNotExist)),
            _ => {
                if is_key_valid(keys[0]) {
                    return request::Outcome::Success(SecretKey(keys[0].to_string()));
                } else {
                    return request::Outcome::Failure((Status::BadRequest, AuthError::InvalidKey));
                }
            }
        }
    }
}

#[get("/")]
fn hello() -> String {
    String::from("boobs")
}

#[post("/download")]
async fn download_file() -> () {}

#[post("/upload", data = "<file>")]
async fn upload_file(_key: SecretKey, mut file: Form<TempFile<'_>>) -> std::io::Result<()> {
    file.persist_to("./clientinfo.xml").await
}

#[launch]
fn launch() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/file", routes![upload_file])
}
