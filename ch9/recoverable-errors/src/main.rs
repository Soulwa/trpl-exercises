use std::fs;
use std::io;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	println!("{:?}", read_username_from_file()?);
	Ok(())	
}

fn read_username_from_file() -> Result<String, io::Error> {
	fs::read_to_string("hello.txt")
}