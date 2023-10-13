
/* 1. Define a struct called Person with fields for name (String) and age (u32).
Create an instance of the Person struct and print its name field. DONE
2. Define an enum called Color with variants for Red, Green, and Blue. Write a
function that takes a Color enum as an argument and returns a
corresponding RGB value as a tuple. DONE
3. Write a function that accepts a tuple (i32, i32) and returns the sum of its
elements.  DONE
4. Create an Option enum to represent either a string or a number. Write a
function to print the value if it's a number. 
5. Write a function that borrows a string and appends " World!" to it.
6. Define a struct Book with a title (String) and implement a method get_title
that returns a reference to the title.
7. Define a struct named Book with fields title, author, and pages.
a. Create an instance of the Book struct and print its title.        DONE
b. Define an enum named Status with variants Active, Inactive, and   DONE
Suspended. 
c. Write a function that takes a Book and a Status and returns a tuple DONE
containing the book's title and its status.
8. Given the enum Status from the previous section, use pattern matching to DONE
write a function that returns a string description of each variant.
a. Destructure the Book struct to extract and print the author's name.
b. Write a match statement for an Option<i32> that prints "Has a value"
if it's Some and "No value" if it's None. DONEE
9. Create a function that takes ownership of a Book and returns its title.
a.Write a function that borrows a Book and modifies its title. Ensure it
doesn't consume the book.
*/
#[warn(unused_variables)]
struct Person{
    name:String,
    age: u32,
}
#[derive(Debug)]
 #[warn(dead_code)]
#[warn(unused_variables)]
enum Color{
    Red, 
    Green, 
    Blue,
}
// fn Color enum as an argument and returns a corresponding RGB value as a tuple.
#[warn(unreachable_patterns)]
fn rgb(color : Color) -> (u8, u8, u8){
    match color {
        Color::Red =>  (255,0,0),
        Color::Green => (0,255,0),
        Color::Blue => (0,0,255),
        _ => (0,0,0),
    }
}
//3
fn sum_of_tuple_elements(tuple: (i32, i32)) -> i32 {
    let (x, y) = tuple; // Destructure the tuple into individual elements
    x + y // Return the sum of the elements
}

// 4
enum Value {
    StrValue(String),
    NumVal(u8),
}
#[warn(unused_variables)]
fn my_option(option: Value) {
    match option {
        Value::StrValue(_) => {
            println!("It's a string type");
        }
        Value::NumVal(num) => {
            println!("It's a number type {}", num);
        }
    }
}
// 5
    fn borrowstrings (s : &mut String) {
        s.push_str("World!")
    }
// 6. Define a struct Book with a title (String) and implement a method get_title that returns a reference to the title.
#[warn(unused_variables)]
struct Book {
    title:String, 
    author:String,
    pages:i32,
}
impl Book{
    fn new(title:&str,author:&str,pages:i32 ) -> Self{
        Self{
            title : title.to_string(),
            author : author.to_string(),
            pages : pages,
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
    
    fn notake_ownership_and_return_title(book: &Book, new_title: &str) -> String {
        let mut modified_title = book.title.clone(); // Clone the title
        modified_title.push_str(new_title); // Append the new_title
        modified_title // Return the modified title
    }
    


}
// 7.B
#[derive(Debug)]
#[warn(unused_variables)]
enum Status{
    Active, Inactive, Suspended,
}
//7 c
impl Status{
    fn book_info(title : &String, stat : Status) -> (&String,Status){
        match stat{
            Status::Active => (title,Status::Active) ,
            Status::Inactive => (title,Status::Inactive) ,
            Status::Suspended => (title,Status::Suspended) ,
        }
    }

// 8.a use pattern matching to write a function that returns a string description of each variant.
    fn stat(s : Status) -> String{
        match s{
            Status::Active =>  "The status is Active".to_string(),
            Status::Inactive =>  "The status is Inactive".to_string(),
            Status::Suspended =>"The status is Suspended".to_string(),
        }
    }
}

fn main() {
// 1
    let p1 = Person{
        name:"Fareeha".to_string(),
        age : 20,
    };
    println!("p1 is {}", p1.name);
    println!();
// 2
    let red = Color::Red;
    let red_rgb = rgb(red);
    println!("Color of red in rgb is {:?}",red_rgb );
    println!();
// 3
    //let my_tuple = (5, 7);
    let result = sum_of_tuple_elements((5,5));
    println!("Sum of tuple elements: {}", result);
    println!();
// 4
#[warn(unused_variables)]
    let is_num = Value::NumVal(2); println!();

// 5
    let mut s = String::from("Hello");
    borrowstrings(&mut s);
    println!("{}", s);
    println!();
// 6
    let s = Book::new("IM ALIVE", "Fareeha", 250);
    let ss = s.get_title();
    println!("{:?}", ss); println!();

// 7.A instance of a book struct nd print title
    let b = Book {
        title : "hehe".to_string(),
        author:"Fareeha".to_string(),
        pages: 20,
    };
    println!("title: {}", b.title);println!();

// 7 c
    let binding = "Marry Poppins".to_string();
    let try7a = Status::book_info(&binding , Status::Active);
    println!("{:?}",try7a); println!();
// 8 a
    let try8a = Status::stat(Status::Active);
    println!("{:?}",try8a);   println!();
//8 a Destructure the Book struct to extract and print the author's name.
    let destroy_bookstruct = Book::new("hehe","Fareeha",250);
    destroy_bookstruct.print_author(); println!();
//8 b.
    let _val = Book::is_it_val(Some(5));
    let  _val2 =Book::is_it_val(None);

//9 
    let b2= Book::new("Elenor&Pak","Her", 899);
    //let print_b2 = Book::notake_ownership_and_return_title(&b2);
    let print_b2 = Book::notake_ownership_and_return_title(&b2, " Hers");

    println!("{:?}", print_b2);
}
