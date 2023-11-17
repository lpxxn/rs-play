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

// 生命周期省略规则
/*
对于 first_word 函数，它的返回值是一个引用类型，那么该引用只有两种情况：
从参数获取
从函数体内部新创建的变量获取
如果是后者，就会出现悬垂引用，最终被编译器拒绝，因此只剩一种情况：返回值的引用是获取自参数，这就意味着参数和返回值的生命周期是一样的。
rust 1.0版本之前，是需要手动指定的 `fn first_word<'a>(s: &'a str) -> &'a str {`
 */
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() { // enumerate() 方法将一个可迭代的序列（比如一个数组）变成一个迭代器，这个迭代器会返回元素的索引和元素值
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
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