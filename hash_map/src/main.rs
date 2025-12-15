use std::{collections::HashMap, vec};

fn main() {
    // 基础使用------------------
    //     // 创建HashMap , k、v的类型必须一样
    //     let mut scores = HashMap::new();
    //     scores.insert(String::from("Blue"), 10);
    //     scores.insert(String::from("Yellow"), 50);

    //     // let vec = vec![("key1", "value1"), ("key2", "value2")];
    //     // let map: HashMap<_, _> = vec.into_iter().collect();

    //     // 访问HashMap的值 使用get 方法
    //     // let team_name = String::from("Blue");
    //     // let score = scores.get(&team_name).copied().unwrap_or(0);

    //     // 遍历 K-V
    //     for (key, value) in &scores {
    //         println!("{}: {}", key, value);
    //     }

    // ----------------------------------所有权

    // let field_name = String::from("Efavior=te coordinate.");
    // let field_value = String::from("1234567890");
    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);

    // 更新hashMap
    let mut scores = HashMap::new();
    // 1.key存在，替换了
    scores.insert(String::from("Blue"), 20);
    // scores.insert(String::from("Blue"), 10);
    // 2. 获取给定键在映射中的对应条目以进行原地操作。 就是看这个key 在map中存不存在
    // scores.entry(String::from("Yellow")).or_insert(50); // 不存在才起作用，存在不操作
    // scores.entry(String::from("Blue")).or_insert(50);
    // println!("{scores:?}");

    // 3.基于Key 原有的值来进行更新
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count: &mut i32 = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");
}
