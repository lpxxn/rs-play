由于 Cell 类型针对的是实现了 Copy 特征的值类型，因此在实际开发中，Cell 使用的并不多，因为我们要解决的往往是可变、不可变引用共存导致的问题，此时就需要借助于
RefCell 来达成目的。

我们可以将所有权、借用规则与这些智能指针做一个对比：

| Rust规则             | 	智能指针带来的额外规则           |
|:-------------------|:-----------------------|
| 一个数据只有一个所有者        | 	Rc/Arc让一个数据可以拥有多个所有者  |
| 要么多个不可变借用，要么一个可变借用 | RefCell实现编译期可变、不可变引用共存 |
| 违背规则导致编译错误         | 违背规则导致运行时panic         |

可以看出，Rc/Arc 和 RefCell 合在一起，解决了 Rust 中严苛的所有权和借用规则带来的某些场景下难使用的问题

可以看出，Rc/Arc 和 RefCell 合在一起，解决了 Rust 中严苛的所有权和借用规则带来的某些场景下难使用的问题。但是它们并不是银弹，例如
RefCell 实际上并没有解决可变引用和引用可以共存的问题，只是将报错从编译期推迟到运行时，从编译器错误变成了 panic 异常：

```rust
use std::cell::RefCell;

fn main() {
    let s = RefCell::new(String::from("hello, world"));
    let s1 = s.borrow();
    let s2 = s.borrow_mut();

    println!("{},{}", s1, s2);
}
```

上面代码在编译期不会报任何错误，你可以顺利运行程序：

thread 'main' panicked at 'already borrowed: BorrowMutError', src/main.rs:6:16
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
但是依然会因为违背了借用规则导致了运行期 panic


RefCell 简单总结
与 Cell 用于可 Copy 的值不同，RefCell 用于引用
RefCell 只是将借用规则从编译期推迟到程序运行期，并不能帮你绕过这个规则
RefCell 适用于编译期误报或者一个引用被在多处代码使用、修改以至于难于管理借用关系时
使用 RefCell 时，违背借用规则会导致运行期的 panic
选择 Cell 还是 RefCell
根据本文的内容，我们可以大概总结下两者的区别：

Cell 只适用于 Copy 类型，用于提供值，而 RefCell 用于提供引用
Cell 不会 panic，而 RefCell 会