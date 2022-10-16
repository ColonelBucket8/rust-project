#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn vectors() {
    let v: Vec<i32> = Vec::new();

    let v1 = vec![1, 2, 3];

    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);
    v2.push(9);

    println!("The value of vector v2 is {:?}", v2);

    let v4 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v4[2];
    println!("The third element is {}", third);

    // get returns Option<&T>
    let third: Option<&i32> = v4.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // Iterating over the values in a vector

    let v5 = vec![100, 32, 57];

    for i in &v5 {
        println!("{}", i);
    }

    let mut v6 = vec![100, 32, 57];
    for i in &mut v6 {
        *i += 50;
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("the spreadsheetcell vector is {:?}", row);
}

pub fn store_string() {
    let data = "initial contents";

    let s = data.to_string();

    let hello = String::from("こんにちは");

    println!("strings {} {}", s, hello);

    let mut str = String::from("foo");

    str.push_str("bar");

    let mut s2 = String::from("lo");

    s2.push('l');

    // Concatenation
    let s3 = String::from("Hello, ");
    let s4 = String::from("world!");

    // fn add(self, s: &str) -> String
    let s5 = s3 + &s4;

    println!("The string is {}", s5);

    let s6 = String::from("tic");
    let s7 = String::from("tac");
    let s8 = String::from("toc");

    let tic = format!("{}-{}-{}", s6, s7, s8);
    println!("{}", tic);

    // Rust doesn't have strings indexing
    // Methods for iterating over strings

    for c in "hello".chars() {
        println!("{}", c);
    }

    for b in "hello".bytes() {
        println!("{}", b);
    }
}
