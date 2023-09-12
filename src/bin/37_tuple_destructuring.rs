fn main() {
    let triple = (3, -2, 5);
    // TODO ^ Try different values for `triple`

    println!("Tell me about {:?}", triple);
    // Match can be used to destructure a tuple
    match triple {
        (0, y, z) => println!("First is `0`, second is {:?}, and third is {:?}", y, z),
        (1, ..) => println!("First is `1` and we didn't check the rest"),
        (.., 2) => println!("Last is `2` and we don't care about the others"),
        (3, .., 4) => println!("First is `3` and last ist `4`, but nothing else matters"),
        _ => println!("Nothing that we matched against is in here"),
        // This is the same but unnecessarily complicated
        // (_, _, _) => println!("IDGAF"),
    }
}
