use crate::smart_pointers::List::{Cons, Nil};
use crate::smart_pointers::ListV2::{ConsV2, NilV2};
use std::rc::Rc;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum ListV2 {
    ConsV2(i32, Rc<ListV2>),
    NilV2,
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn smart_pointers() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // Rust automatically called drop when instances went out of scope
    // Variable are dropped in the reverse order of their creation
    // so d was dropped before c
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created.");

    // Dropping a value early with std::mem::drop

    let e = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(e);
    println!("CustomSmartPointer dropped before the end of main.");

    let x = Rc::new(ConsV2(5, Rc::new(ConsV2(10, Rc::new(NilV2)))));
    println!("count after creating x = {}", Rc::strong_count(&x));
    let y = ConsV2(3, Rc::clone(&x));
    println!("count after creating y = {}", Rc::strong_count(&x));
    {
        let z = ConsV2(4, Rc::clone(&x));
        println!("count after creating z = {}", Rc::strong_count(&x));
    }
    println!("count after z goes out of scope = {}", Rc::strong_count(&x));
}
