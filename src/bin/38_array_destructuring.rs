fn main() {
    // Try changing the value in the array, or make it a slice!
    let array = [3, -2, 6];

    match array {
        // Binds the second and the third elements to the respective variables
        [0, second, third] => println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
        // Single values can be ignored with the hole `_`
        [1, _, third] => println!("array[0] = 1, array[1] = ?, array[2] = {}", third),
        // You can also bind some and ignore the rest
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the rest is ignored",
            second
        ),
        // The code below would not compile because it has a different amount
        // of elements in the array
        // [-1, second] => println("...

        // You can store some parts of the array in a slice or array
        [3, tail @ ..] => println!("array[0] = 3, tail = {:?}", tail),
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[-1] = {}",
            first, middle, last
        ),
    }
}
