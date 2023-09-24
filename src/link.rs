use thiserror::Error;

use crate::actions;
use crate::models;
use diesel::{prelude::*, r2d2};
use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    App, HttpResponse,
};

type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("there was an error talking with the database `{0}`")]
    DatabaseError(String),
    #[error("The link could not be found")]
    NotFound,
    #[error("Unkown error occurred")]
    UnknownError,
}

impl error::ResponseError for StorageError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            StorageError::UnknownError => StatusCode::INTERNAL_SERVER_ERROR,
            StorageError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            StorageError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

pub type Result<T> = std::result::Result<T, StorageError>;

#[derive(Clone)]
pub struct Storage {
    pool: r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>,
}

impl Storage {
    pub fn new() -> Self {
        let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
        let manager = r2d2::ConnectionManager::<SqliteConnection>::new(conn_spec);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("database URL should be valid path to SQLite DB file");

        Self { pool }
    }

    pub fn get_by_id(&self, id: &str) -> Result<models::Links> {
        let mut conn = self.pool.get().unwrap(); //TODO
        let url = actions::find_link_by_id(&mut conn, id)
            .map_err(|e| StorageError::DatabaseError(e.to_string()))?;
        if let Some(url) = url {
            Ok(url)
        } else {
            Err(StorageError::NotFound)
        }
    }

    pub fn get_by_url(&self, url: &str) -> Result<models::Links> {
        let mut conn = self.pool.get().unwrap(); //TODO
        let url = actions::find_link_by_url(&mut conn, url)
            .map_err(|e| StorageError::DatabaseError(e.to_string()))?;
        if let Some(url) = url {
            Ok(url)
        } else {
            Err(StorageError::NotFound)
        }
    }

    pub fn put(&self, url: String, id: Option<String>) -> Result<models::Links> {
        let mut conn = self.pool.get().unwrap(); //TODO
        let url = actions::insert_new_url(&mut conn, url, id)
            .map_err(|e| StorageError::DatabaseError(e.to_string()))?;
        Ok(url)
    }
}
