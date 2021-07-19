use crate::chess;
use crate::storage;

use hash_ids::HashIds;
use rand::prelude::*;

const HASH_ID_SALT: &str = "CHESS_SERVER";

pub fn generate_new_game_code() -> String {
    HashIds::builder()
    .with_salt(&HASH_ID_SALT)
    .finish()
    .encode(&[random()])
}

pub fn create_new_game() -> Result<String, storage::op::StorageError> {
    storage::op::write_board(chess::initial_board(), generate_new_game_code())
}

pub fn load(code: &str) -> Result<storage::models::Game, storage::op::StorageError> {
    storage::op::read_board(code)
}