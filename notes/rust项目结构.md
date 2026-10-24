
# Rust 项目结构

练习: cargo run -p project-struct

层级：**package**（Cargo.toml）→ **crate**（编译单元）→ **module**（代码组织单元）

## package

可以简单理解为rust项目

- 有一个Cargo.toml
- 可以有多个Crate

## crate

编译器一次处理的最小单位。日常说的“库/包”，在 Rust 里通常指 crate

| 类型          | 入口          | 特点                                   |
| ------------- | ------------- | -------------------------------------- |
| binary crate  | `src/main.rs` | 有 `main`，可执行                      |
| library crate | `src/lib.rs`  | 无 `main`，不可执行，供其他 crate 使用 |

**package中可以有多个binary crate，但是只能有一个library crate**

## module

module是实际的代码片段，为了方便管理划分成一个一个的module

- `mod garden;` 声明，可嵌套子模块
- `use` 引入使用
- `pub` 导出使用（pub也可以用可见性来理解，但是在模块这方面我直接将其理解为导出）

## 定义使用模块的完整步骤

rust采用约定的方式来规范模块的路径

目前最标准的定义使用模块方式：

```rust
// lib.rs
// 在lib.rs中定义了一个module1->查找lib.rs同层级下的module1.rs
pub mod module1;

// module1.rs
// 再声明一个子模块
pub mod module1_hello;

// module1/module1_hello.rs
// 这里写模块的具体代码
pub fn hello() {
    println!("hi,this is module1!");
}



// main.rs
// 路径：package名/模块名/子模块名/……
use project_struct::module1::module1_hello;

fn main() {
    module1_hello::hello();
}

```

## 几个特殊的模块路径指向型关键字

- crate
- super
- self


```rust
// crate关键字用来指向当前模块所在crate

// 如这里 我在module1.rs中增加了一个新的module:module1_bye

// module1.rs
pub mod module1_bye;
pub mod module1_hello;

// module1/module1_bye.rs
// 在该module下，我可以用crate指向module1_bye所在的crate，也就是项目的library crate
// 可以通过这个crate找到module1_hello中的hello函数
pub fn introduce() {
    crate::module1::module1_hello::hello();
    println!("I'm hako");
}


// super关键字用来指向当前模块的父模块
// self关键字用来指向当前模块
```

## 二次导出

`pub use` 可以实现将当前模块的某个子模块进行二次导出(虽然感觉没卵用)

```rust
// lib.rs
mod front_of_house {                    // 模块本身私有
    pub mod hosting {                   // 模块内部 pub
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting; // 把 hosting 重新导出到 crate 根

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```