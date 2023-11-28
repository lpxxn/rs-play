/*
三种 Deref 转换
在之前，我们讲的都是不可变的 Deref 转换，实际上 Rust 还支持将一个可变的引用转换成另一个可变的引用以及将一个可变引用转换成不可变的引用，规则如下：

当 T: Deref<Target=U>，可以将 &T 转换成 &U，也就是我们之前看到的例子
当 T: DerefMut<Target=U>，可以将 &mut T 转换成 &mut U
当 T: Deref<Target=U>，可以将 &mut T 转换成 &U
 */

use std::ops::{Deref, DerefMut};


#[derive(Debug)]
struct MyBox<T> {
    value: T,
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox { value: x }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
        /*
        要实现 DerefMut 必须要先实现 Deref 特征：pub trait DerefMut: Deref
T: DerefMut<Target=U> 解读：将 &mut T 类型通过 DerefMut 特征的方法转换为 &mut U 类型，对应上例中，就是将 &mut MyBox<String> 转换为 &mut String
         */
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

fn display(s: &mut String) {
    s.push_str("world");
    println!("{}", s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disp() {
        let mut s = MyBox::new(String::from("hello "));
        display(&mut s);
        println!("s: {:?}", s)
    }
}