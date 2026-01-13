#[derive(Debug)]
struct Rectangle {
    x: u32,
    y: u32
}

fn main() {
    let rect1 = Rectangle {
        x: 2,
        y: 4
    };
    println!("rect1 is {rect1:?}");

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.x * rectangle.y
}