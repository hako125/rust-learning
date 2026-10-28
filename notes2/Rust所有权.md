# Rust所有权

>  **The Rust Programming Language（Brown 大学 Cognitive Engineering Lab 的交互式实验版本）：**https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html#safety-is-the-absence-of-undefined-behavior
>  **The Rust Programming Language（官方）：**https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

> **一句话记忆**：一个主人（own）、交出即失效（move）。

## 关于 Rust 所有权的个人理解

C/C++ 中经常出现的堆内存问题：

- 没人负责 free → 内存泄漏
- 多个人想 free → 二次释放
- free 之后还有人用 → 悬垂指针

这三类问题的根因其实是同一个：**同一块内存同时有两个名字，且其中一个改了或释放了它**。

所以 Rust 的所有权就是为避免这类问题而制定的**编译期堆内存使用规则**：每份堆数据只有一个主人，主人决定它何时交出 **（move）**、何时释放（drop）。

## Move

Rust 赋值操作时会伴随所有权责任（谁负责 drop 数据）的转移（Move），失去所有权的变量将会在变量表上被写死，因此无法再访问、释放数据（E0382: use of moved value）。

### Copy Trait

 `Copy` 是 Move 的豁免：像 `let a = 123; let b = a;` 这种完全没有必要使用所有权管理，Rust 用 `Copy` 标记这类类型，因此这种赋值操作不触发 Move。


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
