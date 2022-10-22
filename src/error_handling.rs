use std::fs::File;
use std::io::{self, ErrorKind, Read};

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

pub fn error_handling() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("Problem creating the file {:?}", error),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    println!("File open {:?}", greeting_file);

    // shortcurts for panic on error

    let greeting_file_2 = File::open("hello.txt").unwrap();

    // most Rustaceans choose expect rather than unwrap and give more context
    // about why the operation is expected to always succeed
    let greeting_file_3 =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    let from_username = read_username_from_file("test.txt");

    println!("read username from file {:?}", from_username);

    let from_username_2 = read_username_from_file_v2();

    println!("read username from file v2 {:?}", from_username_2);

    let from_username_3 = read_username_from_file_v3();

    println!("read username from file v3 {:?}", from_username_3);

    let guess_number = Guess::new(10);

    println!("the number of guess is {}", guess_number.value);
}

// Using closures and if else
fn open_file() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    println!("greeting file {:?}", greeting_file);
}

fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
    let username_file_result = File::open(file_name);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error),
    }
}

fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut username_file = File::open("ihello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_v3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)

    // shorter way
    // fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
