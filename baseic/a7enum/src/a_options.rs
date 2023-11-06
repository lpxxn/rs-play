

// test
#[cfg(test)]
mod tests {
    #[test]
    fn test_option1() {
        let some_number = Some(5);
        let some_string = Some("a string");
        let absent_number: Option<i32> = None;
        // extract value from some_number
        if let Some(i) = some_number {
            println!("some_number: {}", i);
        }
    }
    #[derive(Debug)]
    enum Number {
        Zero = 1,
        One,
    }

    #[test]
    fn number_test() {
        println!("zero is {:?}", Number::Zero);
        println!("zero is {}", Number::Zero as i32);
        println!("one is {}", Number::One as i32);
    }
}