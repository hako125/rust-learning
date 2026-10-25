# rust引用与借用-原理

> 来源：https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html
> 核心：借用检查器 = 一套**编译期权限系统（R/W/O/F）**，保证"数据不能同时被别名和修改"。

## 1. 为什么需要引用

move-only 的 API 很麻烦：函数拿到值就吃掉所有权，调用后就没了。

```rust
fn greet(g1: String, g2: String) -> String { g1 + " and " + &g2 }

let m1 = String::from("Hello");
let m2 = String::from("world");
let msg = greet(m1, m2);
println!("{m1}");   // ❌ E0382: m1 已被 move
```

两种解法：

```rust
// 方案 A：返回所有权（可行但啰嗦）
fn greet(g1: String, g2: String) -> String { g1 + " and " + &g2 }
let (m1, m2, msg) = ...

// 方案 B：借用（Rust 推荐）✅
fn greet(g1: &String, g2: &String) -> String { g1 + " and " + g2 }
greet(&m1, &m2);     // m1/m2 依然可用
```

## 2. 引用是"非拥有指针"

```rust
let m1 = String::from("Hello");
let g1: &String = &m1;
// g1 ──> m1 ──> "Hello"（两跳）
```

- `&m1` 用取地址运算符创建**借用（borrow）**
- `g1` 不拥有 m1，也不拥有堆数据 → 函数结束时只销毁 `g1` 的栈帧，**堆数据不释放**

> 呼应 Box 释放原则：只有 `m1` 拥有堆数据，只有它离开作用域才 free。

## 3. 解引用 `*`

`&` 取引用，`*` 解引用（顺着指针拿到数据）。

```rust
let x = 5;
let r1: &i32 = &x;      // r1 指向栈上的 x
let r2: &Box<i32> = &Box::new(2);  // r2 指向栈，堆值 2
println!("{}", *r1);    // 5
```

### 隐式解引用（实际代码里很少手写 `*`）

点语法会**自动插入 `&` / `*`**，且**可多层**：

```rust
let x: Box<i32> = Box::new(-1);
i32::abs(*x);      // 显式解引用
x.abs();           // 隐式等价

let r: &Box<i32> = &x;
i32::abs(**r);     // 需要两层
r.abs();           // 隐式等价

let s = String::from("Hello");
str::len(&s);      // 显式借用（String → str 还有一层 deref）
s.len();           // 隐式等价
```

## 4. 指针安全原则（核心）

**别名（aliasing）= 通过不同变量访问同一份数据。**

别名本身无害，但**别名 + 修改**就危险，可能造成：

- 一个变量 free 了数据，另一个变量悬垂
- 修改破坏了另一个变量依赖的不变式（如 Vec 的 len 与 cap）
- 并发修改 → 数据竞争

```rust
let mut v = vec![1, 2, 3];
let num: &i32 = &v[2];   // 别名
v.push(4);               // 修改：容量满 → 重新分配 → 旧数组 free
println!("{}", *num);    // ❌ UB：num 悬垂
```

> **Pointer Safety Principle：数据不能在别名存在的同时被修改。**
> 可以别名；可以修改；但不能同时。

- **Box（拥有型指针）**：靠"禁止别名"实现 → 赋值即 move，旧变量失效
- **引用（非拥有指针）**：天生就是为了别名 → 需要另一套规则 = **借用检查器**

## 5. 权限模型：Read / Write / Own

借用检查器的核心思想：**变量对数据有三种权限**（只在编译期存在）。

| 权限 | 含义 |
|---|---|
| **R** (Read) | 数据可被复制到别处 |
| **W** (Write) | 数据可被修改 |
| **O** (Own) | 数据可被 move 或 drop |

- 默认：`let x` → **R + O**
- `let mut x` → **R + W + O**
- 引用会**临时移除**被借用者的权限

### 不可变借用

```rust
let mut v = vec![1, 2, 3];
let num = &v[2];       // 借用
println!("{}", *num);  // 使用
v.push(4);             // 借用结束后才 push
```

逐行权限变化：

1. `let mut v` → v 获得 **+R +W +O**（已初始化）
2. `let num = &v[2]`
   - v **失去 W 和 O**（仍可 R）→ 借用期间既不能改、也不能 move/drop
   - `num` 获得 **R + O**（没有 W，因为不是 `let mut num`）
   - `*num` 获得 **R**
3. 最后使用 `num` 之后 → 借用结束，v 恢复 **W + O**，`num`/`*num` 全部丢失
4. `v.push(4)` 之后 v 丢失全部权限（不再使用）

### Place（位置）

权限作用于 **place**，不是只作用于变量。place = 能放在赋值号左边的东西：

```rust
a          // 变量
*a         // 解引用
a[0]       // 数组/切片索引
a.0        // 元组字段
a.field    // 结构体字段
*((*a)[0].1)  // 任意组合
```

**为什么 `num` 和 `*num` 分开看？** 因为"操作引用本身"和"通过引用操作数据"是两件事：

```rust
let mut x = 1;
let y = 2;
let mut x_ref = &x;   // x_ref 有 W → 可以改指向
x_ref = &y;           // ✅ 换引用
// *x_ref += 1;       // ❌ *x_ref 没有 W → 不能改数据
```

## 6. 借用检查器如何发现违规

每次操作都会**要求**某些权限；权限不满足 → 编译错误。

```rust
let mut v = vec![1, 2, 3];
let num = &v[2];
v.push(4);              // 要求 v 的 R+W，但 W 已被借用拿走
println!("{}", *num);
```

