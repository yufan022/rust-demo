fn main2() {
    {                      // s 在这里无效, 它尚未声明
        let s = "hello";   // 从此处起，s 是有效的
        // 使用 s
    }                      // 此作用域已结束，s 不再有效

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() 在字符串后追加字面值

    println!("{}", s); // 将打印 `hello, world!`

    {
        let s = String::from("hello"); // 从此处起，s 是有效的

        // 使用 s
    }                                  // 此作用域已结束，
    // s 不再有效
}

fn main3() {
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1);
    // 报错 禁止无效引用 确保内存安全 防止二次释放
}

fn main4() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn main(){
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}