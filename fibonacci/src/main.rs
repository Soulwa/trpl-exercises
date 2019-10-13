use std::io;

fn main() {
	let mut num = String::new();

    println!("input what fibonacci number you would like to find to: ");

    io::stdin().read_line(&mut num)
    .expect("Failed to read line");

    let num: u32 = match num.trim().parse() {
    	Ok(num) => num,
    	Err(_)  => panic!("input not of type u32: {}", num),
    };
    println!("Your number: {}", fib(num));
}

fn fib(n: u32) -> u32 {
	if n == 0 || n == 1 {
		n
	} else {
		fib(n-1) + fib(n-2)
	}
}
