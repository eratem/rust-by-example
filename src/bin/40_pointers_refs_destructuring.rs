fn main() {
    // Assign a reference of type `i32`. The `&` signifies there
    // is a reference being assigned.
    let reference = &4;

    match reference {
        // If `reference` is pattern matched against `&val`, it results
        // in a comparison like:
        // `&i32`
        // `&val`
        // ^ we see that is the matching `&`s are dropped, the the `i32`
        // should be assigned to `val`.
        &4 => println!("Got a value via destructuring: 4"),
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid the `&`, you dereference with `*` before matching.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // What if you don't start with a reference? `reference` was a `&`
    // because the right side was already a reference. This is not
    // a reference because the right side is not one.
    let _not_a_reference = 3;

    // Rust provides `ref` for exactly this purpose. It modifies the
    // assignment so that a reference is created for the element;
    // this reference is assigned.
    let ref _is_a_reference = 3;

    // Accordingly, by defining 2 value without references, references
    // can be retrieved via `ref` and `ref mut`.
    let value = 5;
    let mut mut_value = 6;

    // Use `ref` to create a reference.
    match value {
        ref r => println!("Got a reference to a value {:?}", r),
    }

    // Use `ref mut` similarly.
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereferenceit before we can
            // add anything to it.
            *m += 10;
            println!("We added 10. `mut value`: {:?}", m);
        }
    }
}
