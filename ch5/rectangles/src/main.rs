#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}

	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width >= other.width && self.height >= other.height
	}

	fn square(size: u32) -> Rectangle {
		Rectangle { width: size, height: size }
	}
}

fn main() {

	// declaring width, height as separate variables doesn't show the relation
	// having rect1 be a tuple doesn't show meaning of values
	// rect1 as a struct shows meaning of type as well as fields- most clear
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("The area of the rectangle is {} square pixels",
    		 rect1.area());

    println!("rect1 is {:?}", rect1);

    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);

    println!("sq is {:?}", sq);
}

// functions like this are better implemented as methods, since it only works for a specific struct
fn area(rect: &Rectangle) -> u32 {
	rect.width * rect.height
}
