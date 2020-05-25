#[derive(Debug)]
struct Box {
    width1: i32,
    height1: i32,
}

fn main1() {
    let width1 = 30;
    let height1 = 50;
    let boxs = Box {
        width1: 30,
        height1: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        // area(width1, height1)
        area_struct(&boxs)
    );

    println!("{:#?}", boxs);
}

fn area(width1: i32, height1: i32) -> i32 {
    width1 * height1
}

fn area_struct(boxs: &Box) -> i32 {
    boxs.height1 * boxs.width1
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, target: &Rectangle) -> bool {
        let flag = false;
        self.width > target.width && self.height > target.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
}