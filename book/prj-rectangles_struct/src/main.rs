// program that calculates the area of a rectangle

#![allow(unused_variables)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        (self.width + self.height) * 2
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50
    };

    let rect2 = Rectangle {
        width: 20,
        height: 30
    };

    println!("Area of rect: {}", rect.area());
    println!("Perimeter of rect: {}", rect.perimeter());
    println!("Can rect hold rect2?: {}", rect.can_hold(&rect2));
}

