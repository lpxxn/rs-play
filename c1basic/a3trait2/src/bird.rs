trait Bird {
    fn quack(&self);
    fn fly(&self) {
        println!("I'm flying");
    }
}

struct Duck;

impl Bird for Duck {
    fn quack(&self) {
        println!("quack");
    }
    fn fly(&self) {
        println!("I'm can't fly");
    }
}

struct Swan;

impl Bird for Swan {
    fn quack(&self) {
        println!("swan quack");
    }
}

fn fly(bird: &dyn Bird) {
    bird.fly();
}

fn quack(bird: Box<dyn Bird>) {
    bird.quack();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bird() {
        let duck = Duck;
        duck.quack();
        duck.fly();

        let swan = Swan;
        swan.quack();
        swan.fly();
        fly(&swan);
        quack(Box::new(swan));

        let birds: Vec<Box<dyn Bird>> = vec![Box::new(Duck), Box::new(Swan)];

        for bird in birds.iter() {
            bird.fly();
            bird.quack();
            // bird is a &Box<dyn Bird>
            // cast from &Bird to &dyn Bird
            // & is a pointer to a trait object
            // * is a trait object ** is a concrete object
            fly(&**bird);
        }

        duck.quack();
        duck.fly();
    }
}
