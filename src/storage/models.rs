table! {
    games {
        id -> Integer,
        code -> Text,
        state -> Nullable<Binary>,
    }
}

#[derive(Insertable)]
#[table_name="games"]
pub struct NewGame<'a> {
    pub code: &'a str,
    pub state: Option<Vec<u8>>
}

#[derive(Queryable)]
pub struct Game {
    pub id: i32,
    pub code: String,
    pub state: Option<Vec<u8>>
}