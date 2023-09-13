fn main() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // Try changing the values in the struct to see what happens
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {}, y = {}", b, y),

        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about the rest", y),
        // this will give an error: pattern does not mention field `x`
        // Foo {y} => println!("y = {}", y),
    }

    let spam = Foo { x: (1, 2), y: 3 };

    // You don't need a match block to destructure structs:
    let Foo { x: x0, y: y0 } = spam;
    println!("Outside: x0 = {x0:?}, y0 = {y0}");
}
