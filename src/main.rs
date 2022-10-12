mod control_flow;
mod functions;
mod guessing_game;
mod ownership;
mod variables;
mod references_and_borrowing;

use control_flow::control_flow;
use control_flow::fibonacci;
use control_flow::nested_loops;
use control_flow::while_loop;
use ownership::{ownership, ownership2, tuple_ex};

use crate::functions::another_function_params;
use crate::functions::expression;
use crate::functions::print_labeled_measurement;
use crate::guessing_game::guessing_game;
use crate::references_and_borrowing::references;
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
    control_flow();
    nested_loops();
    while_loop();
    let x = fibonacci(5);
    println!("{x}");
    ownership();
    ownership2();
    tuple_ex();
    references();
}

fn another_function() {
    println!("Another function");
}
