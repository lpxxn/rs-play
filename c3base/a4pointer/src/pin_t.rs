use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;

// 下面是一个自引用结构体的例子，包含了一个 `String` 和一个指向它的 `slice`。
// 我们无法使用普通引用来实现，因为违背了rust的编译规则
// 因此，这里我们使用了一个裸指针，通过NonNull来确保它不会为null
#[derive(Debug)]
struct Unmovable {
    data: String,
    slice: NonNull<String>,
    _pin: PhantomPinned,
}

impl Unmovable {
    fn new(data: String) -> Pin<Box<Self>> {
        let res = Unmovable {
            data,
            // 只有在数据到位时，才创建指针，否则数据会在开始之前就被转移所有权
            slice: NonNull::dangling(),
            _pin: PhantomPinned,
        };

        let mut boxed = Box::pin(res);
        let slice = NonNull::from(&boxed.data);
        // SAFETY: 我们只在数据到位时创建了一个指向它的指针
        unsafe {
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).slice = slice;
        }
        boxed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let unmoved = Unmovable::new("hello".to_string());
        println!("unmoved: {:?}", unmoved);
        // 只要结构体被固定在内存中，那么它的引用就是有效的
        let mut still_unmoved = unmoved;
        assert_eq!(still_unmoved.slice, NonNull::from(&still_unmoved.data));

        // // 因为我们没有实现 `Unpin`，所以下面这行代码是不能编译的
        // let mut new_unmoved = Unmovable::new("world".to_string());
        // println!("new_unmoved: {:?}", new_unmoved);
        // std::mem::swap(&mut *still_unmoved, &mut *new_unmoved);
    }
}