fn main() {
	const STEP: f32 = 20.0;
	let mut i = -100.0;

	println!("Fahrenheit To Celsius Table");

	while i <= 100.0 {
		println!("{0:<5} {1:>5.1}", i, fahr_to_cel(i));
		i += STEP;
	}

}

fn fahr_to_cel(f: f32) -> f32 {
	(f - 32.0) * (5.0/9.0)
}