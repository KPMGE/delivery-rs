use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    PgPool, Row,
};

use crate::models::{PositionDto, PositionDb, RouteInputDto, RouteOutputDto, RouteDb};

pub struct Repository {
    pub db_pool: PgPool,
}

impl Repository {
    pub async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(db_url)
            .await?;

        Ok(Repository { db_pool: pool })
    }

    pub async fn insert_position(&self, new_position: PositionDto) -> Result<PositionDb, sqlx::Error> {
        sqlx::query("INSERT INTO positions (lat, lng) VALUES($1, $2) RETURNING id, lat, lng")
            .bind(new_position.lat)
            .bind(new_position.lng)
            .map(|row: PgRow| PositionDb {
                id: row.get::<i32, _>("id"),
                lat: row.get::<f32, _>("lat") as f64,
                lng: row.get::<f32, _>("lng") as f64,
            })
            .fetch_one(&self.db_pool)
            .await
    }

    pub async fn fetch_position(&self, position_id: i32) -> Result<PositionDb, sqlx::Error> {
        sqlx::query("SELECT * from positions WHERE id = $1")
            .bind(position_id)
            .map(|row: PgRow| PositionDb {
                id: row.get::<i32, _>("id"),
                lat: row.get::<f32, _>("lat") as f64,
                lng: row.get::<f32, _>("lng") as f64,
            })
            .fetch_one(&self.db_pool)
            .await
    }

    pub async fn insert_route(&self, new_route: RouteInputDto) -> Result<RouteOutputDto, sqlx::Error> {
        let start_position = self.insert_position(new_route.start_position).await?;
        let end_position = self.insert_position(new_route.end_position).await?;

         sqlx::query("INSERT INTO routes (title, start_position_id, end_position_id) VALUES($1, $2, $3) RETURNING id, title")
            .bind(new_route.title.clone())
            .bind(start_position.id)
            .bind(end_position.id)
            .map(|row: PgRow| RouteOutputDto { 
                id: row.get::<i32, _>("id"),
                title: row.get("title"),
                start_position: PositionDto {
                    lat: start_position.lat,
                    lng: start_position.lng
                }, 
                end_position: PositionDto {
                    lat: end_position.lat,
                    lng: end_position.lng
                }
            })
            .fetch_one(&self.db_pool)
            .await
    }

    pub async fn get_all_routes(&self) -> Result<Vec<RouteOutputDto>, sqlx::Error> {
        let db_routes = sqlx::query("SELECT * FROM routes")
        .map(|row: PgRow| RouteDb {
            id: row.get::<i32, _>("id"),
            title: row.get("title"),
            start_position_id: row.get::<i32, _>("start_position_id"),
            end_position_id: row.get::<i32, _>("end_position_id")
        })
        .fetch_all(&self.db_pool)
        .await?;

        let mut all_routes = Vec::new();

        for route in db_routes {
            let start_pos_db = self.fetch_position(route.start_position_id).await?;
            let end_pos_db = self.fetch_position(route.end_position_id).await.unwrap();
            let start_position = PositionDto {
                lat: start_pos_db.lat,
                lng: start_pos_db.lng
            };
            let end_position = PositionDto {
                lat: end_pos_db.lat,
                lng: end_pos_db.lng
            };

            all_routes.push(
                RouteOutputDto {
                    id: route.id, 
                    title: route.title,
                    start_position,
                    end_position
                }
            );
        }

        Ok(all_routes)
    }
}
