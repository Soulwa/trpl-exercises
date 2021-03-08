fn main() {
    let mut counter = 0;

    let result = loop {
    	counter += 1;

    	if counter == 10 {
    		break counter * 2;
    	}
    };

    println!("result is {}", result);

    let mut number = 3;

    while number != 0 {
    	println!("{}", number);
    	number -= 1;
    }

    println!("liftoff!");

    let a = [10, 20, 30, 40, 50];

    //less preferred way to iterate over collection, might go out of bounds, could panic depending on collection, overhead
    let mut index = 0;

    while index < 5 {
    	println!("value of element is {}", a[index]);

    	index += 1;
    }

    //cleaner, safer
    for elem in a.iter() {
    	println!("value is {}", elem);
    }

    //can even do liftoff cleaner, with range
    for i in (1..4).rev() {
    	println!("{}", i);
    }
    println!("liftoff!");

}

fn loop_examples() {

	//prints "again!" forever in terminal, can't put in main or won't show other examples
	loop {
    	println!("again!");
    }
}
