#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    fn a(&self) -> &str {
        &self.a
    }

    fn b(&self) -> &str {
        assert!(!self.b.is_null(), "Test: b called without init");
        unsafe { &*self.b }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut test1 = Test::new("test1");
        // test1的b字段指向test1的a字段,保存的是a字段的地址
        test1.init();
        println!("test1: {:?} a: {}, b: {}", test1, test1.a(), test1.b());

        let mut test2 = Test::new("test2");
        // test2的b字段指向test2的a字段,保存的是a字段的地址
        test2.init();
        println!("test2: {:?} a: {}, b: {}", test2, test2.a(), test2.b());
        // swap交换test1和test2的值,b字段指向的地址也发生了交换，但是a字段的地址没有发生交换，只是交换了值
        // 所以 test1的b字段指向的地址是test2的a字段的地址，
        // test2的b字段指向的地址是test1的a字段的地址
        std::mem::swap(&mut test1, &mut test2);
        test2.a = String::from("hahahah");
        println!("test1: {:?} a: {}, b: {}", test1, test1.a(), test1.b());
        println!("test2: {:?} a: {}, b: {}", test2, test2.a(), test2.b());
    }
}
