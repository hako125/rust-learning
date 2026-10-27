fn main() {
    test_reference2();
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

// 测试引用
fn test_reference() {
    let x = 233;
    let x = &x;  // 创建一个引用
    let p: *const i32 = x as *const i32;  // 将引用转换为指针
    println!("p的地址：{:p}", p);   //  0x2efcf7f914
    println!("x的地址：{:p}", x);   //  0x2efcf7f914
}

fn test_reference2() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");

    let p1 = &m1 as *const String; // note the ampersands
    let p2 = &m2 as *const String; // note the ampersands
    greet(p1, p2); // note the ampersands
    let s = format!("{} {}", m1, m2);
}

fn greet(g1: *const String, g2: *const String) { // note the ampersands
    unsafe {
        println!("{} {}!", *g1, *g2);
    }
}

// 测试解引用
// fn test_dereference() {
//     let x = Box::new("2233");
//     let y = &x;
//     let z = **y;
// }  