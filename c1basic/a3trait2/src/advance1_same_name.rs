// 同名方法
trait Pilot {
    fn fly(&self); // 这种是有&sel
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("pilot fly");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("wizard fly");
    }
}

impl Human {
    fn fly(&self) {
        println!("human fly");
    }
}

// 下面的是没有self
trait Animal {
    fn baby_name() -> String;
}

struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_same_name_method1() {
        let human = Human;
        Pilot::fly(&human);
        Wizard::fly(&human);
        human.fly();
    }

    // 没有名字的方法
    #[test]
    fn test_same_name_method2() {
        println!("A baby dog is called a {}", Dog::baby_name());
        // println!("A baby dog is called a {}", Animal::baby_name()); // 这种是错误的
        println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    }
}