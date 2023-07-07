fn main() {
    // {} are replaced by arguments provided if they can be
    // stringified
    println!("{} days", 30);

    // We can use ints in the {} for positional arguments
    println!("{0}, this is {1}, {1}, this is {0}", "Alice", "Bob");

    // We can also declare named arguments
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // arguments in the {} can define the formatting of numbers
    println!("Base 10: {}", 69420);
    println!("Base 2 (binary): {:b}", 69420);
    println!("Base 8 (octal): {:o}", 69420);
    println!("Base 16 (hexadecimal): {:x}", 69420);
    println!("Base 16 (hexadecimal): {:X}", 69420);

    // arguments can be right padded
    println!("{number:>5}", number = 1);
    // or left padded with definet filler
    println!("{number:0<5}", number = 1);
    // maybe 0 is not that goot
    println!("{number:-<5}", number = 1);
    // and fancy with named arguments for the padding
    println!("{number:->width$}", number = 1, width = 5);

    // Rust will complain about wrong number of arguments, macro magic
    // println!("My name is {0}, {1} {0}", "Bond"); //would not compile

    //Only types that implement fmt::Display can be formatted with `{}`.
    // User-defined types do not implement fmt::Display by default.
    // with #[derive(Debug)] we can get the debug print with {:?}
    #[derive(Debug)]
    struct Structure(i32);

    let test = Structure(5);
    println!("my int-struct {:?}", test);

    // with Rust >1.58 we can directly access the varibles from context
    let number = 1.0;
    let width = 6;
    println!("{number:|>width$}");
}
