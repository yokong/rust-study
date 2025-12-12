#[derive(Debug)] // 实现 derrive 这个
struct Rectangle {
    w: u32,
    h: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.w * self.h
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.w > other.w && self.h > other.h
    }

    fn square(size: u32) -> Self {
        Self {
            w: size,
            height: size,
        }
    }
}
fn main() {
    let rect1 = Rectangle { w: 10, h: 10 };

    println!("rect1 is {rect1:#?}");
}
