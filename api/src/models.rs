use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, FromForm, Clone)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct PositionDto {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Serialize, Deserialize, Debug, FromForm, Clone)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct PositionDb {
    pub id: i32,
    pub lat: f64,
    pub lng: f64,
}

#[derive(Serialize, Deserialize, Debug, FromForm, Clone)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct RouteInputDto {
    pub title: String,
    pub start_position: PositionDto,
    pub end_position: PositionDto,
}


#[derive(Serialize, Deserialize, Debug, FromForm, Clone)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct RouteOutputDto {
    pub id: i32,
    pub title: String,
    pub start_position: PositionDto,
    pub end_position: PositionDto,
}

#[derive(Serialize, Deserialize, Debug, FromForm)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct RouteDb {
    pub id: i32,
    pub title: String,
    pub start_position_id: i32,
    pub end_position_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteMessage {
    pub route_id: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    pub route_id: String,
    pub client_id: String
}
