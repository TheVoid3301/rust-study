fn main() {
    let number = 3;

    //if必须是bool类型的,不能像c++一样直接if number
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //可以用if赋值
    let condition = true;
    //if赋值的所有分支返回的数据类型必须相同
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}
