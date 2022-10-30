enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::smart_pointers::List::{Cons, Nil};

pub fn smart_pointers() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
