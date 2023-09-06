// Suppress all warning from casts which overflow.
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // Error! No implicit conversion
    // let integer: u8 = decimal;

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    // Error! There are limitations in conversion rules.
    // A float cannot be directly converted to a char.
    // let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // when casting any value to an unsigned type, T,
    // T::MAX +1 is added or subtracted until the value
    // fits into the new type

    // 1000 fits in a u16
    println!("1000 as u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 256 = 232
    println!("1000 as u8 is: {}", 1000 as u8);
    // -1 + 256 = 255
    println!(" -1 as u8 is: {}", (-1i8) as u8);

    // Since Rust 1.45, the `as` keyword performs a *saturating cast*
    // when casting from float to int. If the floating point value exceeds
    // the upper bound or is less than the lower bound, the returned value
    // will be equal to the bound crossed.

    // 300.0 as u8 is 255
    println!("300.0 as u8 is: {}", 300.0_f32 as u8);
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is: {}", -100.0_f32 as u8);
    // nan as u8 is 0
    println!("nan as u8 is: {}", f32::NAN as u8);

    // This behavior incurs a small runtime cost and can be avoided
    // with unsafe methods, however the results might overflow and
    // return **unsound values** . Use these methods visely:
    // [If you have to ask if it's a good idea, it's not ;-)]
    unsafe {
        // 300.0 as u8 is 44
        println!("300.0 as u8 is: {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!(
            "-100..0 as u8 is: {}",
            (-100.0_f32).to_int_unchecked::<u8>()
        );
        // nan as u8 is 0
        println!("nan as u8 is: {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
