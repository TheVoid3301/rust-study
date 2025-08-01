use std::io;
use std::collections::HashMap;

fn main() {
    let (mut average, mut mid, mut mode) = (0, 0, 0);
    let mut v = Vec::new();
    let mut count_map = HashMap::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error");
        let input = input.trim();

        if input.is_empty() {
            break; // 空行结束输入
        }

        match input.parse::<i32>() {
            Ok(num) => v.push(num),
            Err(_) => println!("无效数字，请重新输入！"),
        }
    }
    for i in &v {
        average += i;
        *count_map.entry(*i).or_insert(0) += 1;
    }
    v.sort();
    mid = v[v.len() / 2];
    mode = count_map.iter()
        .max_by_key(|&(_, count)| count)
        .map_or(0, |(&num, _)| num);

    println!("平均值: {}", average as f64 / v.len() as f64);
    println!("中位数: {}", mid);
    println!("众数: {}", mode);
}