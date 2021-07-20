#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;

pub mod chess;
mod game;
mod render;
mod storage;

use rocket::http::Status;
use storage::op::StorageError;
use storage::models::Game;

type Response = (Status, String);

fn server_error_response() -> Response {
    (Status::InternalServerError, String::from(""))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        index,
        create_new_game,
        find_game_by_code
    ])
}

#[get("/")]
fn index() -> Response {
    (Status::Ok, render::to_text(chess::initial_board(), render::get_space_map()))
}

#[post("/game")]
fn create_new_game() -> Response {
    game::create_new_game().map_or_else(
        |_err: StorageError| server_error_response(),
        |new_game_code: String| (Status::Created, new_game_code)
    )
}

#[get("/game/<code>")]
fn find_game_by_code(code: &str) -> Response {
    game::load(code)
    .map_or_else(
        |err: StorageError| match err {
            StorageError::Query(err) if (err==diesel::NotFound)
              => (Status::NoContent, String::from("")),
            _ => server_error_response()
        },
        |found_game: Game| found_game.state.map_or(
            (Status::Ok, String::from("No board found though sorry")),
            |bytes: Vec<u8>| match storage::op::binary_to_board(bytes) {
                Ok(board) => (Status::Ok, render::to_text(board, render::get_space_map())),
                Err(_) => server_error_response()
            }
        )
    )
}