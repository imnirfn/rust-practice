// program that calculates the area of a rectangle

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50
    };

    println!("Area of the rectangle is : {}", area(&rect));
    println!("{:#?}", rect);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
