#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;

pub mod chess;
mod game;
mod render;
mod storage;

use rocket::http::Status;
use storage::op::StorageError;
use storage::models::Game;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        index,
        create_new_game,
        find_game_by_code
    ])
}

#[get("/")]
fn index() -> (Status, String) {
    (Status::Ok, render::to_text(chess::initial_board(), render::get_space_map()))
}

#[post("/game")]
fn create_new_game() -> (Status, String) {
    game::create_new_game().map_or_else(
        |_err: StorageError| (Status::InternalServerError, String::from("")),
        |new_game_code: String| (Status::Created, new_game_code)
    )
}

#[get("/game/<code>")]
fn find_game_by_code(code: &str) -> (Status, String) {
    game::load(code)
    .map_or_else(
        |err: StorageError| match err {
            StorageError::Query(err) if (err==diesel::NotFound)
              => (Status::NoContent, String::from("")),
            _ => (Status::InternalServerError, String::from(""))
        },
        |found_game: Game| (Status::Ok, found_game.code)
    )
}