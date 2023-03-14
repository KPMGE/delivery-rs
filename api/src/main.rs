#[macro_use]
extern crate rocket;

mod models;
mod routes;
mod repository;

use repository::Repository;

#[launch]
async fn rocket() -> _ {
    let repo = Repository::new("postgres://postgres:1234@localhost:5432/deliver_rs")
        .await
        .expect("error while connecting to the database");

    rocket::build()
        .manage::<Repository>(repo).mount("/", routes![
            routes::get_all_routes,
            routes::add_route, 
       ],
    )
}
