use std::io;

pub fn variables() {
    let mut x = 5;

    println!("The value of x is: {x}");

    x = 6;

    println!("The value of x is: {x}");

    // cant use mut on const as its immutable by default
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("The value of const is: {THREE_HOURS_IN_SECONDS}");

    // shadowing
    let x = 5;

    let x = x + 1;

    println!("The value of x is : {x}");

    let guess: u32 = "42".parse().expect("Not a number");

    println!("The value of guess is : {guess}");

    // Floating-point types

    // f64
    let x = 2.0;
    let y: f32 = 3.0;

    println!("The value of x and y are: {} and {}", x, y);

    // The tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructure tuple

    let (x, y, z) = tup;
    let five_hundred = tup.0;

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
    println!("The value of x is: {five_hundred}");

    // The array type
    // Array has fixed length and must have the same type
    let a: [u8; 5] = [1, 2, 3, 4, 5];

    let first = a[0];

    println!("The value of a is: {first}")
}

pub fn enter_array() {
    let a: [u8; 5] = [1, 2, 3, 4, 5];

    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
