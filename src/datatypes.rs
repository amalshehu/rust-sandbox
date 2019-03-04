// Primitives
pub fn run() {
    // Numbers
    let max_i32 = std::i32::MAX;
    let max_i64 = std::i64::MAX;
    let max_i128 = std::i128::MAX;
    println!("MAX: i32: {}", max_i32);
    println!("MAX: i64: {}", max_i64);
    println!("MAX: i128: {}", max_i128);
    // MAX: i32: 2147483647
    // MAX: i64: 9223372036854775807
    // MAX: i128: 170141183460469231731687303715884105727

    // Boolean
    let is_user_logged = false;
    println!("User logged ? {}", is_user_logged);

    //let is_greater: bool = max_i128 > max_i64;  -> don't do this :) // mismatched types
    // expected i128, found i64
    // help: you can cast an `i64` to `i128`, which will sign-extend the source value: `max_i64.into()`

    println!(
        "PRINT ALL (max_i32, max_i64, max_i128): {:?}",
        (max_i32, max_i64, max_i128)
    );

    // Char -> only one charcter
    let c = 'x';
    println!("CHAR {}", c);
    // Unicode
    let u = '\u{1F601}';
    println!("UNICODE {}", u);
}
