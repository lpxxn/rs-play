pub struct Cat {
    pub name: String,
    pub age: u8,
}

impl Cat {
    pub fn new(name: String, age: u8) -> Cat {
        Cat { name, age }
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn print_info(&self) {
        println!("name: {}, age: {}", self.name, self.age);
    }
}