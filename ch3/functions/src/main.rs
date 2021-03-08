fn main() {
    println!("Hello, world!");

    // let x = (let y = 6);
    // doesn't compile as assignment doesn't return a value
    // statement, not expression
    // cannot do x = y = 6
    let x = plus_one(4);

    let y = {
    	let x = 3;
    	x + 1 // expressions do not end with semicolons, adding one makes it a statement which doesn't return
    };

    another_function(x, y);
}

fn another_function(x: i32, y: i32) {
	println!("value of x is {}, y is {}", x, y);
}

fn four() -> i32 {
	4 // implicitly returns last expression
}

fn plus_one(x: i32) -> i32 {
	x + 1
}