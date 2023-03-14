use models::Position;
use sqlx::{postgres::PgPoolOptions, PgPool};

#[macro_use]
extern crate rocket;

mod models;
mod routes;

#[get("/test")]
async fn insert_route(db_pool: &rocket::State<PgPool>) {
    let new_position = Position {
        lat: 10.25,
        lng: 62.52,
    };

    sqlx::query("INSERT INTO positions (lat, lng) VALUES($1, $2)")
        .bind(new_position.lat)
        .bind(new_position.lng)
        .execute(db_pool.inner())
        .await
        .expect("error while inserting position into the database");

    println!("success!");
}

#[launch]
async fn rocket() -> _ {
    let db_url = "postgres://postgres:1234@localhost:5432/deliver_rs";
    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(db_url)
        .await
        .expect("error while connecting to database");

    rocket::build().manage::<PgPool>(db_pool).mount(
        "/",
        routes![routes::get_all_routes, routes::add_route, insert_route],
    )
}
