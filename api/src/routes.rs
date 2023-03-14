use crate::models::{Position, Route};
use rocket::serde::json::Json;

#[get("/routes")]
pub fn get_all_routes() -> Json<Vec<Route>> {
    let sample_users = vec![
        Route {
            id: "1".to_string(),
            title: "Rota 1".to_string(),
            start_position: Position {
                lat: 12.31,
                lng: 13.13,
            },
            end_position: Position {
                lat: 53.51,
                lng: 12.25,
            },
        },
        Route {
            id: "1".to_string(),
            title: "Rota 1".to_string(),
            start_position: Position {
                lat: 12.31,
                lng: 13.13,
            },
            end_position: Position {
                lat: 53.51,
                lng: 12.25,
            },
        },
    ];

    Json(sample_users)
}

#[post("/routes", data = "<new_route>")]
pub fn add_route(new_route: Json<Route>) -> Json<Route> {
    println!("route received: {:?}", new_route);

    let r = Route {
        id: "1".to_string(),
        title: "Rota 1".to_string(),
        start_position: Position {
            lat: 12.31,
            lng: 13.13,
        },
        end_position: Position {
            lat: 53.51,
            lng: 12.25,
        },
    };

    Json(r)
}
