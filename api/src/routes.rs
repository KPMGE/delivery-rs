use rocket::serde::json::Json;
use crate::models::{Route, Position};

#[get("/route")]
pub fn get_all_routes() -> Json<Vec<Route>> {
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
