pub mod abc {
    pub fn hello() {
        println!("hello");
    }
}

#[cfg(test)]
mod test_large {
    /*
使用泛型参数，有一个先决条件，必需在使用前对其进行声明：

fn largest<T>(list: &[T]) -> T {
该泛型函数的作用是从列表中找出最大的值，其中列表中的元素类型为 T。首先 largest<T> 对泛型参数 T 进行了声明，
然后才在函数参数中进行使用该泛型参数 list: &[T] （还记得 &[T] 类型吧？这是数组切片）。
 */

    // fn largest<T: PartialOrd>(list: &[T]) -> T {
    //     let mut largest = &list[0];
    //
    //     for &item in list.iter() {
    //         if item > *largest {
    //             largest = &item;
    //         }
    //     }
    // }
    //
    //
    // #[test]
    // fn test_largest() {
    //     let num_list = [34, 50, 25, 100, 65];
    //     let result = largest(&num_list);
    //     println!("The largest number is {}", result);
    // }
}
