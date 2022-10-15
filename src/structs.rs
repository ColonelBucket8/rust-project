struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Method
// Implementation
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // can give method name the same name as one of the struct's field
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // Associated function
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn defining_structs() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // the entire instance must be mutable
    // Rust doesnt allow us to mark only certain fields
    // as mutable
    let mut user2 = User {
        email: String::from("someone2@example.com"),
        username: String::from("someusername1234"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("anotheremail@example.com");

    // has different value for email but has the same values
    // for the username, active and sign_in_count fields from user1
    // the ..user1 must come last to specify that any remaining fields
    // should get values from the corresponding fields in user1
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let user4 = build_user(
        String::from("mygoogleemail@gmail.com"),
        String::from("myusername"),
    );

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    // shorthand
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

pub fn rectangles() {
    let scale = 2;
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: dbg!(10 * scale),
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // Associated function
    let square1 = Rectangle::square(30);

    println!("The area of rectangle is {}", area(&rect1));
    println!("react1 is {:?}", rect1);
    dbg!(&rect2);

    println!("The area of rectangle is {} square pixels.", rect1.area());
    println!("The rectangle has a nonzero width; it is {}", rect1.width());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("The area of square is {:?}", square1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
