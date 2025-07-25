struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//类单元结构体
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        email: String::from("da"),
        username: String::from("nihao"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("abc");

    //结构更新语法是移动构造
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
}


fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

//如果参数名字和字段名相同
// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }


//派生debug特性
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {:?}", rect1);
// }


#[derive(Debug)]
struct Rectangle2 {
    width: u32,
    height: u32,
}

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     dbg!(&rect1);
// }

// [src/main.rs:10] 30 * scale = 60
// [src/main.rs:14] &rect1 = Rectangle {
//     width: 60,
//     height: 50,
// }
