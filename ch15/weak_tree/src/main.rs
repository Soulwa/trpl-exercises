use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
	value: i32,
	parent: RefCell<Weak<Node>>,
	children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
	let leaf = Rc::new(Node {
		value: 3, 
		parent: RefCell::new(Weak::new()),
		children: RefCell::new(vec![]),
	});

	println!("leaf strong = {}, weak = {}", 
		Rc::strong_count(&leaf),
		Rc::weak_count(&leaf)
	);

	{
		let branch = Rc::new(Node {
			value: 5,
			parent: RefCell::new(Weak::new()),
			children: RefCell::new(vec![Rc::clone(&leaf)]),
		});

		*leaf.parent.borrow_mut() = Rc::downgrade(&branch);

		println!("branch strong = {}, weak = {}", 
			Rc::strong_count(&branch),
			Rc::weak_count(&branch)
		);

		println!("leaf strong = {}, weak = {}", 
			Rc::strong_count(&leaf),
			Rc::weak_count(&leaf)
		);
	}

	// trying to upgrade the weak ptr here will give a None, since
	// the data the weak ptr has a reference to is dropped (the branch)
	// or, scopes: like it never had a reference?
	println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());

	println!("leaf strong = {}, weak = {}", 
		Rc::strong_count(&leaf),
		Rc::weak_count(&leaf)
	);
}
