fn main() {
    print_labeled_measurement(100, 'C');

    println!("plus_one(100) = {}", plus_one(100));
}

// 必须声明每个参数的类型参数名：类型
// 多个参数需使用“，“分开

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {} {}", value, unit_label);
}

// 函数里最后一个表达式的值是 函数的返回值 or use 'return'
fn plus_one(x: i32) -> i32 {
    // 表达式的值也是返回值
    x + 1
}
