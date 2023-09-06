fn main() {
    // Integer addition (unsigned 32 and type inference)
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction (signed 32 and type inference)
    // If we would use u32, we get an underflow, rust calls it overflow
    // Compiler protects us from doing silly things again
    println!("1 - 2 = {}", 1i32 - 2);

    //Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOt true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101u32);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101u32);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101u32);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // We can use underscores in long numbers to improve readability
    // the rust compiler will not check if those separators are sanely placed!!
    println!("One million is written as {}", 1_000_000u32);
}
