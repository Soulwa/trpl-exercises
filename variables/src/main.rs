fn main() {
    let mut x = 5;
    println!("value of x is {}", x);
    x = 6;
    println!("value of x is {}", x);

    //shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("value of y is {}", y);
}
