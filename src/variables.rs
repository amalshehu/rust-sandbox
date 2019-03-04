// Variables are immutable.
// Rust is a blocked scope lang.
pub fn run() {
    let user = "DEXTER";
    // let is immutable, can't reassign.
    // For that have to use mut keyword.
    const CODE: i32 = 1010101;
    println!("{} has a lab.", user);
    println!("LAB CODE: {}", CODE);
}
