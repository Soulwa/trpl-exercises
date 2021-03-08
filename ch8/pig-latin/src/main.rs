use std::io;

fn main() {
	loop {
		println!("input your word:");
		let mut word = String::new();
		io::stdin().read_line(&mut word).expect("no word given");

		pig_latinify(&word);
	}
}

fn pig_latinify(word: &String) -> String {
	let mut pig_word = format!("{}-ay", word.trim_right());
	for (i, c) in word.chars().enumerate() {
		match c {
			'a' | 'e' | 'i' | 'o' | 'u' | 'y'  => {
				if i > 0 {
					let (ending, remainder) = word.split_at(i);
					pig_word = format!("{}-{}ay", remainder.trim_right(), ending.trim_right());
				}
				else {
					pig_word = format!("{}-hay", word.trim_right());
				}
				break;
			},
			_ => continue,
		}
	}
	println!("{}", pig_word);
	pig_word
}