# Rust所有权

>  参考资料：
>  **The Rust Programming Language（Brown 大学 Cognitive Engineering Lab 的交互式实验版本）：**https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html#safety-is-the-absence-of-undefined-behavior
>  **The Rust Programming Language（官方）：**https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html


## Why Ownership?

C/C++ 使用中需要时刻留意堆内存问题：

- 没人负责 free → 内存泄漏
- 多个人想 free → 二次释放
- free 之后还有人用 → 悬垂指针

这三类问题的根因其实是同一个：**因为用了指针，多个变量共享了同一个数据的内存空间**。

Rust想要通过限制共享情景来达到编译期间就避免以上内存问题的发生，于是创造了所有权规则（Ownership）

## Move

可以先把所有权理解成一种责任：**负责销毁这一块数据的内存空间**。

- Rust 中使用赋值操作的时候，会将数据的所有权交给变量，拥有所有权的变量一旦被销毁，那么对应的数据的内存空间就会被释放。
- 所有权是可以转移的，例如从变量a转移到变量b，但是所有权转移之后，失去所有权的变量将被标记为“moved”，这个变量将无法再访问之前的数据。

误区1： **moved 变量并不是被销毁了**，如果给它赋值了其他的值，那么它依然可用。

```rust
// 1. 一个变量被move后， 那么我给这个变量赋值成其他值会怎样
let mut x = String::from("hello");
let y = x; // x被move到y中，x不再有效

// x.push_str("world");  // 报错， x已经move，无法再访问之前所拥有的数据

x = String::from("world");   // x重新赋值，x重新拥有了一个新的String对象和之前的已经没关系了
println!("y: {}", y);
println!("x: {}", x);
```

Rust只有拥有所有权的变量才负责对应的数据的销毁，**避免了二次销毁问题**。

```rust
let x = String::from("hello");
let y = x;  // y 获得所有权
// 执行完毕后，销毁x和y，因为所有权只有y有，所以堆上的数据在y销毁的时候才销毁，销毁x的时候不会销毁
```

**Copy Trait**： `Copy` 是 Move 的豁免：像 `let a = 123; let b = a;` 这种完全没有必要使用所有权管理，Rust 用 `Copy` 标记这类类型（如：i32, u32,bool），因此这种赋值操作不触发 Move。

### 函数传参 = 赋值

`fn takes_ownership(s: String)` 调用即 Move；`fn makes_copy(x: i32)` 因为 `i32` 是 `Copy`，不 Move。

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);              // s 被 Move 走，之后再用 s 就是 E0382

    let x = 5;
    makes_copy(x);                   // i32 是 Copy，复制了一份，x 仍可用
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}                                    // 在这里 drop，堆内存在这里释放

fn makes_copy(some_integer: i32) {   // 复制出来的独立副本
    println!("{some_integer}");
}
```

> 注意 `takes_ownership` 结尾那行：**drop 发生在新主人 `some_string` 那里，不在 `s` 那里**。
> `s` 走出作用域时什么都不做——这正是“责任已经转移”的直接证据。

### 函数返回也会移动所有权

`gives_ownership()` 和 `takes_and_gives_back()` 都说明：**返回值是所有权从函数移出到调用者的过程**。

```rust
fn main() {
    let s1 = gives_ownership();         // 返回值 move 进 s1

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);  // s2 move 进函数，返回值 move 进 s3
} // s1、s3 各自 drop；s2 早已 move，什么都不做

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string                         // move 出函数，交给调用者
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string                            // 收进来又交出去
}
```
