fn main2() {
    let mut i = 1;
    let result = loop {
        i += 1;
        if i == 10 {
            break i * 2;
        };
    };
    println!("result is {}", result);
}
fn main3() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}

fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}