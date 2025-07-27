enum IpAddrKind {
    V4, 
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrEnum {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // Method implementation
    }
}

//空值
enum Option<T> {
    Some(T),
    None,
}
//可以不用Option::前缀来直接使用里面这值,这是rust的标准库提供的
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;


let x: i8 = 5;
let y: Option<i8> = Some(5);
//类型不同,不能相加
let sum = x + y;

let m = Message::Write(String::from("hello"));
m.call();

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("0.0.0.0"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}

fn route(ip_type: IpAddrKind) {

}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... other states
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

let dice_roll = 9;
match dice_roll {
    3 => println!("You rolled a three!"),
    7 => println!("You rolled a seven!"),
    other => println!("You rolled a {}", other),
}

match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
    //_ => (),
}

if let Some(3) = some_u8_value {
    println!("Three!");
} else {
    println!("Not three!");
}