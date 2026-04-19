use std::fs;
use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {

    let contents = "I want to become a PRO in Rust by June, Latest.";

    fs::write("motivation.txt", contents)?;

    println!("\"{}\" has been inserted in the file.", contents);

    let message = "I need to become the best bioinformatics rust developer in Nigeria".to_string();

   let mut file = File::create("output.txt")?;

   file.write_all(message.as_bytes());

   println!("Messages written to file");


    Ok(())
}
