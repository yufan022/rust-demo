fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    {
        let v = vec![1, 2, 3, 4];

        // 处理变量 v
    } // <- 这里 v 离开作用域并被丢弃

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // v.push(6);

    println!("The first element is: {}", first);


    let mut v = vec![100, 32, 57];
    for i in &mut v {
        println!("loop i {}", i);
        *i += 50;
    }

    let v = vec![SpreadsheetCell::Int(1),
                 SpreadsheetCell::Float(1.1), SpreadsheetCell::Text(String::from("aa"))];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
