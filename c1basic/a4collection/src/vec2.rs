/*
在 rust 里，实现了两种排序算法，分别为稳定的排序 sort 和 sort_by，以及非稳定排序 sort_unstable 和 sort_unstable_by。
当然，这个所谓的 非稳定 并不是指排序算法本身不稳定，而是指在排序过程中对相等元素的处理方式。在 稳定 排序算法里，对相等的元素，不会对其进行重新排序。而在 不稳定 的算法里则不保证这点。
总体而言，非稳定 排序的算法的速度会优于 稳定 排序算法，同时，稳定 排序还会额外分配原数组一半的空间。
 */
fn sort_vector() {
    let mut v = vec![1, 5, 10, 2, 15];
    v.sort();
    println!("v: {:?}", v);

    let mut v = vec![1, 5, 10, 2, 15];
    v.sort_unstable();
    println!("v: {:?}", v);

    let mut v = vec![1, 5, 10, 2, 15];
    v.sort_by(|a, b| b.cmp(a));
    println!("v: {:?}", v);

    let mut v = vec![1, 5, 10, 2, 15];
    v.sort_unstable_by(|a, b| b.cmp(a));
    println!("v: {:?}", v);

    // 对比浮点数
    let mut v = vec![1.0, 5.0, 10.0, 2.0, 15.0];
    // 浮点数不能使用 cmp，只能使用 partial_cmp
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("v: {:?}", v);
}

// 结构体排序
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
}

/*
从上面我们学习过程当中，排序需要我们实现 Ord 特性，那么如果我们把我们的结构体实现了该特性，是否就不需要我们自定义对比函数了呢？
是，但不完全是，实现 Ord 需要我们实现 Ord、Eq、PartialEq、PartialOrd 这些属性。好消息是，你可以 derive 这些属性：
需要 derive Ord 相关特性，需要确保你的结构体中所有的属性均实现了 Ord 相关特性，否则会发生编译错误。derive 的默认实现会依据属性的顺序依次进行比较，如上述例子中，当 Person 的 name 值相同，则会使用 age 进行比较。
 */

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Person1 {
    name: String,
    age: u32,
}

impl Person1 {
    fn new(name: String, age: u32) -> Self {
        Self {
            name,
            age,
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort_vector() {
        sort_vector();
    }

    #[test]
    fn test_person1() {
        let mut people = vec![
            Person::new("ZhangSan", 20),
            Person::new("LiSi", 18),
            Person::new("WangWu", 19),
            Person::new("Ahaoliu", 18),
        ];

        // 按照年龄排序
        people.sort_by(|a, b| a.age.cmp(&b.age));
        println!("people: {:?}", people);
        people.sort_unstable_by({ |a, b| a.age.cmp(&b.age) });
        println!("people: {:?}", people);
    }

    #[test]
    fn test_person2() {
        let mut people = vec![
            Person1::new("ZhangSan".to_string(), 20),
            Person1::new("LiSi".to_string(), 18),
            Person1::new("LiSi".to_string(), 19),
            Person1::new("WangWu".to_string(), 19),
            Person1::new("Ahaoliu".to_string(), 18),
        ];
        people.sort_unstable();
        println!("people: {:?}", people);
    }

    #[test]
    fn test_person3() {
        let mut people = vec![
            Person1::new("ZhangSan".to_string(), 20),
            Person1::new("LiSi".to_string(), 18),
            Person1::new("LiSi".to_string(), 19),
            Person1::new("WangWu".to_string(), 19),
            Person1::new("Ahaoliu".to_string(), 18),
        ];
        people.sort();
        println!("people: {:?}", people);
    }
}