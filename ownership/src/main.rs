fn main() {
    let first = String::from("Ferris");
    // 移动堆（heap）数据原则：
    // 如果变量x将堆（heap）数据的所有权移动给另一个变量y，那么在移动后，× 不能再使用。
    // let full = add_suffix(first);
    // println!("{full},originally {first}");//!  borrow of moved value: `first`
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
