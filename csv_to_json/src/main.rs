fn main() {
    // inline,mod xxx{}
    // crate:: crate 可以忽略 - 绝对路径引入
    let y = m1::m2::method1();
}
mod m1 {
    pub mod m2 {
        pub fn method1() {
            // self::x
        }
    }
}

mod x1 {
    fn method3() {
        self::x2::method2();
    }
    mod x2 {
        pub fn method2() {
            // 相对路径
            super::super::m1::m2::method1();
        }
    }
}
