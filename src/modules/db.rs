use actix_web::web::{Data, ServiceConfig};
use sqlx::{postgres::PgPoolOptions, Database, PgPool, Postgres};

use crate::actix::AppConfig;

pub type Pool = PgPool;
pub type QueryResult = <Postgres as Database>::QueryResult;
pub type PoolOptions = PgPoolOptions;

#[derive(Clone)]
pub struct DbConfig(Pool);

impl DbConfig {
    pub async fn new() -> Self {
        let database = dotenvy::var("DATABASE_URL").expect("DATABASE could not load");
        let pool = PoolOptions::new()
            .max_connections(10)
            .connect(&database)
            .await
            .expect("Pg pool not conected");

        Self(pool)
    }
}

impl AppConfig for DbConfig {
    fn configure(self, config: &mut ServiceConfig) {
        config.app_data(Data::new(self.0));
    }
}