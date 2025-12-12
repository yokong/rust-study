fn main() {
    let rect =   {
        width: 30,
        height: 50,
    };
    println!("Area: {}", area(&rect));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

struct Rectangle {
    width: u32,
    height: u32,
}
