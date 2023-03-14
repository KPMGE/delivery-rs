use rocket::serde::{json::Json, Serialize, Deserialize};

#[macro_use] extern crate rocket;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
struct Position {
    lat: f64,
    lng: f64
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
struct Route {
    id: String,
    title: String,
    start_position: Position,
    end_position: Position
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_all_routes])
}

#[get("/route")]
fn get_all_routes() -> Json<Vec<Route>> {
    let sample_users = vec![
        Route {
            id: "1".to_string(),
            title: "Rota 1".to_string(),
            start_position: Position { 
                lat: 12.31, 
                lng: 13.13
            },
            end_position: Position {
                lat: 53.51,
                lng: 12.25
            }
        },
        Route {
            id: "1".to_string(),
            title: "Rota 1".to_string(),
            start_position: Position { 
                lat: 12.31, 
                lng: 13.13
            },
            end_position: Position {
                lat: 53.51,
                lng: 12.25
            }
        }
    ]; 

    Json(sample_users)
}
