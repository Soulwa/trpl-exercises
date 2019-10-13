use std::collections::HashMap;

enum ModeType {
	One(i32),
	Many(Vec<i32>),
	NoMode(String)
}

fn main() {
	let mut mylst = vec![1, 1, 1, 2, 5, 4, 6, 18, 15];
	println!("list: {:?}", mylst);
	{
		let mut sorted = mylst.clone();
		sorted.sort();
		println!("sorted list: {:?}", sorted);
	}
	println!("mean: {:.3}", find_mean(&mylst));
	println!("median: {}", find_median(&mut mylst));
	match find_mode(&mylst) {
		ModeType::One(mode) => println!("mode: {}", mode),
		ModeType::Many(mode) => println!("mode: {:?}", mode),
		ModeType::NoMode(mode) => println!("mode: {}", mode),
	}
}

fn find_mean(lst: &Vec<i32>) -> f64 {
	lst.iter().sum::<i32>() as f64 / lst.len() as f64
}

fn find_median(lst: &mut Vec<i32>) -> f64 {
	lst.sort();
	let len = lst.len();
	if len % 2 == 0 {
		(lst[len / 2] + lst[len / 2 + 1]) as f64 / 2.0
	}
	else {
		lst[len / 2] as f64
	}
}

fn find_mode(lst: &Vec<i32>) -> ModeType {
	let mut vec_map: HashMap<i32, i32> = HashMap::new();

	for &i in lst {
		*vec_map.entry(i).or_insert(0) += 1;
	}

	let mut modes: Vec<i32> = Vec::new();
	let count = *vec_map.values().max().unwrap();

	for (&key, &val) in vec_map.iter() {
		if val == count {
			modes.push(key);
		}
	}
	modes.sort();

	match modes {
		ref modes if modes == lst  => ModeType::NoMode(String::from("no mode")),
		ref modes if modes.len() == 1 => ModeType::One(modes[0]),
		_ => ModeType::Many(modes)
	}
}