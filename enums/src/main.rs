// fn main() {
//     // println!("Hello, world!");
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
//     route(four);
//     route(six);
// }

// fn route(ip_type: IpAddrKind) {}
// enum IpAddrKind {
//     V4,
//     V6,
// }

// fn main() {
//     enum IpAddrKind {
//         V4(u8, u8, u8, u8),
//         V6(String),
//     }

//     let home = IpAddrKind::V4(127, 0, 0, 1);
//     let loopback = IpAddrKind::V6(String::from("::1"));
//     // ---------------------------------
//     // enum IpAddrKind {
//     //     V4,
//     //     V6,
//     // }

//     // struct IpAddr {
//     //     kind: IpAddrKind,
//     //     address: String,
//     // }

//     // let home = IpAddr {
//     //     kind: IpAddrKind::V4,
//     //     address: String::from("127.0.0.1"),
//     // };
//     // let loopback = IpAddr {
//     //     kind: IpAddrKind::V6,
//     //     address: String::from("::1"),
//     // };
// }

// fn main() {
//     let m = Message::Write(String::from("hello"));
//     m.call();
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call(&self) {
//         // ...
//     }
// }

// ----------------- OPTION ENUM --------------------------------
// enum Option<T> {
//     Some(T),
//     None,
// }

// fn main() {
//     let some_number = Some(5);
//     let some_string = Some("a string");

//     let absent_number: Option<i32> = None;
// }

// fn main() {
//     let x: i8 = 5;
//     let y: Option<i8> = Some(5); // y 可能是空的
//     let sum = x + y;//
// }

// --------------------------------------- match -------------------------------
// fn main() {
//     #[derive(Debug)]
//     enum UsState {
//         Alabama,
//         Alaska,
//     }

//     enum Coin {
//         Penny,
//         Nickel,
//         Dime,
//         Quarter(UsState),
//     }

//     fn value_in_cents(coin: Coin) -> u8 {
//         match coin {
//             Coin::Penny => {
//                 println!("penny");
//                 1
//             }
//             Coin::Nickel => 5,
//             Coin::Dime => 10,
//             Coin::Quarter(state) => {
//                 println!("State quarter from {:?}!", state);
//                 25
//             }
//         }
//     }
// }

fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
