// This can't be printed with neither `fmt::Display` or
// with `fmt::Debug`
struct Unprintable(i32);

// Adding the Debug trait to the struct allows to print with `fmt::Debug`
#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Deep(DebugPrintable);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );
    let printable = DebugPrintable(10);
    println!("{:?}", printable);
    let deep_one = Deep(printable);
    println!("{:?}", deep_one);
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}
