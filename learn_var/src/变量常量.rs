fn main() {
    // let x = 5;
    // println!("the value of x is: {}", x);
    // //! cannot mutate immutable variable 'x'
    // // x = 6;

    // shadowing 遮蔽后实际上是一个新变量
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is :{x}");
    }
    println!("value of x is:{}", x)
    // The value of x in the inner scope is :12
    // value of x is:6
}
