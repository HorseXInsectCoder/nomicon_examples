struct A<'a> {
    _a: i32,        // 4 bytes
    _b: &'a [u8],   // 引用 16 bytes， &[T] 是一个胖指针，保存指向数据的地址和元素个数
}

trait MyTrait {
    fn test() {}
}

struct Foo {}

struct Bar {
    _foo: Foo,
    _qux: (),
    _bax: [u8; 0],
}

// 空类型
enum Void {}

mod test {
    use std::mem;
    use super::*;

    #[test]
    fn works() {
        let array: [u8; 10] = [1; 10];
        let s = &array[..];

        println!("s size = {}", mem::size_of_val(s));   // size = 10
        println!("s size = {}", mem::size_of_val(&s));  // 胖指针的长度, size = 16
        println!("&i32 size = {}", mem::size_of::<&i32>());  // 查看i32指针的长度： size = 8
        println!("&i64 size = {}", mem::size_of::<&i64>());  // 查看i64指针的长度： size = 8
        println!("i32 size = {}", mem::size_of::<i32>());  // 查看i32的长度： size = 4
        println!("i64 size = {}", mem::size_of::<i64>());  // 查看i64的长度： size = 8
        println!("128 size = {}", mem::size_of::<i128>());  // 查看i129的长度： size = 16

        println!("A size = {}", mem::size_of::<A>());         // size = 24，4 + 16，然后按8字节进行对齐。如果是&'a [u8; 0]，那么size = 4 + 对齐4 + 8 = 16
        println!("&A size = {}", mem::size_of::<&A>());       // size = 8

        // trait不能直接用这种方法打印长度
        // 但是trait对象的胖指针大小是2个指针，一个指向具体的数据结构（抽象的，需要具体的数据结构才能确实，与slice不一样），另一个指向虚表
        // println!("MyTrait size = {}", mem::size_of::<MyTrait>());

        println!("&[u8] size = {}", mem::size_of::<&[u8]>());   // size = 16

        // 零尺寸
        println!("Foo size = {}", mem::size_of::<Foo>());   // size = 0
        println!("Foo size = {}", mem::size_of::<Bar>());   // size = 0

        // 空类型
        println!("Void size = {}", mem::size_of::<Void>()); // size = 0
    }
}