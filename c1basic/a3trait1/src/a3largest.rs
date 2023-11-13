
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[cfg(test)]
mod test_largest {
    use super::*;

    #[test]
    fn test_largest() {
        let num_list = [34, 50, 25, 100, 65];
        let result = largest(&num_list);
        println!("The largest number is {}", result);
    }
}