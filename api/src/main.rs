mod models;
mod routes;
mod repository;
mod websocket;
mod kafka;

use std::thread;
use dotenv::dotenv;
use repository::Repository;
use websocket::WebSocketServer;
use rocket::http::{Header};
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

#[macro_use]
extern crate rocket;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        println!("Setting access control allow origin");
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
  
    }
}

#[launch]
async fn rocket() -> _ {
    // load environment variables
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let repo = Repository::new(db_url.as_str())
        .await
        .expect("error while connecting to the database");

    thread::spawn(|| {
        let url = "0.0.0.0:3012";
        println!("websocket server on: ws://{}", url);
        ws::listen(url, |out| {
            WebSocketServer { out }
        })
        .unwrap();
    });

    rocket::build()
        .manage::<Repository>(repo)
        .attach(CORS)
        .mount("/", routes![
            routes::get_all_routes,
            routes::add_route, 
       ],
    )
}
