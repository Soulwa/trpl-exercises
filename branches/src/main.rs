fn main() {
    let number = 6;

    if number < 5 {
    	println!("true- less than 5");
    } 
    else {
    	println!("false- not less than 5");
    }

    //cannot do if number { ... }, expression must be bool
    if number != 0 {
    	println!("number not zero");
    }

    //match preferred to else if branches
    if number % 4 == 0 {
    	println!("number divisible by 4");
    }
    else if number % 3 == 0 {
    	println!("number divisible by 3");
    }
    else if number % 2 == 0 {
    	println!("number divisible by 2");
    }
    else {
    	println!("number not divisible by 4, 3, or 2");
    }

    println!("\n");

    let condition = true;
    let new_number = if condition {
    	5
    } else {
    	6
    };

    println!("the value of new_number is {}", new_number);
}
