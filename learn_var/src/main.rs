fn main() {
    // Compound Type  复杂类型
    // Tuple 元组 可以有不同类型
    let tup = (100, 'a', false);
    let tup: (i32, char, bool, u8, f32) = (100, 'a', false, 255, 3.14);
    print!("tup.0: {}", tup.0);

    let arr = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 3, 4, 5, 6];
    let c = [3; 5]; // 五个元素都是3

    println!("arr[0] {}", arr[0]);
}
