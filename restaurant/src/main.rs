use restaurant;
use std::collections::HashMap;

fn main(){
    restaurant::eat_at_restaurant();
    restaurant::eat_at_restaurant_enum();
    restaurant::hosting::add_to_waitlist();
    restaurant::serve_order();
    let mut map = HashMap::new();
    map.insert(1, 2);
}


use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
    fmt::Result::Ok(())

}

fn function2() -> io::Result<()> {
    // --snip--
    io::Result::Ok(())

}

use std::fmt::Result;
use std::io::Result as IoResult;

fn function3() -> Result {
    // --snip--
    Result::Ok(())
}

fn function4() -> IoResult<()> {
    // --snip--
    IoResult::Ok(())
}