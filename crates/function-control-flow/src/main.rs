fn main() {
    let res = add({
        let y = 1;
        y + 1
    });

    let _c = {};
    println!("{res}");

    test_if();

    test_loop();
}

fn add(x: i32) -> i32 {
    x + 1
}

fn test_if() {
    let condition = true;
    let condition2 = false;
    let num: i32 = if condition { 5 } else { 6 };
    println!("{num}");

    // let x = 1;
    // let y = if x { 0 } else { 1 };
    // println!("{y}")
    let x;
    if condition {
        x = 1;
    } else if condition2 {
        x = 2;
    } else {
        x = 3;
    }

    println!("{x}")
}

fn test_loop() {
    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("{result}");

    // let mut count = 0;

    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }
    //     count += 1;
    // }

    // let mut number = 3;

    // let a = while number != 0 {
    //     println!("{number}");
    //     number -= 1;
    // };
    let a = [10, 20, 30, 40, 50];

    for ele in a {
        println!("{ele}");
    }

    for number in 1..4 {
        println!("{number}")
    }
}
