fn main() {
        {// s 在这里无效, 它尚未声明
            let s = "hello";// 从此处起，s 开始有效
            // 使用 s
        }// 此作用域已结束，s 不再有效

        let mut s = String::from("hello");
        s.push_str(", world!"); // push_str() 在字符串后追加字面值

        println!("{}", s); // 将打印 `hello, world!`

        {
            let s = String::from("hello"); // 从此处起，s 开始有效
    
            // 使用 s
        }// 此作用域已结束，
        // s 不再有效

        //这相当于拷贝赋值因为是栈上的变量
        let x = 5;
        let y = x;

        //这里就不一样了, 相当于两个指针同时指向string的这个数据地址
        //s2仅仅只是在站上拷贝了s1的指针,长度和容量,没有复制指针所指堆上的数据
        //在执行第二局语句后,s1不在有效, 也就是说s1连基础的打印都不行了,更不能访问
        let s1 = String::from("Hello");
        let s2 = s1;
        //需要注意的是rust永远不会自动深拷贝数据,基本都是移动拷贝

        //如果需要使用深拷贝
        let mut s3 = s2.clone();

        //实现了Copy trait他特性的类型,可以实现把旧的变量赋值给其他变量后仍然可用
        //比如整型, 布尔型,浮点型,字符型,元组(只有在其内的变量都是先该特性的时候才是)

        let r2 = &mut s3;
}
