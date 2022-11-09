mod advanced_traits;
mod closures;
mod common_collections;
mod control_flow;
mod enum_and_pattern_matching;
mod error_handling;
mod functions;
mod generic_types;
mod guessing_game;
mod lifetimes;
mod multi_thread;
mod oop;
mod oop_pattern;
mod ownership;
mod patterns_and_matching;
mod references_and_borrowing;
mod smart_pointers;
mod structs;
mod the_slice_type;
mod traits;
mod unsafe_rust;
mod variables;

use advanced_traits::advanced_traits;
use closures::{closures, iterator};
use control_flow::{control_flow, fibonacci, nested_loops, while_loop};
use enum_and_pattern_matching::enumeration;
use functions::{another_function_params, expression, print_labeled_measurement};
use guessing_game::guessing_game;
use lifetimes::lifetimes;
use multi_thread::multi_thread;
use oop::oop;
use oop_pattern::oop_pattern;
use ownership::{ownership, ownership2, tuple_ex};
use patterns_and_matching::patterns_and_matching;
use references_and_borrowing::references;
use smart_pointers::smart_pointers;
use structs::{defining_structs, rectangles};
use the_slice_type::the_slice_type;
use traits::traits_ex;
use unsafe_rust::unsafe_rust;
use variables::{enter_array, variables};

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
    multi_thread();
    oop();
    oop_pattern();
    patterns_and_matching();
    unsafe_rust();
    advanced_traits();
}

fn another_function() {
    println!("Another function");
}
