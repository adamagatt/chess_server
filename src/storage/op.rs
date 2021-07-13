use diesel::Connection;
use diesel::prelude::*;
use diesel::sql_types;
use diesel::sqlite::SqliteConnection;

use crate::chess::{Board};
use super::models;

no_arg_sql_function!(last_insert_rowid, sql_types::Integer, "Represents the sqlite last_insert_rowid() function");

const DB_FILE: &str = "games.db";

pub enum StorageError {
    Connection(diesel::ConnectionError),
    Query(diesel::result::Error),
    Generic(String)
}

impl From<diesel::result::Error> for StorageError {
    fn from(err: diesel::result::Error) -> StorageError {
        StorageError::Query(err)
    }
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::Connection(err) => write!(f, "{}", err.to_string()),
            StorageError::Query(err) => write!(f, "{}", err.to_string()),
            StorageError::Generic(msg) => write!(f, "{}", msg)
        }
    }
}

fn get_storage() -> Result<diesel::SqliteConnection, StorageError> {
    SqliteConnection::establish(DB_FILE).map_err(StorageError::Connection)
}

fn get_last_insert_rowid(conn: &diesel::SqliteConnection) -> Result<i32, StorageError>{
    diesel::select(last_insert_rowid)
    .first(conn)
    .map_err(StorageError::Query)
}

pub fn write_board(_board: Board) -> Result<i32, StorageError> {

    get_storage()
    .and_then(|conn: diesel::SqliteConnection| {
        conn.transaction(|| {
            diesel::insert_into(models::games::table)
                .values(&models::Game{id: None})
                .execute(&conn)
                .map_err(StorageError::Query)
            .and_then(
                |a: usize| if a > 0 {
                    get_last_insert_rowid(&conn)
                } else {
                    Err(StorageError::Generic(String::from("New game wasn't inserted")))
                }
            )
        })
    })
}