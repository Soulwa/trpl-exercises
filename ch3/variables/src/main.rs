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

    let decimal: i32 = 42;
    let octal: i32 = 0o5;
    let hex: i32 = 0x01ff;
    let binary: i32 = 0b00101;
    let byte: u8 = b'F';

    let floaty: f32 = 7.0;
    let boolean: bool = true;

    let quotient = decimal / octal;
    //can't convert between floats/ints in division, types must match unlike c

    let tup = (500, 9.6, 7);
    let (x, y, z) = tup;
    println!("value of y is {}", y);

    let a = [0; 10];
    println!("value of a[0] is {}", a[0]);
}
