fn main() {
    let str = String::from("test");
    find_first(&str);

    let mut s = String::from("h");

    // let hello = &s[0..5];
    // let world = &s[6..11];

    // println!("{}", hello);
    // println!("{}", world);

    let first_s = first_word(&mut s);
    println!("fisr{}", first_s);

    println!("{}", s);

    s = String::from("hello world new");


    println!("{}", s);
}

fn find_first(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {}

    s.len()
}

fn first_word(s: &mut String) -> &str {
    // 默认不允许修改引用的值
    s.insert_str(0, "a");
    s.remove(0);
    &s[0..1]
}

