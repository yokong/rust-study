use core::panic;
use std::{fs::File, io::ErrorKind};

fn main() {
    // panic!("crash and burn");
    // let v = vec![2, 3, 4];
    // v[99];

    let greeting_file_result = File::open("hello.txt");
    // 成功： 返回 Ok<T> T: std::fs::File
    // 失败： 返回 Err<E> E: std::io::Error
    println!("{greeting_file_result:?}");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // };
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(e) => {
                    panic!("Problem creating the file: {:?}", e);
                }
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
