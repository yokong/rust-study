fn main() {
    // let my_number: i32 = 1;
    // let eight_bit_number: i8 = 2;
    // let another_one: u8 = 128;

    // // Floating point
    // let my_floating_point: f64 = 3.0;
    // let other_floating_point: f32 = 4.0;
    // // f32 四字节
    // // f64(default) 八字节
    // // 都有符号(signed)

    // // Character
    // // 4 字节
    // // 表示一个Unicode 标量值(Unicode SScalar Value)
    // let some_char: char = 'a';

    let num_a = 45; // 默认i32
    let num_b: u32 = 46;
    let num_c: u8 = 255;
    let num_d: usize = 46; // TODO: usize?
    let num_e: i32 = 0xff; //
    let num_f: u8 = b'A'; // 字节 byte =>65

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z: char = 'A';
    let chinese_char: char = '中';
    let emoji_char: char = '😻';

    println!(
        "num_a:{}, num_b:{}, num_c:{}, num_d:{}, num_e:{}, num_f:{}",
        num_a, num_b, num_c, num_d, num_e, num_f
    );
    println!("x:{}, y:{}", x, y);
    println!("t:{}, f:{}", t, f);
    println!(
        "c:{}, z:{}, chinese_char:{}, emoji_char:{}",
        c, z, chinese_char, emoji_char
    );
}
