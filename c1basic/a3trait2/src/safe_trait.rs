/*
一个特征能变成特征对象，首先该特征必须是对象安全的，即该特征的所有方法都必须拥有以下特点：

返回类型不能是 Self.
不能使用泛型参数
 */

/*
// 使用至少两种方法让代码工作
// 不要添加/删除任何代码行
trait MyTrait {
    // 返回了 Self
    fn f(&self) -> Self;
}

impl MyTrait for u32 {
    fn f(&self) -> Self { 42 }
}

impl MyTrait for String {
    fn f(&self) -> Self { self.clone() }
}

fn my_function(x: Box<dyn MyTrait>) {
    x.f()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        my_function(Box::new(13_u32));
        my_function(Box::new(String::from("abc")));

        println!("Success!")
    }
}

 */

/*
error[E0038]: the trait `MyTrait` cannot be made into an object
  --> src/safe_trait.rs:23:23
   |
23 | fn my_function(x: Box<dyn MyTrait>) {
   |                       ^^^^^^^^^^^ `MyTrait` cannot be made into an object
   |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> src/safe_trait.rs:12:20
   |
10 | trait MyTrait {
   |       ------- this trait cannot be made into an object...
11 |     // 返回了 Self
12 |     fn f(&self) -> Self;
   |                    ^^^^ ...because method `f` references the `Self` type in its return type
   = help: consider moving `f` to another trait
 */

trait MyTrait1 {
    fn f(&self) -> Self; // 如果返回Self，那么就不是对象安全的, my_function方法参数使用Box<dyn MyTrait>就会报错
}

impl MyTrait1 for u32 {
    fn f(&self) -> Self { 42 }
}

impl MyTrait1 for String {
    fn f(&self) -> Self { self.clone() }
}

fn my_function1(x: impl MyTrait1) -> impl MyTrait1 {
    x.f()
}

#[cfg(test)]
mod test2 {
    use super::*;

    #[test]
    fn test_1() {
        my_function1(13_u32);
        my_function1(String::from("abc"));
        println!("Success!")
    }
}

trait MyTrait2 {
    fn f(&self) -> Box<dyn MyTrait2>;
}

impl MyTrait2 for u32 {
    fn f(&self) -> Box<dyn MyTrait2> { Box::new(42) }
}

impl MyTrait2 for String {
    fn f(&self) -> Box<dyn MyTrait2> { Box::new(self.clone()) }
}

fn my_function2(x: Box<dyn MyTrait2>) -> Box<dyn MyTrait2> {
    x.f()
}

#[cfg(test)]
mod test3 {
    use super::*;

    #[test]
    fn test_1() {
        my_function2(Box::new(13_u32));
        my_function2(Box::new(String::from("abc")));
        println!("Success!")
    }
}