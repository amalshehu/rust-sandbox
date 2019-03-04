pub fn run() {
    // loop {
    //     println!("hello world forever!"); :p
    // }
    let mut num = 0;
    loop {
        num += 1;
        println!("Num {}", num);
        if num > 99 {
            break;
        };
    }
}