```
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
  |
3 | let num: &i32 = &v[2];
  |                  - immutable borrow occurs here
4 | v.push(4);
  | ^^^^^^^^^ mutable borrow occurs here
5 | println!("{}", *num);
  |                 ---- immutable borrow later used here
```

> 错误信息只是表面原因；**真正原因是 `push` 可能让 `num` 悬垂**。

## 7. 可变引用：唯一且非拥有

`&mut T` —— 允许修改，**禁止别名**。

```rust
let mut v: Vec<i32> = vec![1, 2, 3];
let num: &mut i32 = &mut v[2];
*num += 1;      // ✅ 通过 *num 修改 v[2]
// v.push(4);   // ❌ 借用期间 v 无任何权限
```

与不可变借用的两点区别：

1. 不可变借用时 v 仍保留 **R**；可变借用时 v **丢掉全部权限**（R/W/O 全没）→ 因此不构成别名，安全
2. 不可变借用时 `*num` 只有 **R**；可变借用时 `*num` 获得 **W** → 能改数据

注意 `num` 本身仍无 W（不是 `let mut num`）→ **不能重新赋值引用本身**，只能通过 `*num` 改数据。

### 可变引用的"降级"

```rust
let num2 = &*num;   // 把 &mut 转成 &
// 此时 *num 失去 W，保留 R
// println! 能同时读 *num 和 *num2 ✅
```

## 8. 生命周期 = 借用的作用范围

**lifetime（生命周期）**：从引用创建（出生）到最后一次使用（死亡）的代码范围。

```rust
let x = 1;
let y = &x;      // y 出生
let z = *y;      // y 最后一次使用 → 死亡
// 此后 x 恢复 W 权限
```

### 控制流下生命周期不连续

```rust
fn capitalize_first(v: &mut Vec<char>) {
    let c = &v[0];              // 借用 *v
    if c.is_ascii_lowercase() {
        let upper_c = c.to_ascii_uppercase();  // c 在这里还用
        v[0] = upper_c;         // ❌ 此处 *v 还没收回 W（c 仍活）
    } else {
        v[0] = 'X';             // ✅ else 分支里 c 没用过 → W 已收回
    }
}
```

> 同一个变量在**不同分支**可以有不同生命周期 → 权限收回时机也不同。
> 想让它编译通过：把 `c` 的值先拷出来（`to_ascii_uppercase()` 返回 `char`），或在 if 之前结束借用。

## 9. 数据必须比引用活得久

### 情况一：同一函数内 —— 用 R/W/O 检查

```rust
let s = String::from("hello");
let s_ref = &s;     // 借用拿走 s 的 O
drop(s);            // ❌ drop 需要 O 权限 → 权限不匹配
println!("{s_ref}");
```

## 情况二：跨函数 —— 引入第四个权限 F（Flow）

当引用作为**函数参数**或**返回值**时，编译器不知道它活多久，需要用 **F 权限**（引用能否在某表达式里"流动"）。

```rust
fn first(strings: &Vec<String>) -> &String {
    let s_ref = &strings[0];
    return s_ref;        // ✅ &strings[0] 有 F 权限
}
```

```rust
fn first_or(strings: &Vec<String>, default: &String) -> &String {
    if strings.len() > 0 { &strings[0] } else { default }
    // ❌ E0106: missing lifetime specifier
}
```

```
= help: this function's return type contains a borrowed value,
        but the signature does not say whether it is borrowed from
        `strings` or `default`
```

为什么必须报错？看这个调用：

```rust
let strings = vec![];
let default = String::from("default");
let s = first_or(&strings, &default);
drop(default);        // 若返回值可能来自 default，这里就让 s 悬垂
println!("{}", s);
```

**编译器的困境**：只看签名，无法判断返回的 `&String` 来自 `strings` 还是 `default`。于是拒绝通过。
解法是**生命周期参数**（第 10.3 章）：

```rust
fn first_or<'a>(strings: &'a Vec<String>, default: &'a String) -> &'a String
```

返回栈上变量的引用同理：

```rust
fn return_a_string() -> &String {
    let s = String::from("hi");
    return &s;     // ❌ missing lifetime specifier（&s 没有返回所需的 F 权限）
}
```

> **要点**：(1) 函数内引用 vs 输入/输出引用，检查机制不同；(2) 跨函数用 **F 权限**。

## 10. 总结（原文 Summary）

引用让你**不消耗所有权**地读写数据：用 `&` / `&mut` 创建，用 `*`（常隐式）解引用。

借用检查器用权限系统保证安全：

1. **所有变量都能 R、O，可选 W**
2. **创建引用会把权限从被借用的 place 转移给引用**
3. **引用生命周期结束后，权限归还**
4. **数据必须比所有指向它的引用活得更久**

> 这一节讲的多是"Rust 不能做什么"——这是故意的。理解这些规则，才能不跟编译器打架。

## 11. 速查

| 操作 | 对被借用者的权限影响 |
|---|---|
| `let r = &v[i]` | v 失去 **W、O**，保留 R |
| `let r = &mut v[i]` | v 失去 **R、W、O**（全部） |
| 引用不再被使用（生命周期结束） | 权限**全部归还** |
| `drop(s)` 时有活跃引用 | ❌ 缺 O 权限 |
| `v.push()` 时有活跃引用 | ❌ 缺 W 权限（E0502） |
| 函数签名里有输入+输出引用 | 需要生命周期参数，否则 E0106 |

## 12. 一句话记忆

**引用是非拥有别名 → 别名不能与修改共存 → 借用把 W/O 权限暂时拿走 → 引用死了权限归还 → 数据必须比引用活得久。**
