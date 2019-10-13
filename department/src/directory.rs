use std::collections::HashMap;

pub fn add_person(directory: &mut HashMap<String, Vec<String>>,
				  name: String, dept: String) {
	println!("adding {} to {} department", name, dept);
	{
		let current_dept = directory.entry(dept).or_insert(Vec::new());
		if current_dept.contains(&name) {
			println!("{} is already in this department!", name);
		}
		else {
			current_dept.push(name);
		}
	}
}

pub fn list_dept(directory: &mut HashMap<String, Vec<String>>, dept: String) {
	if directory.contains_key(&dept) {
		let dept_people = directory.get(&dept).unwrap();
		for person in dept_people {
			println!("{}", person);
		}
	}
	else {
		println!("that directory doesn't exist!!");
	}
}

pub fn give_help() {
	println!("Commands:");
	println!("add <name> to <department>    Adds employee <name>to <department>.");
	println!("list <department>             Gives a list of all employees in <department>.");
	println!("exit                          Stops the program.");
	println!("help                          Prints this message.");
}