use std::io;

fn main() {
    println!("what fibonacci number should i count to?");

    let n: u32 = loop {
    	let mut input = String::new();

    	io::stdin().read_line(&mut input)
    		.expect("failed to read line");

    	match input.trim().parse() {
    		Ok(num) => break num,
    		Err(_) => continue
    	}
    };

    for i in 0..n  {
    	println!("{}", fib(i));
    }
}

fn fib(n: u32) -> u32 {
	if n == 0 || n == 1 {
		n 
	}
	else {
		fib(n-1) + fib(n-2)
	}
}