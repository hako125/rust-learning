fn main() {
    test1();
}

// 指针传递  所有权变化
fn test1() {
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
