pub fn run() {
    let greet = "Hello"; // Primitive str, fixed len, in memory
    println!("Immutable str: {}", greet);

    let mut greet2 = String::from("Hello ");
    greet2.push('U'); // '' is enough for Char type
    greet2.push_str("ser"); // Won't work without ""
    println!("Heap allocated string str: {}", greet2);
    // Capacity
    println!(
        "Capacity in bytes of Heap allocated string : {}",
        greet2.capacity()
    );
}
