// 按b的大小对齐
struct A {
    a: u8,      // 1 byte
    // padding     3 byte

    b: u32,     // 4 byte

    c: u16,     // 2 byte
    // padding     2 byte
}

// 编译器的可能优化
struct ARepr {
    a: u8,      // 1 byte
    c: u16,     // 2 byte
    // padding     1 byte
    b: u32      // 4 byte
}

struct B {
    a: u32,
    b: u32,
    c: u32,
}


enum Foo {
    X(u32),
    Y(u64),
    Z(u8),
}

// 枚举类型的表示
struct FooRepr {
    data: u64,      // 根据tag的不同，这一项可以为u64, u32, u8，类型按最大的值来分配
    tag: u8,        // 0 = A, 1 = B, 2 = C
}


mod test {
    use std::mem;
    use super::*;

    #[test]
    fn alignment_works() {
        let aa = A {
            a: 1,
            b: 2,
            c: 3,
        };
        println!("size = {}", mem::size_of::<A>());             // 8
        println!("size = {}", mem::size_of::<B>());             // 12
        println!("size = {}", mem::size_of_val(&aa));       // 8
    }
}