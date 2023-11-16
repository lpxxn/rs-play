#[derive(Debug)]
struct Container(i32, i32);

trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool;
    fn first(&self) -> i32;
}

trait Contains2 {
    type A;
    type B;
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, n1: &i32, n2: &i32) -> bool {
        let a = &self.0;
        println!("a: {:?}", a);
        // 对比两个引用时, 会自动解引用，所以这里不需要 *
        // 哪些类型可以自动解引用呢？只有实现了 Deref trait 的类型可以自动解引用。
        let x = 5;
        let x_ref = &x;
        println!("x_ref: {}", *x_ref); // Dereference explicitly use *
        println!("x_ref: {}", x_ref); // auto dereference

        (&self.0 == n1) && (&self.1 == n2)
    }

    fn first(&self) -> i32 {
        self.0
    }
}


fn difference<A, B, C: Contains<A, B>>(container: &C) -> i32 {
    container.first() - 1
}

fn difference2<C: Contains2>(container: &C) -> i32 {
    container.first() - 1
}

#[derive(Debug)]
struct Container2(i32, i32);

impl Contains2 for Container2 {
    type A = i32;
    type B = i32;
    // 只可以这样
    // fn contains(&self, n1: i32, n2: i32) -> bool {
    // 也可以 这样
    fn contains(&self, n1: &Self::A, n2: &Self::B) -> bool {
        let a = &self.0;
        println!("a: {:?}", a);
        (&self.0 == n1) && (&self.1 == n2)
    }

    fn first(&self) -> i32 {
        self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contains() {
        let n1 = 3;
        let n2 = 10;
        let container = Container(n1, n2);
        println!("Does container contain {} and {}: {}", &n1, &n2, container.contains(&n1, &n2));
        let n3 = 10;
        let n4 = 3;
        println!("Does container contain {} and {}: {}", &n4, &n3, container.contains(&n4, &n3));
        println!("First number: {}", container.first());
        println!("The difference is: {}", difference(&container));

        let container = Container2(n1, n2);
        println!("The difference2 is: {}", difference2(&container));
    }
}