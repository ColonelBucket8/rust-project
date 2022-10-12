pub fn references() {
    let s1 = String::from("hello");
    let mut s = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
    change(&mut s);

    // let mut s2 = String::from("hello");

    // cannot borrow s2 as mutable more than once at a time
    // let r1 = &mut s2;
    // let r2 = &mut s2;

    // println!("{}, {}", r1, r2)

    // cannot have a mutable reference while we have an immutable one to the same value
    // let mut s3 = String::from("hello");

    // let r1 = &s3;
    // let r2 = &s3;
    // let r3 = &mut s3;

    // println!("{}, {} and {}", r1, r2, r3)
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
// it refers to, it is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String { // dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String
    
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope and is dropped. Its memory goes away
// Danger!
