fn main() {
    // 1.循环集合里的元素
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("element = {}", element);
    }

    // 2. range 1..=10 =>  1-10包含10  1..10 => 1-9不包含10
    for number in 1..=10 {
        println!("number = {}", number);
    }
}
