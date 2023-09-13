#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn main() {
    let temperature = Temperature::Celsius(35);
    // TODO ^ try different values for `temperature`

    match temperature {
        Temperature::Celsius(t) if t > 25 => println!("{}°C is above 25°C", t),
        Temperature::Celsius(t) => println!("{}°C is below 25°C", t),
        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}F is below 86 Fahrenheit", t),
    }

    let number: u8 = 4;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => unreachable!("Should never happen."),
        // TODO ^ uncomment to fix compilation
    }
}
