use std::rc::Rc;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List<T> {
	Cons(T, Rc<List<T>>),
	Nil
}

impl<T> List<T> {
	fn change_rest(&mut self, new_rest: List<T>) {
		if let Cons(_, rest) = self {
 			*Rc::get_mut(rest).unwrap() = new_rest;
		}
	}
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let mut b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
	{
    	let c = Cons(4, Rc::clone(&a));
    	println!("count after creating c = {}", Rc::strong_count(&a));
   	}
   	println!("count after c goes out of scope = {}", Rc::strong_count(&a));

   	// cannot do this! bc b has a reference to a: b.change_rest(Cons(4, Rc::new(Nil)));
   	println!("{:?}", b);
   	
   	let mut d = Cons(5, Rc::new(Nil));
   	d.change_rest(Cons(4, Rc::new(Nil)));
   	println!("{:?}", d);
}