pub struct Book {
    pub title: String,
    pub author: String,
    pub pages: u32,
}

pub fn display_book(book: &Book) {
    println!("Title: {}", book.title);
    println!("Author: {}", book.author);
    println!("Pages: {}", book.pages);
}

