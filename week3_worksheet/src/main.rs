use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::io;
pub mod geometry;
use geometry::{Circle, Rectangle, Area, Perimeter};
pub mod capitalize;
use capitalize::capitalize_1st_letter;
fn main(){
// 1.   
println!("#Task 1");
        println!("Enter a sentence: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error to read that line.");
        let ans = sentence_spliter(&input); 
        println!("{:?}", ans);
        println!();

// 2. //
println!("#Task 2");
    println!("Enter a vector of integers separated by spaces please: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input.");

    let mut numbers: Vec<i32> = Vec::new();

    for s in input.split_whitespace() {
        if let Ok(num) = s.parse() { // parse() returns a Result type, which is an enum with two variants: Ok and Err. error handling numbers because parse() can fail if the input is not a valid integer.
            numbers.push(num);
        } else {
            println!("Invalid integer: {}", s);
        }
    }
    let ans = sqr_sum(&numbers);
    println!("Sum of squared values: {}", ans);
    println!();

// 3. // program reads a text file and counts the frequency of each word using a HashMap. Ignore punctuation and words in a case-insensitive manner.
println!("#Task 3");
    let mut file = File::open("text.txt").expect("Unable to open file");
    let cont= read_file("text.txt").expect("Unable to read file"); // 4. //
    println!("#Task 4");
    println!("File contents:\n{}", &cont);

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    let mut map = HashMap::new();
    for word in contents.split_whitespace() {
        let count = map.entry(word).or_insert(0); // returns a mutable reference (&mut V) to the value for this key. If the key is not present in the map, the entry() method inserts the key with a value of 0 before returning a mutable reference to the value.
        *count += 1; // dereference count to get the value it refers to, and then add 1 to that value.
    }
    println!("Word frequencies: {:?}", map); // program has counted the frequency of each word, it has a HashMap that maps each word to its frequency. 
    println!();

// 5. // Create a function that parses a string as an integer. If the parsing fails, return an Option with None; otherwise, return Some(parsed_integer).
println!("#Task 5");
    let input = "1234".to_string();
    let ans = parse_int(&input);
    println!("{:?}", ans);
    println!();

// 7. // generic function that filters elements of a vector based on a provided predicate function. The function should work for vectors of different types.
println!("#Task 7");
    let int_vec = vec![1, 2, 3, 4, 5];
    let char_vec = vec!['a', 'b', 'c', 'd', 'e'];
    
    // Define a condition (predicate) to filter even numbers
    let is_even = |x: &i32| x % 2 == 0;
    let even_numbers = filter(&int_vec, is_even);
    println!("Even numbers: {:?}", even_numbers);
    let is_vowel = |x: &char| x == &'a' || x == &'e' || x == &'i' || x == &'o' || x == &'u';
    let vowels = filter(&char_vec, is_vowel);
    println!("Vowels: {:?}", vowels);
    println!();
// 8 // 
println!("#Task 8");
    let shape = Shape {
        name: "Circle".to_string(),
    };
    shape.draw();
    println!();

// 9 complements the code in week3_worksheet/src/lib/geometry.rs
println!("#Task 9");
    let circle = Circle { radius: 2.0 };
    println!("Area of circle: {:?}", circle.area());
    println!("Perimeter of circle: {:?} ", circle.perimeter());
    let rectangle = Rectangle {
        width: 2.0,
        height: 6.0,
    };
    println!("Area of rectangle: {:?}", rectangle.area());
    println!("Perimeter of rectangle: {:?}", rectangle.perimeter());
    println!();

println!("#Task 10");
// 10. //
    let sentence = "i want the french fries";
    let ans = capitalize_1st_letter(sentence);
    println!("Capitalized sentence: {}", ans);
    println!();
// 12. //
println!("#Task 12");
     let mut std_scores = HashMap::new();
     std_scores.insert("Fareeha".to_string(), 95);
     std_scores.insert("Danial".to_string(), 92);
     std_scores.insert("Azca".to_string(), 88);
 
     let threshold = 90; // threshold for high scorers
 
     let high_scorers = threshold_std(&std_scores, threshold); 
 
     for (name, score) in high_scorers {
         println!("{} scored above {} with a score of {}", name, threshold, score);
     } println!();

// 13. //
println!("#Task 13");
    let ans = division(10, 0);
    println!("{:?}", ans);
    println!();
}
                                                                    // 13 //
/*
    13.Create a function that divides two integers and returns a Result with the result
    of the division if the denominator is not zero. If the denominator is zero, return
    an error indicating division by zero.
*/
fn division(a: i32, b: i32) -> Result<i32, String> {
    match b{
        0 => Err("Division by zero".to_string()),
        _ => Ok(a / b),
    }
}

                                                                    // 12 //
/*
        12.Implement a function that takes a HashMap containing student names and
        their scores as input. Return a new HashMap with only the students who
        scored above a certain threshold.
*/

fn threshold_std( students: &HashMap<String, u32>, threshold: u32,) -> HashMap<String, u32> {
    let mut new_hashmap: HashMap<String, u32> = HashMap::new();
    for (name, score) in students { // iterate over the key-value pairs in the HashMap
        if *score > threshold { // dereference score to get the value it refers to
            new_hashmap.insert(name.clone(), *score); // clone() to create a new String with the same contents as the original.
        }
    }
    new_hashmap
}


                                                                    // 8 // 
//  Create a struct called Shape that stores the name of a shape and implement the Drawable trait for it.
struct Shape {
    name: String,
}
//  Define a trait called Drawable with a method draw.
trait Drawable {
    fn draw(&self);
}
impl Drawable for Shape {
    fn draw(&self){
        println!("Drawing a shape: {}", self.name);
    }
}
// 7 
fn filter<T, F>(vector: &Vec<T>, condition: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
    T: Clone,
{
    let mut new_vec: Vec<T> = Vec::new();
    for i in vector {
        if condition(i) {
            new_vec.push(i.clone());
        }
    }
    new_vec
}



//5
fn parse_int(input : &str) -> Option<i32>{
    match input.parse::<i32>() { //  string should be parsed into an i32 integer.
        Ok(parsed_int) => Some(parsed_int),
        Err(_) => None,
    }
}

//4
fn read_file(read : &str) -> Result<String, std::io::Error>{
    let mut file = File::open(read)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

//2
fn sqr_sum(numbers: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for &num in numbers {
        sum += num * num;
    }
    sum
}
//1
fn sentence_spliter(sentence: &str) -> Vec<String>{
    let mut vec : Vec<String> = Vec::new();
    let mut word = String::new();
    for i in sentence.chars(){
        if i.is_ascii_alphanumeric() { // to avoid input text might have contained newline characters, which are typically represented as \r\n
            word.push(i);
           // word.clear();
        }
        else if i == ' ' {
            vec.push(word.clone());
            word.clear();
        }
        else{
            word.push(i); // Push the last word as there is no space after the last word
            vec.sort();   // Sort the vector alphabetically
        }
    }
    vec
}
