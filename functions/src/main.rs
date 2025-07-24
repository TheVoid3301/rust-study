fn main() {
    another_function(5);
    print_label_measurement(1, 'a');

    //y = 4
    //表达式的结尾没有分号。如果在表达式的末尾加上分号，那么它就转换为语句，而语句不会返回值
    let y = {
        let x = 3;
        x + 1
    };

    println!("y is :{}", y);

    let x = five();

    let x = plus_one(5);
}

//函数和变量名使用蛇形命名法,字母小写,用下划线分割
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_label_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

//带返回值的函数
fn five() -> i32 {
    5
}

//如果在x + 1后面加;
//那么默认返回单元类型(), 表示不返回值
//但是函数定义返回一个i32所以相矛盾会出现错误
fn plus_one(x: i32) -> i32 {
    x + 1
}