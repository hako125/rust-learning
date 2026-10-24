pub fn introduce() {
    crate::module1::module1_hello::hello();
    println!("I'm hako");
}

pub fn get_name() -> String {
    return "hako".to_string();
}
