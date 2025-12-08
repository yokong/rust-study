// fn main() {
//     println!("Hello, world!");
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

// fn main() {
//     let s = String::from("hello");

//     let slice = &s[0..2]; // [0,2)
//     let slice = &s[..2]; // [0,2)

//     let len = s.len();

//     let slice = &s[3..len];
//     let slice = &s[3..];

//     let slice = &s[0..len];
//     let slice = &s[..];
// }

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
