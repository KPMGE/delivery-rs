use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, FromForm)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Position {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Serialize, Deserialize, Debug, FromForm)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Route {
    pub id: String,
    pub title: String,
    pub start_position: Position,
    pub end_position: Position,
}

#[derive(Serialize, Deserialize, Debug, FromForm)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct RouteDb {
    pub id: String,
    pub title: String,
    pub start_position_id: i32,
    pub end_position_d: i32,
}
