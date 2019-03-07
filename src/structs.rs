struct User {
    username: String,
    email_addrress: String,
}

impl User {
    fn create(name: &str, email: &str) -> User {
        User {
            username: name.to_string(),
            email_addrress: email.to_string(),
        }
    }
}
pub fn run() {
    let _user = User::create("Amal Shehu", "amalshehu@gmail.com");
    println!("Name: {}", _user.username);
    println!("E mail: {} ", _user.email_addrress);
}
