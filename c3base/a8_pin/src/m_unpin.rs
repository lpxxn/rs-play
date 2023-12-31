use std::marker::PhantomPinned;
use std::pin::Pin;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}


impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned, // 这个标记可以让编译器知道这个结构体是可以pin的，会自动实现特征 `!Unpin`
        }
    }

    fn init(self: Pin<&mut Self>) {
        let self_ptr: *const String = &self.a;
        let this = unsafe { self.get_unchecked_mut() };
        this.b = self_ptr
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        assert!(!self.b.is_null(), "Test: b called without init");
        unsafe { &*(self.b) }
    }
}
/*
上面代码中，我们使用了一个标记类型 PhantomPinned 将自定义结构体 Test 变成了 !Unpin (编译器会自动帮我们实现)，因此该结构体无法再被移动。
一旦类型实现了 !Unpin ，那将它的值固定到栈( stack )上就是不安全的行为，因此在代码中我们使用了 unsafe 语句块来进行处理，
你也可以使用 pin_utils 来避免 unsafe 的使用。
 */

#[cfg(test)]
mod tests {
    use std::iter::from_fn;
    use super::*;

    #[test]
    fn test_pin() {
        // 此时test1可以被安全的移动
        let mut test1 = Test::new("test1");
        // 新的test1由于使用了Pin,因此不能被移动，这埋在的声明会将之前的test1遮蔽掉(shadow)
        let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
        Test::init(test1.as_mut());
        println!("test1: {:?} a: {}, b: {}", test1, Test::a(test1.as_ref()), Test::b(test1.as_ref()));

        let mut test2 = Test::new("test2");
        let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
        Test::init(test2.as_mut());
        println!("test2: {:?} a: {}, b: {}", test2, Test::a(test2.as_ref()), Test::b(test2.as_ref()));
        // 再去尝试移动被固定的值，就会导致编译错误：
        // 编译期间就会报错
        // std::mem::swap(&mut test1.get_mut(), &mut test2.get_mut());
        println!("after swap test1: {:?} a: {}, b: {}", test1, Test::a(test1.as_ref()), Test::b(test1.as_ref()));
        println!("after swap test2: {:?} a: {}, b: {}", test2, Test::a(test2.as_ref()), Test::b(test2.as_ref()));
    }
}