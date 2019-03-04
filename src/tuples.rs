pub fn run() {
    let user: (&str, i8, &str) = ("Amal Shehu", 26, "Programmer");
    println!(
        "{} is a {} year old {}, who loves Rust.",
        user.0,
        user.1,
        user.2.to_lowercase()
    );
}
