fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

#[cfg(test)]
mod test {
    use crate::l1::ImportantExcerpt;

    #[test]
    fn test_f1() {
        let string1 = String::from("abcd");
        let string2 = "xyz";
        let result = super::longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
        println!("string1: {}", string1);
        println!("string2: {}", string2);
    }

    #[test]
    fn test_struct1() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        println!("first_sentence: {}", first_sentence);
        let i = ImportantExcerpt { part: first_sentence };

        println!("i.part: {}", i.part);
    }
}