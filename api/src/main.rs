#[macro_use] extern crate rocket;

mod models;
mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        routes::get_all_routes
    ])
}
