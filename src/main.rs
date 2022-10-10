mod functions;
mod guessing_game;
mod variables;

use crate::functions::another_function_params;
use crate::functions::print_labeled_measurement;
use crate::functions::expression;
use crate::guessing_game::guessing_game;
use crate::variables::enter_array;
use crate::variables::variables;

fn main() {
    // guessing_game();
    variables();
    // enter_array();
    another_function();
    another_function_params(5);
    print_labeled_measurement(123, 't');
    expression();
}

fn another_function() {
    println!("Another function");
}
