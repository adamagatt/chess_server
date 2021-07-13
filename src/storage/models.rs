table! {
    games {
        id -> Integer,
    }
}

#[derive(Insertable)]
pub struct Game {
    pub id: Option<i32>
}