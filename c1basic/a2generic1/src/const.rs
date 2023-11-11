fn display_array(arr: &[i32]) {
    println!("arr: {:?}", arr);
}

// 要注意的是需要对 T 加一个限制 std::fmt::Debug，该限制表明 T 可以用在 println!("{:?}", arr) 中，因为 {:?} 形式的格式化输出需要 arr 实现该特征。
fn display_array2<T: std::fmt::Debug>(arr: &[T]) {
    println!("generic arr: {:?}", arr);
}

// const 泛型，也就是针对值的泛型，正好可以用于处理数组长度的问题：
// N 就是 const 泛型，定义的语法是 const N: usize，表示 const 泛型 N ，它基于的值类型是 usize。
fn display_array3<T: std::fmt::Debug, const N: usize>(arr: &[T; N]) {
    println!("generic arr: {:?}", arr);
}

#[cfg(test)]
mod test_const {
    use super::*;

    #[test]
    fn test_display_array() {
        let arr: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        display_array(&arr);
        display_array2(&arr);
        display_array3(&arr);

        let arr2: [i32; 2] = [1, 8];
        display_array(&arr2);
        let arr: [f32; 3] = [1.1, 2.2, 3.3];
        display_array2(&arr);
        display_array3(&arr);
    }
}