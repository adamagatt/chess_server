use diesel::Connection;
use diesel::prelude::*;
use diesel::sql_types;
use diesel::sqlite::SqliteConnection;

use crate::chess::{Board};
use super::models;

no_arg_sql_function!(last_insert_rowid, sql_types::Integer, "Represents the sqlite last_insert_rowid() function");

const DB_FILE: &str = "games.db";

pub enum StorageError {
    Serialize(Box<bincode::ErrorKind>),
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
            StorageError::Serialize(err) => write!(f, "{}", err.to_string()),
            StorageError::Connection(err) => write!(f, "{}", err.to_string()),
            StorageError::Query(err) => write!(f, "{}", err.to_string()),
            StorageError::Generic(msg) => write!(f, "{}", msg)
        }
    }
}

fn get_storage() -> Result<diesel::SqliteConnection, StorageError> {
    SqliteConnection::establish(DB_FILE).map_err(StorageError::Connection)
}

fn get_last_insert_rowid(conn: &diesel::SqliteConnection) -> Result<i32, StorageError> {
    diesel::select(last_insert_rowid)
    .first(conn)
    .map_err(StorageError::Query)
}

pub fn write_board(board: Board, code: String) -> Result<String, StorageError> {

    let conn_result = get_storage();
    let serialised_board_result = board_to_binary(board);

    if let(Ok(conn), Ok(board_serialised)) = (conn_result, serialised_board_result) {
        let entry = models::NewGame {
            code: &code.to_string(),
            state: Some(board_serialised)
        };

        diesel::insert_into(models::games::table)
            .values(entry)
            .execute(&conn)
            .map_err(StorageError::Query)
        .and_then(
            |inserts: usize| {
                if inserts > 0 {
                    Ok(code)
                } else {
                    Err(StorageError::Generic(String::from("Failed to create new game")))
                }
            }
        )
    } else {
        Err(StorageError::Generic(String::from("Failed to create new game")))
    }
}

pub fn read_board(search_code: &str) -> Result<models::Game, StorageError> {
    use models::games::dsl::*;

    get_storage()
    .and_then(|conn: diesel::SqliteConnection| {
        games.filter(code.eq(search_code))
            .first::<models::Game>(&conn)
            .map_err(StorageError::Query)
        }
    )
}

pub fn board_to_binary(board: Board) -> Result<Vec<u8>, Box<bincode::ErrorKind>> {
    bincode::serialize(&board)
}

pub fn binary_to_board(bytes: Vec<u8>) -> Result<Board, Box<bincode::ErrorKind>> {
    bincode::deserialize(&bytes)
}