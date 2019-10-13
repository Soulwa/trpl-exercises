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
		self.width > other.width && self.height > other.height
	}

	fn square(size: u32) -> Rectangle {
		Rectangle { width: size, height: size }
	}
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    let sq = Rectangle::square(3);

    println!("{:?}", sq	);
    println!("The area of the rectangle is {} square pixels.", rect.area());
    println!("Rect can hold rect2: {}", rect.can_hold(&rect2));
    println!("Rect can hold rect3: {}", rect.can_hold(&rect3));
}