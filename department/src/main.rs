pub mod directory;

use directory::*;

use std::io;
use std::io::Write;
use std::collections::HashMap;


#[derive(Debug)]
enum Command {
	Add(String, String),
	List(String),
	Error(String),
	Help,
	Exit
}

fn main() {
	let mut directory: HashMap<String, Vec<String>> = HashMap::new();
    println!("input your command, or help to see a list of commands:");

    loop {
    	print!("> ");
    	io::stdout().flush().unwrap();

    	let mut input = String::new();
    	io::stdin().read_line(&mut input).expect("no command given");

    	let command = parse_input(&input);

    	match command {
    		Command::Add(name, dept) => add_person(&mut directory, name, dept),
    		Command::List(dept) => list_dept(&mut directory, dept),
    		Command::Error(msg) => println!("error: {}", msg),
    		Command::Help => give_help(),
    		Command::Exit => break
    	}
    }
}

fn parse_input(input: &String) -> Command {
	let lower = input.to_lowercase();
	let split_input = lower.split_whitespace().collect::<Vec<&str>>();

	let command: Command;

	match split_input.get(0) {
		Some(&"add") => {
			if split_input.get(2) == Some(&"to") && split_input.len() == 4 {
				let name = split_input.get(1).unwrap();
				let dept = split_input.get(3).unwrap();
				command = Command::Add(String::from(*name), String::from(*dept));
			}
			else {
				command = Command::Error(String::from("improper command syntax"));
			}
		},

		Some(&"list") => {
			if split_input.len() == 2 {
				let dept = split_input.get(1).unwrap();
				command = Command::List(String::from(*dept));
			}
			else {
				command = Command::Error(String::from("improper command syntax"));
			}
		},

		Some(&"help") => command = Command::Help,

		Some(&"exit") => command = Command::Exit,

		None => command = Command::Error(String::from("no command given")),

		_ => command = Command::Error(String::from("invalid command"))
	}
	command
}