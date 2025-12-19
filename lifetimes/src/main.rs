// fn main() {
//     let r;
//     {
//         let x = 5;
//         r = &x; // borrowed value does not live long enough
//     }
//     // - `x` dropped here while still borrowed x所在内存被释放了 所以引用不到
//     // println!("r: {}", r); // error: use of dropped value: `x`
// }

// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest(string1.as_str(), string2);
//     println!("result: {}", result);
// }

// // 泛型生命周期
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() { x } else { y }
// }

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
