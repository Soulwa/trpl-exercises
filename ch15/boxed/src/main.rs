use std::ops::Deref;

enum List<T> {
	Cons(T, Box<List<T>>),
	Nil,
}

use crate::List::{Cons, Nil};

fn main() {
	let lst: List<i32> = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

	let x = 5;
	let y = &5;
	let z = Box::new(5);

	assert_eq!(x, 5);
	assert_eq!(*y, 5);
	assert_eq!(*z, 5);

	let w = MyBox::new(5);

	assert_eq!(*w, 5);

	let m = MyBox::new("Rust");
	let m2 = MyBox::new(String::from("Rust"));

	hello(&m);
	hello(&m2);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
	fn new(x: T) -> MyBox<T> {
		MyBox(x)
	}
}

impl<T> Deref for MyBox<T> {
	type Target = T;

	fn deref(&self) -> &T {
		&self.0
	}
}

fn hello(name: &str) {
	println!("Hello, {}!", name);
}