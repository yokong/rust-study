use std::vec;

fn main() {
    // let v: Vec<i32> = Vec::new();

    // let v = vec![1, 2, 3];

    // 添加 vec
    // let mut v = Vec::new();
    // v.push(1);

    // 使用
    // let v = vec![1, 2, 3, 4, 5];
    // let third: &i32 = &v[2];
    // println!("The third element is {}", third);

    // let third: Option<&i32> = v.get(2);
    // match third {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element."),
    // }

    // let v = vec![1, 2, 3, 4, 5];
    // let not_exist = &v[100];// panic

    // let not_exist = v.get(100); // 不会引起 Panic

    // 所有权
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0]; // 不可变引用
    // v.push(6);

    let v = vec![100, 32, 57];
    for n_ref in &v {
        let n_plus_one: i32 = *n_ref + 1;
        println!("n_plus_one:{n_plus_one}");
    }
}
