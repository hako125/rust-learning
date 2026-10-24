# rust函数&流程控制.md

## 函数

rust定义函数的标准模板：

```rust
fn add(x: i32) -> i32 { 
   // rust默认用最后一个表达式的值来作为返回值，意思就是return可以不写  
    x + 1
}
```

## 语句，表达式

- 语句： 执行某些操作，不返回值
- 表达式： 计算并返回一个结果

```rust
// 表达式举例： { } 在rust中既代表一个作用域，也可以代表一个表达式， 其中最后一个表达式的值代表{ }表达式整体的值
let y = {
    let x = 3;
    x + 1
}

// 语句举例：
let z = 1;
```

**{ }表达式的值默认为空元组()， 据此也可以推断出函数的返回值默认为空元组()**

## if表达式

rust用if表达式统一了三元表达式和if语句

```rust
let condition = true;
let num: i32 = if condition { 5 } else { 6 };  //代替三元表达式功能
println!("{num}");

// if 后面必须是布尔值，不存在隐式转换
let x = 1;
let y = if x { 0 } else { 1 };  //编译不通过
println!("{y}")

// if-else if-else
if condition {
    x = 1;
} else if condition2 {
    x = 2;
} else {
    x = 3;
}
```

## loop

```rust
fn test_loop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{result}");
}
```

- break:  停止loop， 支持从loop中返回值。
- continue:  跳出本次迭代

### loop标签

```rust
'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
```

## while

```rust
let mut number = 3;

    let a = while number != 0 {
        println!("{number}");
        number -= 1;
    };
```

## for

主要用来遍历集合

```rust
for ele in a {
    println!("{ele}");
}
```

使用range来让for执行特定次数

```rust
for number in 1..4 {
        println!("{number}")
    }
```