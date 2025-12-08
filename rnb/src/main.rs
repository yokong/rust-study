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

// fn main() {
//     let m1 = String::from("Hello");
//     let m2 = String::from("World");
//     println!("m1: {}", m1);
//     println!("m1的地址: {:p}", &m1); // 对m1的引用
//     greet(&m1, &m2);
//     let s = format!("{} {}", m1, m2);

//     fn greet(g1: &String, g2: &String) {
//         println!("{} {}!", g1, g2);
//         let address_in_g1 = g1 as *const String;
//         println!("{g1}");
//         println!("g1存的内容 {:p}", address_in_g1); // 存的 &m1
//         println!("g1的地址 {:p}", &g1);
//     }
// }

// fn main() {
//     let mut v: Vec<i32> = vec![1, 2, 3];
//     let num: &i32 = &v[0];
//     println!("{num}");
//     v.push(4);
//     println!("Third element is {}", *num);
// }

fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    // v = [1, 2, 3]，是一个可变向量
    //         ^ 这里是索引 2，值是 3
    let num: &mut i32 = &mut v[2];
    // &mut v[2] 的意思是：借用向量 v 的第 2 个元素（也就是 3），并且是 可变借用
    // 所以 num 是一个 “指向 v 里面那个 3 的可变引用”
    // 此时 v 已经被可变借用了，在 num 活着的期间，你不能再对 v 做其他借用（除了极少数特殊情况）
    let num2: &i32 = &*num;
    println!("{} {}", *num, *num2);
}
