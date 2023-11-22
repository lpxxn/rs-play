pub mod user {
    pub struct User {
        pub name: String,
        pub age: u8,
    }

    impl User {
        pub fn new(name: String, age: u8) -> User {
            User { name, age }
        }
        pub fn name(&self) -> &str {
            &self.name
        }
    }
}

pub fn add(x: i32, y: i32) -> i32 {
    x + y
}