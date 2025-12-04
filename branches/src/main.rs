fn main() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4 or 2");
    }

    let condation = true;
    // if是一个表达式，它返回一个值
    let number = if condation { 6 } else { 7 };
    println!("number = {}", number);
}
