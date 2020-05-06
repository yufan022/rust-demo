#[warn(unused_variables)]
const MAX_POINTS: u32 = 100_000;

fn main() {
    // var_type();

    // var_tup();
    var_array();
}

fn var_array() {
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("Array {}", a.len());

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    println!("months one {}", months[0]);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];

}

fn var_tup() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
    println!("one is {}", one);
}

fn var_type() {
    println!("Constant MAX_POINTS is: {}", MAX_POINTS);

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The space length is: {}", spaces);

    let test_num: u8 = 255;
    println!("{}", test_num);

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32


    // 加法
    let sum = 5 + 10;

    // 减法
    let difference = 95.5 - 4.3;

    // 乘法
    let product = 4 * 30;

    // 除法
    let quotient = 56.7 / 32.2;

    // 取余
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // 显式指定类型注解


    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
