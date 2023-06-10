use std::{env, fs};

use dotenvy::dotenv;
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};

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
                    return request::Outcome::Failure((
                        Status::Unauthorized,
                        AuthError::InvalidKey,
                    ));
                }
            }
        }
    }
}

#[get("/")]
async fn download_file() -> NamedFile {
    NamedFile::open("clientinfo.xml")
        .await
        .expect("no xml file")
}

#[get("/upload/<ip>")]
async fn upload_file(_key: SecretKey, ip: String) {
    create_xml_file(ip)
}

#[launch]
fn launch() -> _ {
    rocket::build()
        .mount("/", routes![download_file])
        .mount("/file", routes![upload_file])
}

fn create_xml_file(ip: String) {
    let xml_before = "<?xml version=\"1.0\" encoding=\"euc-kr\" ?>
<clientinfo>
<desc>Jazara</desc>
	<servicetype>korea</servicetype>
	<servertype>primary</servertype>
	<connection>
		<display>JazaraRO</display>
      		<address>";
    let xml_after = "</address>
      		<port>6900</port>
      		<version>55</version>
      		<langtype>19</langtype>
		<registrationweb>https://www.youtube.com/watch?v=dQw4w9WgXcQ</registrationweb>
		<loading>
			<image>loading00.jpg</image>
			<image>loading01.jpg</image>
			<image>loading02.jpg</image>
			<image>loading03.jpg</image>
			<image>loading04.jpg</image>
			<image>loading05.jpg</image>
			<image>loading06.jpg</image>
		</loading>
   	</connection>
</clientinfo>";
    let xml = [xml_before, &ip, xml_after].join("");
    fs::write("clientinfo.xml", xml).expect("failed to write xmlfile");
}
