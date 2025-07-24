fn main() {
    // let默认不可变
    let x = 5;
    x = 6;

    //mut修饰可变变量
    let mut x = 5;
    x = 6;

    //常量永远不可变, 并且需要指定数据类型
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    //第二次声明相同名称的变量意味着遮蔽之前的变量
    let x = 5
    let x = x + 1;
    {
        //属于内部遮蔽
        let x = x * 2;
        //此处x为12
    }
    //退出内部作用域,x恢复为6

    //这是可以的,变量遮蔽可以变换变量的数据类型,且不可逆
    let space = "   ";
    let space = space.len();

    //这是不允许的,mut可以改变值但是不能改变数据类型
    let mut space = "  ";
    space = space.len();

    //rust的标量类型有整形,浮点型,布尔型,字符类型
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    // remainder
    let remainder = 43 % 5;

    //rust的复合类型有元组(tuple)和数组(array)
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //可以使用模式匹配来解构元组
    let (x, y, z) = tup;
    //也可以使用.来索引访问
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    //注意:没有任何值的元组 () 是一种特殊的类型，只有一个值，也写成 ()。
    //该类型被称为单元类型（unit type），该值被称为单元值（unit value）。
    //如果表达式不返回任何其他值，就隐式地返回单元值。

    //数组的每个元素的数据类型必须相同
    let a = [1, 2, 3, 4, 5];
    //使用方括号编写数组的类型和元素数
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    //如果要创建相同的数比如创建5个3
    let a = [3; 5];
    //可以用索引访问数组
    let first = a[0];
    let second = a[1];
}