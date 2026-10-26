fn main() {
    test_reference();
}

// 测试所有权的移动
fn test_ownership_move() {
    let first = String::from("Ferris");
    println!("first的地址：{:p}", first.as_ptr());
    let full = add_suffix(first);
    println!("full的地址：{:p}", full.as_ptr());
    println!("{full}");
}

fn add_suffix( name: String) -> String {
    println!("name的地址：{:p}", name.as_ptr());
    // name.push_str(" Jr.AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    println!("name的地址：{:p}", name.as_ptr());
    name
}


fn test_reference() {
    let x = Box::new("2233");
    let y = &x;
    let z = **y;
}