use std::fmt::{Debug, Display};
use std::cmp::Ordering;

// where 约束
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    1
}

// 可以用where 进行改进
fn some_function2<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    1
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Pair {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
/*
cmp_display 方法，并不是所有的 Pair<T> 结构体对象都可以拥有，只有 T 同时实现了 Display + PartialOrd 的 Pair<T> 才可以拥有此方法。 该函数可读性会更好，因为泛型参数、参数、返回值都在一起，可以快速的阅读，同时每个泛型参数的特征也在新的代码行中通过特征约束进行了约束。

也可以有条件地实现特征, 例如，标准库为任何实现了 Display 特征的类型实现了 ToString 特征：
 */
impl<T: Display> ToString for Pair<T> {
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}