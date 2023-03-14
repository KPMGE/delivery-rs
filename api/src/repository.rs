use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    PgPool, Row,
};

use crate::models::{Position, PositionDb};

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

    pub async fn insert_position(&self, new_position: Position) -> Result<PositionDb, sqlx::Error> {
        match sqlx::query("INSERT INTO positions (lat, lng) VALUES($1, $2) RETURNING id, lat, lng")
            .bind(new_position.lat)
            .bind(new_position.lng)
            .map(|row: PgRow| PositionDb {
                id: row.get::<i32, _>("id"),
                lat: row.get::<f32, _>("lat") as f64,
                lng: row.get::<f32, _>("lng") as f64,
            })
            .fetch_one(&self.db_pool)
            .await
        {
            Ok(user) => Ok(user),
            Err(e) => Err(e),
        }
    }
}
