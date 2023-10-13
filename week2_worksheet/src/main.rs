mod utils;
use crate::utils::Book;
use utils::display_book;
use utils::Book as BookshelfBook;
//use bookshelf::{Book as BookshelfBook};  // Import the Boook struct and display_book function from your utils module // Import the BookshelfBook and Status types from the library

struct Person {
    name: String,
    age: u32,
}
#[derive(Debug)]
enum Status {
    Active,
    Inactive,
    Suspended,
}
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

fn rgb(color: Color) -> (u8, u8, u8) {
    match color {
        Color::Red => (255, 0, 0),
        Color::Green => (0, 255, 0),
        Color::Blue => (0, 0, 255),
    }
}

fn sum_of_tuple_elements(tuple: (i32, i32)) -> i32 {
    let (x, y) = tuple;
    x + y
}

enum Value {
    StrValue(String),
    NumValue(u8),
}

fn my_option(option: Value) {
    match option {
        Value::StrValue(_) => {
            println!("It's a string type");
        }
        Value::NumValue(num) => {
            println!("It's a number type: {}", num);
        }
    }
}

fn borrow_strings(s: &mut String) {
    s.push_str(" World!");
}

impl Book {
    fn new(title: &str, author: &str, pages: i32) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            pages: pages.try_into().unwrap(),
        }
    }

    fn get_title(&self) -> &String {
        &self.title
    }

    fn print_author(&self) {
        println!("Author: {}", &self.author);
    }

    fn is_it_val(x: Option<i32>) {
        match x {
            Some(_) => println!("Has a value"),
            None => println!("No value"),
        }
    }

    fn not_take_ownership_and_return_title(book: &Book, new_title: &str) -> String {
        let mut modified_title = book.title.clone();
        modified_title.push_str(new_title);
        modified_title
    }
}
impl Status {
    fn stat(s: Status) -> String {
        match s {
            Status::Active => "The status is Active".to_string(),
            Status::Inactive => "The status is Inactive".to_string(),
            Status::Suspended => "The status is Suspended".to_string(),
        }
    }

    fn book_info(title: &String, stat: Status) -> (&String, Status) {
        match stat {
            Status::Active => (title, Status::Active),
            Status::Inactive => (title, Status::Inactive),
            Status::Suspended => (title, Status::Suspended),
        }
    }
}

fn main() {
    // 1
    let p1 = Person {
        name: "Fareeha".to_string(),
        age: 20,
    };
    println!("Person's name: {}", p1.name);
    println!();

    // 2
    let red = Color::Red;
    let red_rgb = rgb(red);
    println!("Color of red in RGB is: {:?}", red_rgb);
    println!();

    // 3
    let result = sum_of_tuple_elements((5, 5));
    println!("Sum of tuple elements: {}", result);
    println!();

    // 4
    let is_num = Value::NumValue(2);
    my_option(is_num);
    println!();

    // 5
    let mut s = String::from("Hello");
    borrow_strings(&mut s);
    println!("{}", s);
    println!();

    // 6
    let s = Book::new("I'm Alive", "Fareeha", 250);
    let ss = s.get_title();
    println!("Book title: {:?}", ss);
    println!();

    // 7.A
    let b = Book {
        title: "Hehe".to_string(),
        author: "Fareeha".to_string(),
        pages: 20,
    };
    println!("Title: {}", b.title);
    println!();

    // 7.C
    let binding = "Mary Poppins".to_string();
    let try7a = Status::book_info(&binding, Status::Active);
    println!("{:?}", try7a);
    println!();

    // 8.A
    let try8a = Status::stat(Status::Active);
    println!("{:?}", try8a);
    println!();

    // 8.Destructure the Book struct to extract and print the author's name.
    let destroy_bookstruct = Book::new("Hehe", "Fareeha", 250);
    destroy_bookstruct.print_author();
    println!();

    // 8.B
    let _val = Book::is_it_val(Some(5));
    let _val2 = Book::is_it_val(None);

    // 9
    let b2 = Book::new("Eleanor & Park", "Her", 899);
    let print_b2 = Book::not_take_ownership_and_return_title(&b2, " Hers");
    println!("{:?}", print_b2);

    // 10
    // 11
    let book = BookshelfBook {
        title: "Rust Programming".to_string(),
        author: "John Doe".to_string(),
        pages: 300,
    };
    display_book(&book);
    // 12
    /*
     git init          # Initialize a new Git repository
    git add .         # Add all files in the directory to the staging area
    git commit -m "Initial commit"  # Commit the changes with a message

    [dependencies]
    my_crate = "1.0.0"
    */
}
