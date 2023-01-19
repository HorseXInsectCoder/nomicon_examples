struct A<'a> {
    _a: i32,
    _b: &'a [u8],
}

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
    }
}