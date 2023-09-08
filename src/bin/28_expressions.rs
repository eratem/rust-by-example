fn main() {
    // a rust program is (mostly) made of statements
    // examples
    let x = 5;
    x + 1;
    15;

    // blocks are also just statements
    // the last experssion is returned unless terminated with `;`
    let y = {
        let x_squared = x * x;
        let x_cubed = x_squared * x;

        x_cubed + x_squared + x
    };

    let z = {
        // The semicolon suppresses theis expression and `()` is assigned to `z`
        2 * x;
    };
    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
