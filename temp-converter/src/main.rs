fn main() {
    println!("Fahrenheit to Celsius:");

    let mut temp: f64 = -100.0;
    let step: f64 = 10.0;

    while temp <= 100.0 {
    	println!("{:>5} F {:>10.1} C", temp, fahr_to_cel(temp));
    	temp += step
    }
}

fn fahr_to_cel(fahrenheit: f64) -> f64 {
	(fahrenheit - 32.0) / 1.8
}