pub fn run() {
    // Array is a primitive type
    let arr: [i8; 8] = [1, 2, 3, 4, 5, 6, 7, 8]; // 1 bytes each
    println!("ARRAY: {:?}", arr);
    // Arrays are in stack
    println!("size in bytes: {}", std::mem::size_of_val(&arr));
}
