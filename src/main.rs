mod closures;
mod common_collections;
mod control_flow;
mod enum_and_pattern_matching;
mod error_handling;
mod functions;
mod generic_types;
mod guessing_game;
mod lifetimes;
mod ownership;
mod references_and_borrowing;
mod structs;
mod the_slice_type;
mod traits;
mod variables;
mod smart_pointers;

use closures::{closures, iterator};
use control_flow::{control_flow, fibonacci, nested_loops, while_loop};
use enum_and_pattern_matching::enumeration;
use functions::{another_function_params, expression, print_labeled_measurement};
use guessing_game::guessing_game;
use lifetimes::lifetimes;
use ownership::{ownership, ownership2, tuple_ex};
use references_and_borrowing::references;
use structs::{defining_structs, rectangles};
use the_slice_type::the_slice_type;
use traits::traits_ex;
use variables::{enter_array, variables};
use smart_pointers::smart_pointers;

use crate::{
    common_collections::{hash_map, store_string, vectors},
    error_handling::error_handling,
    generic_types::generic_types,
};

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
    the_slice_type();
    defining_structs();
    rectangles();
    enumeration();
    vectors();
    store_string();
    hash_map();
    error_handling();
    generic_types();
    traits_ex();
    lifetimes();
    closures();
    iterator();
    smart_pointers();
}

fn another_function() {
    println!("Another function");
}
