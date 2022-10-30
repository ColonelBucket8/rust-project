use crate::smart_pointers::List::{Cons, Nil};
use crate::smart_pointers::ListV2::{ConsV2, NilV2};
use crate::smart_pointers::ListV3::{ConsV3, NilV3};
use std::cell::RefCell;
use std::rc::Rc;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum ListV2 {
    ConsV2(i32, Rc<ListV2>),
    NilV2,
}

#[derive(Debug)]
enum ListV3 {
    ConsV3(Rc<RefCell<i32>>, Rc<ListV3>),
    NilV3,
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: you've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
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

    // Having multiple owners of mutable data by combining Rc<T> and RefCell<T>
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(ConsV3(Rc::clone(&value), Rc::new(NilV3)));
    let b = ConsV3(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = ConsV3(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }

        // Runtime error
        // fn send(&self, message: &str) {
        //     let mut one_borrow = self.sent_messages.borrow_mut();
        //     let mut two_borrow = self.sent_messages.borrow_mut();

        //     one_borrow.push(String::from(message));
        //     two_borrow.push(String::from(message));
        // }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
