pub fn abc() {
    let mut s = String::new();
    let data = "initial contents";

    let s = data.to_string();

    // 该方法也可直接用于字符串字面量：
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    //push_str不获取所有权

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // format! 宏不会获取所有权
    let s = format!("{}-{}-{}", s1, s2, s3);


    let hello = "Здравствуйте";
    //因为是utf-8所以s会出现一个字符不能用一个字节表示的情况
    let s = &hello[0..4];

    //chars()会返回string中的字符迭代器
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    //bytes()会返回string中的字节迭代器
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}