use actix_web::{middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::env;

//create a function that accepts a string as input and returns a sha256 hash of the string
pub fn sha(text: &str) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(text);
    let result = hasher.finalize();
    format!("{:x}", result)
}

//create a struct that will be used to deserialize the JSON payload
#[derive(Deserialize)]
struct Text {
    text: String,
}

//create a struct to serialize the response using serde and hash as the key
#[derive(serde::Serialize)]
struct Hash {
    hash: String,
}

// /api is default route - can customize it
// exposed endpoints should go under here
#[post("/api/token")]
async fn tokenize(text: web::Json<Text>) -> impl Responder {
    // receive json text - hashes input - responds
    //use the hash function to create a hash of the text and then return the hash
    //as JSON
    let hash = sha(&text.text);
    // return the hash as JSON using the Hash struct defined above
    HttpResponse::Ok().json(Hash { hash })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // instantiate logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // following critical for azure functions
    // azure functions - you get a special port
    // need to make sure app has same port mapping
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT"; // set when 'bind' called in application
    // 13:00 in recording if it comes up again, i think its set up fine now though


    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(tokenize)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}