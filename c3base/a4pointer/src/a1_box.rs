#[cfg(test)]
mod tests {
    #[test]
    fn test_box1() {
        let arr = vec![Box::new(1), Box::new(2)];
        let (first, second) = (&arr[0], &arr[1]);
        let sum = **first + **second;
        /*
        使用 & 借用数组中的元素，否则会报所有权错误
        表达式不能隐式的解引用，因此必须使用 ** 做两次解引用，第一次将 &Box<i32> 类型转成 Box<i32>，第二次将 Box<i32> 转成 i32
         */
    }
}