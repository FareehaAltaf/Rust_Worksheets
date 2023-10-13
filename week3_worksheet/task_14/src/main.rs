/*
    14.Implement a program that reads data from a CSV file and converts it into a custom data structure. 
    Define custom error types for various potential parsing errors 
    use the ? operator for error handling.
*/

use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Step 1: Define your custom data structure
#[derive(Debug)]
struct MyData {
    name: String,
    age: u32,
    location: String,
}


// Step 2: Create a custom error type
#[derive(Debug)]
enum MyError {
    // Define various parsing error types
    InvalidName,
    InvalidAge,
    InvalidLocation,
    InvalidData,
}

fn main() -> Result<(), Box<dyn std::error::Error> > {
    let file_path = "file.csv";
    let file = File::open(&file_path)?;

    // Create a reader to read the file line by line
    let reader = io::BufReader::new(file);

    // vector holds the parsed data ; MyData which is the custom data structure
    let mut data: Vec<MyData> = Vec::new();

    // Read and parse into each line
    for line in reader.lines() {
        let line = line?; // Unwrap the line & check for errrs
        match parse_line(&line) {
            Ok(parsed_data) => {
                data.push(parsed_data);
            }
            Err(e) => {
                // Handle parsing errors (e.g., print the error or log it)
                eprintln!("Error parsing line: {:?}", e);
            }
        }
    }

    // Print the parsed data
    for d in &data {
        println!("{:?}", d);
    }
    Ok(())

}

// Step 4: Implement a function to parse a single line into MyData
fn parse_line(line: &str) -> Result<MyData, MyError> {
    let parts: Vec<&str> = line.split(',').collect();

    if parts.len() != 3 {
        // Ensure that each line has three fields (name, age, location)
        return Err(MyError::InvalidData);
    }

    let name = match parts[0].to_string().trim().parse() {
        Ok(name) => name,
        Err(_) => return Err(MyError::InvalidName),
    };
    
    let age = match parts[1].parse() {
        Ok(age) => age,
        Err(_) => return Err(MyError::InvalidAge),
    };
    
    let location = match parts[2].to_string().trim().parse() {
        Ok(location) => location,
        Err(_) => return Err(MyError::InvalidLocation),
    };
    
    Ok(MyData { name, age, location })
}
