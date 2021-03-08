mod front_of_house;

mod back_of_house {

	pub enum Appetizer {
		Soup,
		Salad,
	}

	pub struct Breakfast {
		pub toast: String,
		seasonal_fruit: String
	}

	impl Breakfast {
		pub fn summer(toast: &str) -> Breakfast {
			Breakfast {
				toast: String::from(toast),
				seasonal_fruit: String::from("peaches")
			}
		}
	}

	fn fix_incorrect_order() { 
		cook_order();
		super::serve_order();
	}

	fn cook_order() { }
}

fn serve_order() { }

// bring a crate into scope with use keyword

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {

	// can call from hosting thanks to the use above
	hosting::add_to_waitlist();
	hosting::add_to_waitlist();
	hosting::add_to_waitlist();
	
	// order a meal in summer w rye toast
	let mut meal = back_of_house::Breakfast::summer("rye");

	// can change mind about bread
	meal.toast = String::from("wheat");
	println!("I'd like {} toast please", meal.toast);

	// can't see or modify the seasonal fruit with the meal
	// meal.seasonal_fruit = String::from("blueberries")

	// all enum variants are public for a pub enum
	let _order1 = back_of_house::Appetizer::Soup;
	let _order2 = back_of_house::Appetizer::Salad;

}