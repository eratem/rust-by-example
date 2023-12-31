fn main() {
    // This biding lives in the main function
    let long_lived_binding = 1;

    // This is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
    }
    // End of block

    // Error! `short_lived_binding` dosen't exist in this scope
    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);
}
