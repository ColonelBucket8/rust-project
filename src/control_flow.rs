pub fn control_flow() {
    // If expressions
    let number = 3;

    // must provide bool
    // this is not javascript
    if number < 5 {
        println!("conditions was true");
    } else {
        println!("conditions was false");
    }

    // handling multiple conditions with else if
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is divisible by 4, 3, or 2");
    }

    // Using if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // Repeating with loops
    // Repeating code with loop
    loop {
        println!("again!");
        break;
    }

    // Returning values from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

// Multiple loops
pub fn nested_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {count}")
}

// Conditional loops with while
pub fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFTOFF!!!");

    // looping through a collection
    let a = [10, 20, 30, 40, 50];

    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }

    // using for
    for element in a {
        println!("using for: the value is: {element}")
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!");
}

// Fibonacci
// Recursion
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
