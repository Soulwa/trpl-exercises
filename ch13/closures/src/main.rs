use std::thread;
use std::cmp::Eq;
use std::hash::Hash;
use std::collections::HashMap;
use std::time::Duration;

fn main() {
	let simulated_user_specified_value = 10;
	let simulated_random_number = 7;

	generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
	let mut expensive_result = Cacher::new(|num| {
		println!("calculating slowly...");
		thread::sleep(Duration::from_secs(2));
		num
	});

	if intensity < 25 {
		println!("Today, do {} pushups!", expensive_result.value(intensity));
		println!("Next, do {} sit-ups!", expensive_result.value(intensity));
	}
	else {
		if random_number == 3 {
			println!("Take a break today! Remember to stay hydrated!");
		}
		else {
			println!("Today, run for {} minutes!", expensive_result.value(intensity));
		}
	}
}

struct Cacher<T, A, R>
where T: Fn(A) -> R,
	  A: Eq + Hash + Copy,
	  R: Copy
{
	calculation: T,
	cache: HashMap<A, R>
}

impl<T, A, R> Cacher<T, A, R> 
where T: Fn(A) -> R,
	  A: Eq + Hash + Copy,
	  R: Copy
{
	fn new(calculation: T) -> Cacher<T, A, R> {
		Cacher {
			calculation,
			cache: HashMap::new(),
		}
	}

	fn value(&mut self, arg: A) -> R {
		match self.cache.get(&arg) {
			Some(val) => *val,
			None => {
				let val = (self.calculation)(arg);
				self.cache.insert(arg, val);
				val
			}
		}
	}

}

// works with hash map cacher implementation
#[test]
fn call_with_different_values() {
	let mut c = Cacher::new(|a| a);
	let _v1 = c.value(1);
	let v2 = c.value(2);

	assert_eq!(v2, 2);
}

// closures can capture their environment
#[test]
fn capture_environment() {
	let x = 4;

	// if we did this with a typical function definition, would not compile!
	let equal_to_x = |z| z == x;

	let y = 4;

	assert!(equal_to_x(y));
}

#[test]
fn move_into_closure() {
	let x = vec![1, 2, 3];

	let equal_to_x = move |z| z == x;
	// cannot use x after this point - it has been moved into the closure
	// println!("{}", x);

	let y = vec![1, 2, 3];

	assert!(equal_to_x(y));
}