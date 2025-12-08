// fn main() {
//     let m1 = String::from("Hello");
//     let m2 = String::from("World");
//     // greet(m1, m2);
//     // let s = format!("{m1},{m2}"); // ! 移动后使用的值
//     let (m1_again, m2_again) = greet2(m1, m2);
//     let s = format!("{m1_again},{m2_again}");

// }

// // fn greet(g1: String, g2: String) {
// //     println!("{g1},{g2}");
// // }
// fn greet2(g1: String, g2: String) -> (String, String) {
//     println!("{g1},{g2}");
//     (g1, g2)
// }

// 引用 没有「所有权」的指针
// fn main() {
//     let m1 = String::from("Hello");
//     let m2 = String::from("World");
//     greet(&m1, &m2); // note the ampersands 创建了一个引用 1. g1->m1 ,2.m1->heap
//     let s = format!("{m1},{m2}");
// }

// fn greet(g1: &str, g2: &str) {
//     println!("{g1},{g2}");
// }

fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("World");
    println!("m1: {}", m1);
    println!("m1的地址: {:p}", &m1); // 对m1的引用
    greet(&m1, &m2);
    let s = format!("{} {}", m1, m2);

    fn greet(g1: &String, g2: &String) {
        println!("{} {}!", g1, g2);
        let address_in_g1 = g1 as *const String;
        println!("{g1}");
        println!("g1存的内容 {:p}", address_in_g1); // 存的 &m1
        println!("g1的地址 {:p}", &g1);
    }
}
