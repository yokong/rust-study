fn main() {
    let value = return_a_string();
    println!("{}", value);
}

// Fixing an Unsafe program: Returning a Reference to the stack
// fn return_a_string() -> &String {
//     let s = String::from("hello world");
//     &s
// }
// fn return_a_string() -> String {
//     let s = String::from("hello world");
//     s
// }

fn return_a_string() -> &'static str {
    "hello world" // 活到程序停止，
}
