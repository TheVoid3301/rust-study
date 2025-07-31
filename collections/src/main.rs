mod String;

#![allow(unused)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];


fn main() {
    // let v: Vec<i32> = vec::new();
    // let v = vec![1, 2, 3, 4, 5];

    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    let third: &i32 = &v[2];

    //因为有一个不可变的引用所以在这个不可变引用成效的区域内不能插入元素
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i += 50;
    }

    
}
