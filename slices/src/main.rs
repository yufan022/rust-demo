fn main() {
    let str = String::from("test");
    find_first(&str);

    let mut s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{}", hello);
    println!("{}", world);

    let first_s = first_word(&mut s);
    println!("{}", first_s);

    println!("{}", s);

    s = String::from("hello world new");


    println!("{}", s);

}

fn find_first(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {

    }

    s.len()
}

fn first_word(s: &mut String) -> &str {
    // 默认不允许修改引用的值
    s.push_str("aaa");

        & s[0..1]
}

