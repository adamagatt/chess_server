#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;

pub mod chess;
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
    let board: chess::Board = chess::initial_board();
    storage::op::write_board(board).map_or_else(
        |err: storage::op::StorageError| err.to_string(),
        |new_id: i32| -> String {
            new_id.to_string()
        }
    )
}