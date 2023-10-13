use std::fs::OpenOptions;
use std::io::{self, Write};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    path: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&args.path)?;

    let mut input = String::new();
    println!("Enter the text to append to the file (type 'END' on a new line to finish):");

    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        if buffer.trim() == "END" {
            break;  // End of input
        }
        input.push_str(&buffer);
    }

    file.write_all(input.as_bytes())?;
    println!("Data appended to the file successfully.");
    Ok(())
}
