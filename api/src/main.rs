mod models;
mod routes;
mod repository;
mod websocket;

use std::thread;
use dotenv::dotenv;
use repository::Repository;
use websocket::WebSocketServer;

#[macro_use]
extern crate rocket;

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
        let url = "127.0.0.1:3012";
        println!("websocket server on: ws://{}", url);
        ws::listen(url, |out| {
            WebSocketServer { out }
        })
        .unwrap();
    });

    rocket::build()
        .manage::<Repository>(repo).mount("/", routes![
            routes::get_all_routes,
            routes::add_route, 
       ],
    )
}
