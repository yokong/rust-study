use std::{cmp::Ordering, io};

//  prelude

// 1 -- 100 整数， rand 第三方库(crate)
fn main() {
    // 宏(macro)
    println!("Guess the number!");
    println!("Please input your guess.");

    // 生成随机数
    // 1..=100 闭区间，包含 100
    // 1..101 开区间，不包含 101
    // ! random@0.9X会有 error[E0599]: no method named `gen_range` found for struct `ThreadRng` in the current scope
    // let secret_number = rand::thread_rng().gen_range(1..101);
    let secret_number = rand::random_range(1..=100);
    println!("The secret number is: {}", secret_number);
    loop {
        // mut 可变变量
        let mut guess = String::new();

        io::stdin()
            // Return Result (Ok-usize, Err)
            .read_line(&mut guess)
            // 处理 Result Err
            .expect("Failed to read line");

        // {} 占位符
        println!("You guess: {}", guess);

        // parse 方法，将字符串转换为 u32 类型
        // shadowing(遮蔽) 变量 guess 被重新绑定为 u32 类型
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };
        // match 表达式
        // cmp 方法，比较两个值的大小，返回 Ordering 枚举值
        // Ordering 枚举值：Less, Greater, Equal
        // &secret_number 是一个引用，指向 secret_number 变量
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
