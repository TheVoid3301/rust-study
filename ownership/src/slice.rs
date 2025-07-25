fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    //slice是string中一部分值的引用,相当于一个指针指向从中间开始的源string数据地址
    fn main() {
        let s = String::from("hello world");
    
        let hello = &s[0..5];//hello
        let world = &s[6..11];//world
    }
    
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
    //当拥有某值的不可变引用时，就不能再获取一个可变引用。
    //因为 clear 需要清空 String，它尝试获取一个可变引用。
    //在调用 clear 之后的 println! 使用了 word 中的引用，
    //所以这个不可变的引用在此时必须仍然有效。
    //Rust 不允许 clear 中的可变引用和 word 中的不可变引用同时存在，
    //因此编译失败。Rust 不仅使得我们的 API 简单易用，
    //也在编译时就消除了一整类的错误！

    //slice的类型是str,是不可变长的,&str是引用切片
}
