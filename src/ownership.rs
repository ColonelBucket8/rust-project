pub fn ownership() {
    // String is immutable by default
    let s = "Hello";
    
    // Make it mutable
    let mut s = String::from("Hello");

    s.push_str(", world!");

    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s2);

    // Clone
    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("s1 = {}, s2 = {}", s3, s4);
}

// Return values and scope
pub fn ownership2() {
    let s1 = gives_ownership(); // gives_ownership moevs its return value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moves into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  //  happens, s1 goes out of scope and is dropped

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

pub fn tuple_ex() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
