use std::env;
use std::fs;
use std::error::Error;

pub struct Config {
	pub query: String,
	pub filename: String,
	pub case_sensitive: bool,
}

impl Config {
	pub fn new<I>(mut args: I) -> Result<Config, &'static str>
		where I: Iterator<Item = String> {
		args.next();

		let query = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a query string")
		};

		let filename = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a filename")
		};

		let case_sensitive = match args.next() {
			Some(arg) => match arg.as_str() {
				"--insensitive" => false,
				"--sensitive" => true,
				_ => true
			}
			None => env::var("CASE_INSENSITIVE").is_err()
		};

		Ok(Config { query, filename, case_sensitive })
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.filename)?;

	let results = if config.case_sensitive {
		search(&config.query, &contents)
	}
	else {
		search_case_insensitive(&config.query, &contents)
	};

	for line in results {
		println!("{}", line);
	}

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	contents
		.lines()
		.filter(|line| line.contains(query))
		.collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	contents
		.lines()
		.filter(|line| line.to_lowercase().contains(&query))
		.collect()
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[should_panic(expected = "Didn't get a query string")]
	fn invalid_config() {
		let args = vec![String::from("/path/to/file")];
		if let Err(e) = Config::new(args.into_iter()) {
			panic!("Error constructing: {}", e)
		}
	}

	#[test]
	fn valid_config() {
		let args: Vec<String> = vec![String::from("/path/to/file"), String::from("hello"), String::from("hello.txt")];
		if let Err(e) = Config::new(args.into_iter()) {
			panic!("Error constructing: {}", e);
		}
	}

	#[test]
	#[should_panic]
	fn run_invalid_file() {
		let config = Config { 
			query: String::from("hello"), 
			filename: String::from("text.txt"),
			case_sensitive: true
		};
		run(config).unwrap()
	}

	#[test]
	fn run_valid_file() {
		let config = Config { 
			query: String::from("hello"), 
			filename: String::from("poem.txt"),
			case_sensitive: true
		};
		run(config).unwrap()
	}

	#[test]
	fn case_sensitive() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

		assert_eq!(vec!["safe, fast, productive."], search(query, contents), "Failed with content {} and query {}", contents, query);
	}

	#[test]
	fn case_insensitive() {
		let query = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

		assert_eq!(
			vec!["Rust:", "Trust me."], 
			search_case_insensitive(query, contents)
		);
	}
}