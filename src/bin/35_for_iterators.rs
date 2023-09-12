fn main() {
    let borrowed_names = vec!["Bob", "Frank", "Ferris"];

    for name in borrowed_names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", borrowed_names);

    let consumed_names = vec!["Brian", "Judith", "Ferris"];

    for name in consumed_names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // println!("names: {:?}", consumed_names);
    // FIXME ^ comment out this line of code because consumed_names has been moved

    let mut unstable_names = vec!["Spam", "Egg", "Ferris"];

    for name in unstable_names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", unstable_names);
}
