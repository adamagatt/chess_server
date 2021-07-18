#[macro_use] extern crate diesel;
#[macro_use] extern crate rand;
#[macro_use] extern crate rocket;
#[macro_use] extern crate hash_ids;

pub mod chess;
mod game;
mod render;
mod storage;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, create_new_game])
}

#[get("/")]
fn index() -> String {
    render::to_text(chess::initial_board(), render::get_space_map())
}

#[post("/game")]
fn create_new_game() -> String {
    game::create_new_game().map_or_else(
        |err: storage::op::StorageError| err.to_string(),
        |new_game_code: String| new_game_code
    )
}