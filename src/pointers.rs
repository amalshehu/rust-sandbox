pub fn run() {
    let arr = vec!['\u{1F60B}', '\u{1F601}', '\u{1F60B}'];
    let arr2 = arr;
    // println!("arr: {:?}", arr); // Error // borrow of moved value: `arr`
    println!("arr2: {:?}", arr2)
}
