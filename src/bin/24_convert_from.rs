use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(32);
    println!("My number is {:?}", num);

    // `from` trait is readily available for many `std` library implementations
    // herer for example convert from `str` to `String`
    let my_str = "hello";
    let my_string = String::from(my_str);
    println!("That's a string: {}", my_string);
}
