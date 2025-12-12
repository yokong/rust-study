fn main() {
    // let mut s = String::new();

    // let data = "initial content";
    // // 实现了 Display Trait 的类型，具体有 to_string() 方法
    // let s = data.to_string();
    // let s = String::from("initial content");

    // String 中添加数据
    // let mut s = String::from("foo");
    // let s2 = "bar";
    // s.push_str(s2);
    // println!("{s2}");

    // let mut s = String::from("lo");
    // s.push('l');
    // println!("s");

    // 字符串拼接
    // let s1 = String::from("Hello ");
    // let s2 = String::from("World!");
    // // s1 被移动，s2没有，后面还可以使用
    // //fn add (self, s: &str) - String {} 第一个参数 self 会被获得所有权，后续不可以使用了
    // let s3 = s1 + &s2;
    // println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3; // 比较繁琐，可以使用 format! 宏
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
}
