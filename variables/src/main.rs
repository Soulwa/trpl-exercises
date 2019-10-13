fn main() {
	println!("Main function.");
	other_func(1);
}

fn other_func(x: u32) -> bool {
	println!("x = {}", x);
	true
}