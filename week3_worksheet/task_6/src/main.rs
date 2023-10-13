// 6. //prog that reads a configuration file and returns a custom error type if the file format is invalid. Use the ? operator for error propagation.
    /* 
    "configuring" here means reading and validating the contents of a configuration file to make sure it's set up properly for the program. 
    If the configuration file is empty or malformed, the program reports an error to indicate that it cannot proceed because it lacks the 
    necessary configuration information.
    */
use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::io::ErrorKind;

fn read_configuration(file: &mut File) -> Result<String, std::io::Error> {
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    match contents.is_empty(){
        true => Err(std::io::Error::from(ErrorKind::InvalidData)),
        false => Ok(contents)
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut file = File::open("configuration.txt").expect("Unable to open file");

    match read_configuration(&mut file) {
        Ok(config) => println!("Configuration: {:?}", config),
        Err(err) => println!("Error: {}", err),
    }

    Ok(())
}


