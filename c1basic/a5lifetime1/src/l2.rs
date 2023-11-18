struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }

    /*
    impl<'a: 'b, 'b> ImportantExcerpt<'a>
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str
    a: 'b，是生命周期约束语法，跟泛型约束非常相似，用于说明 'a 必须比 'b 活得久
可以把 'a 和 'b 都在同一个地方声明（如上），或者分开声明但通过 where 'a: 'b 约束生命周期关系
     */
    fn announce_and_return_part1<'b>(&self, announcement: &'b str) -> &'b str
        where 'a: 'b {
        println!("Attention please: {}", announcement);
        self.part
    }
}

mod test {
    use std::fmt::Display;
    use super::*;

    #[test]
    fn test_struct1() {
        let first_sentence = "Call me Ishmael. Some years ago...";
        let i = ImportantExcerpt { part: first_sentence };
        println!("i.part: {}", i.announce_and_return_part("hello"));
        println!("i.part: {}", i.announce_and_return_part1("hello"));
        println!("i.part: {}", i.part);
    }

    #[test]
    fn test_struct2() {
        let r1;
        let r2;
        {
            static STATIC_EXAMPLE: i32 = 42;
            r1 = &STATIC_EXAMPLE;
            let x = "&'static str";
            r2 = x;
            // r1 和 r2 持有的数据都是 'static 的，因此在花括号结束后，并不会被释放
        }

        println!("&'static i32: {}", r1); // -> 42
        println!("&'static str: {}", r2); // -> &'static str

        //let r3: &str;

        {
            let s1 = "String".to_string();

            // s1 虽然没有 'static 生命周期，但是它依然可以满足 T: 'static 的约束
            // 充分说明这个约束是多么的弱。。
            static_bound(&s1);

            // s1 是 String 类型，没有 'static 的生命周期，因此下面代码会报错
            //r3 = &s1;

            // s1 在这里被 drop
        }
        //println!("{}", r3);
    }

    // 原因在于我们约束的是 T，但是使用的却是它的引用 &T，换而言之，我们根本没有直接使用 T，因此编译器就没有去检查 T 的生命周期约束！它只要确保 &T 的生命周期符合规则即可
    fn static_bound<T: Display + 'static>(t: &T) {
        println!("{}", t);
    }
    fn static_bound2<T: Display + 'static>(t: T) {
        println!("{}", t);
    }
}