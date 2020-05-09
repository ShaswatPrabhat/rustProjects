#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        println!("Rectangle info {:#?}", self);
        self.height * self.width
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let rect1 = Rectangle {
        height: 23,
        width: 31,
    };

    let square1 = Rectangle::square(31);
    println!(
        "Area of rectangle with width {} and height {} is {}",
        rect1.width,
        rect1.height,
        rect1.area()
    );
    println!(
        "Area of rectangle with width {} and height {} is {}",
        square1.width,
        square1.height,
        square1.area()
    );
}
