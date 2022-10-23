use std::cmp::PartialOrd;

struct Point<T> {
    x: T,
    y: T,
}

struct PointV2<T, U> {
    x: T,
    y: U,
}

struct PointV3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<X1, Y1> PointV3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointV3<X2, Y2>) -> PointV3<X1, Y2> {
        PointV3 {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn generic_types() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number in {}", largest);

    let largest = get_largest(&number_list);

    println!("The largest number in {} usig fn largest", largest);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let integer_and_float = PointV2 { x: 5, y: 4.0 };

    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x);

    let p1 = PointV3 { x: 5, y: 10.4 };
    let p2 = PointV3 { x: "hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let largest_number = get_largest_v2(&number_list);
    println!("largest number {}", largest_number);
}

fn get_largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

fn get_largest_v2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

