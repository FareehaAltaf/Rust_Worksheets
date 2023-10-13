
pub struct Book {
    pub title: String,
    pub author: String,
    pub status: Status,
}

pub enum Status {
    Available,
    CheckedOut,
}
