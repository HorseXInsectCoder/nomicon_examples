
struct A {
    _a: u8,
    _b: i32,
    _c: u8,
}

#[repr(C)]
struct B {
    _a: u8,
    _b: i32,
    _c: u8,
}

// 无成员枚举类型
enum MyEnum {
    First,
    Second,
}

// Rust一般的枚举，这种在C语言是没有对应的
enum RustEnum {
    First(String),
    Second(i32),
}

mod test {
    use std::mem;
    use super::*;

    #[test]
    fn repr_works() {
        // 报错 expected `i32`, found `MyEnum`，但在C/C++中可以这样操作
        // let a = MyEnum::First;
        // let mut b = 0;
        // b = a;

        println!("A size = {}", mem::size_of::<A>());       // 8
        println!("B size = {}", mem::size_of::<B>());       // 12
    }
}