fn main() {
    _test_borrow();
}

fn _test_move() {
    // 1. 一个变量被move后， 那么我给这个变量赋值成其他值会怎样
    let mut x = String::from("hello");
    let y = x; // x被move到y中，x不再有效

    // x.push_str("world");  // 报错， x已经move，无法再访问之前所拥有的数据

    x = String::from("world"); // x重新赋值，x重新拥有了一个新的String对象和之前的已经没关系了

    println!("y: {}", y);
    println!("x: {}", x);
}

fn _test_borrow() {
    let mut a = vec![1, 2, 3];
    let b = &a[2];
    let c = &a[1];
    // a.pop(); // 报错， 因为在借用期间，不能修改a

    println!("b: {}", b);
    println!("c: {}", c);

    a.pop(); // 现在可以修改a了，因为b的借用已经结束
}

fn _test_borrow_mut() {
    let mut a = vec![1, 2, 3];
    
    let b = &mut a;
    
     // 可变借用a
    b.push(4); // 可以修改a
    // println!("a: {}", a[1]);  // 报错， 因为b是可变借用， 在其使用期间，a不能读取，不能修改
    println!("c: {:?}", c);
    println!("b: {:?}", b);

    println!("a: {}", a[1]);
}
