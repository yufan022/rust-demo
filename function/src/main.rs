fn main() {
    main1();
    main2();
}

fn main1() {
    another_function(5, 6);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {} x is: {}", y, x);

    // let x = (let y = 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}
fn main2() {
    let five = five();
    println!("The value of x is: {}", x);

}