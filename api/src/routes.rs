use crate::{
    models::{RouteInputDto, RouteOutputDto},
    repository::Repository,
};
use rocket::{serde::json::Json, State};

#[get("/routes")]
pub async fn get_all_routes(repo: &State<Repository>) -> Json<Vec<RouteOutputDto>> {
    Json(
        repo.get_all_routes()
            .await
            .expect("error while getting routes"),
    )
}

#[post("/routes", data = "<new_route>")]
pub async fn add_route(
    repo: &State<Repository>,
    new_route: Json<RouteInputDto>,
) -> Json<RouteOutputDto> {
    let route = repo
        .insert_route(new_route.into_inner())
        .await
        .expect("error while inserting route into db");

    Json(route)
}
