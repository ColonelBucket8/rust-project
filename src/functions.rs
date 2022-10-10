// Function with param
pub fn another_function_params(x: i32) {
    println!("The value of x is: {x}")
}

// Multiple params
pub fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

// Expression
pub fn expression() {
    let y = {
        let x = 3;
        // Expression doesnt include ending semiconlons
        // semicolon turns it into statement
        x + 1
    };

    println!("The value of y is: {y}");
}

// Return values
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
