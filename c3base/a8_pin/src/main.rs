fn main() {
    let mut test1 = Test::new(1);
    // test1的b字段指向test1的a字段,保存的是a字段的地址
    test1.init();
    println!("test1: {:?} a: {}, b: {}", test1, test1.a(), test1.b());

    let mut test2 = Test::new(2);
    // test2的b字段指向test2的a字段,保存的是a字段的地址
    test2.init();
    println!("test2: {:?} a: {}, b: {}", test2, test2.a(), test2.b());
    // swap交换test1和test2的值,b字段指向的地址也发生了交换，但是a字段的地址没有发生交换，只是交换了值
    // 所以 test1的b字段指向的地址是test2的a字段的地址，
    // test2的b字段指向的地址是test1的a字段的地址
    std::mem::swap(&mut test1, &mut test2);
    test2.a = 4;
    println!("test1: {:?} a: {}, b: {}", test1, test1.a(), test1.b());
    println!("test2: {:?} a: {}, b: {}", test2, test2.a(), test2.b());
}

#[derive(Debug)]
struct Test {
    a: i32,
    b: *const i32,
}

impl Test {
    fn new(v: i32) -> Self {
        Test {
            a: v,
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let self_ref: *const i32 = &self.a;
        self.b = self_ref;
    }

    fn a(&self) -> &i32 {
        &self.a
    }

    fn b(&self) -> &i32 {
        assert!(!self.b.is_null(), "Test: b called without init");
        unsafe { &*self.b }
    }
}