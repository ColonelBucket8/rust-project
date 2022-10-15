pub fn the_slice_type() {
    let mut s = String::from("hello world");

    let word = first_word_v2(&s);

    // cannot borrow `s` as mutable because it is also borrowed as immutable mutable borrow occurs here
    s.clear();

    // println!("the first word is: {}", word);

    // String slice
    let str = String::from("hello world");

    let len = str.len();

    // same
    let hello = &str[0..5];
    let hello = &str[..5];

    // same
    let world = &str[6..11];
    let world = &str[6..len];

    // same
    let slice = &str[0..len];
    let slice = &str[..];

    let a = [1, 2, 3, 4, 5, 6];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

// Error prone
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_v2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
