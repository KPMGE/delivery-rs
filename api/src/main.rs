#[macro_use]
extern crate rocket;

mod models;
mod routes;
mod repository;

use repository::Repository;
use dotenv::dotenv;

#[launch]
async fn rocket() -> _ {
    // load environment variables
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let repo = Repository::new(db_url.as_str())
        .await
        .expect("error while connecting to the database");

    rocket::build()
        .manage::<Repository>(repo).mount("/", routes![
            routes::get_all_routes,
            routes::add_route, 
       ],
    )
}
