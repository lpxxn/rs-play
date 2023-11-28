use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/*
* 背后的原理
当我们对智能指针 Box 进行解引用时，实际上 Rust 为我们调用了以下方法：
*(y.deref())
首先调用 deref 方法返回值的常规引用，然后通过 * 对常规引用进行解引用，最终获取到目标值
如果 deref 方法直接返回一个值，而不是引用，那么该值的所有权将被转移给调用者，而我们不希望调用者仅仅只是 *T 一下，就拿走了智能指针中包含的值。
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str() {
        let s = "hello".to_string();
        let s1 = s.deref();
        println!("s1: {:?}", s1);
        display(&s);
        /*
        String 实现了 Deref 特征，可以在需要时自动被转换为 &str 类型
&s 是一个 &String 类型，当它被传给 display 函数时，自动通过 Deref 转换成了 &str
必须使用 &s 的方式来触发 Deref(仅引用类型的实参才会触发自动解引用)
         */
    }

    fn display(s: &str) {
        println!("s: {:?}", s);
    }

    #[test]
    fn test_mybox1() {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
        let y1 = *y;
        assert_eq!(5, y1);
        let y2 = y.deref();
        println!("y2: {:?}", y2);
        println!("y2 address: {:p}", y2);
        println!("y address: {:p}", &y);
    }
}

/*
impl<T: ?Sized> Deref for &T {
    type Target = T;

    fn deref(&self) -> &T {
        *self
    }
}
在这段源码中，&T 被自动解引用为 T，也就是 &T: Deref<Target=T> 。 按照这个代码，&&&&T 会被自动解引用为 &&&T，然后再自动解引用为 &&T，以此类推， 直到最终变成 &T。
Self = &T  ----> &self = &&T ----> *self = &T
 */