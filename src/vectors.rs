use std::mem;
pub fn run() {
    // Vector is similar to array
    let mut vect: Vec<i8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    vect.push(9);
    println!("vector: {:?}", vect);
    println!("size in bytes: {}", mem::size_of_val(&vect));

    // vector: [1, 2, 3, 4, 5, 6, 7, 8, 9]
    // size in bytes: 24
}
