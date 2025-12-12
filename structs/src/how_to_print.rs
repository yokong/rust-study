#[derive(Debug)] // 实现 derrive 这个
struct Rectangle {
    w: u32,
    h: u32,
}
fn main() {
    let rect1 = Rectangle { w: 10, h: 10 };

    println!("rect1 is {rect1:#?}");
}
