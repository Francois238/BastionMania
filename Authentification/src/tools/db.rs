
use crate::tools::api_error::ApiError;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;


pub fn connection() -> Result<PgConnection, ApiError> {
    let database_url = env::var("DATABASE_URL")
        .map_err(|_| ApiError::new(500, "DATABASE URL missing".to_string()))?;

    PgConnection::establish(&database_url)
        .map_err(|_| ApiError::new(500, "Failed to connect to Postgres".to_string()))
}