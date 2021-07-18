table! {
    games {
        id -> Integer,
        code -> Text,
        state -> Binary,
    }
}

// Only need to borrow the game code for the lifetime of the insert
#[derive(Insertable)]
pub struct Game<'a> {
    pub id: Option<i32>,
    pub code: Option<&'a String>,
    pub state: Option<Vec<u8>>
}