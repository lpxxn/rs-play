fn address_2_pointer() {
    let a = 1;
    let b = &a;
    let c = &b;
    println!("a: {:p}", &a);
    println!("b: {:p}", b);
    println!("c: {:p}", c);
    println!("c: {:p}", *c);
}

fn add_2_pointer() {
    let mut values: [i32; 2] = [1, 2];
    println!("value address: {:p}", &values);
    println!("value 1 address: {:p}", &values[0]);
    println!("value 2 address: {:p}", &values[1]);
    let p1: *mut i32 = values.as_mut_ptr(); // as_mut_ptr() 返回一个指向数组第一个元素的指针
    println!("p1: {:?}", p1);
    println!("p1: {:p}", &p1);
    let first_address = p1 as usize; // 将p1内存地址转换为usize类型
    println!("int32 size: {}", std::mem::size_of::<i32>()); // 打印i32类型的字节大小
    println!("first_address: {:?}", first_address);

    let second_address = first_address + std::mem::size_of::<i32>(); // 计算第二个元素的内存地址
    println!("second_address: {:?}", second_address);
    println!("second_address: {:p}", second_address as *mut i32); // 将usize类型的内存地址转换为指向i32类型的指针

    let p2: *mut i32 = second_address as *mut i32; // 将usize类型的内存地址转换为指向i32类型的指针
    println!("p2 value: {:?}", p2);
    println!("p2 address: {:p}", &p2);
    unsafe {
        *p2 = 3; // 通过指针修改数组的第二个元素
    }
    println!("values: {:?}", values);
}


fn do_stuff<T: Clone>(value: &T) {
    let cloned = value.clone();
}
/*
上面例子中 cloned 的类型是什么？首先编译器检查能不能进行值方法调用， value 的类型是 &T，同时 clone 方法的签名也是 &T ： fn clone(&T) -> T，因此可以进行值方法调用，再加上编译器知道了 T 实现了 Clone，因此 cloned 的类型是 T。

如果 T: Clone 的特征约束被移除呢？
*/
fn do_stuff2<T>(value: &T) {
    let cloned = value.clone();
}
/*
首先，从直觉上来说，该方法会报错，因为 T 没有实现 Clone 特征，但是真实情况是什么呢？

我们先来推导一番。 首先通过值方法调用就不再可行，因为 T 没有实现 Clone 特征，也就无法调用 T 的 clone 方法。
接着编译器尝试引用方法调用，此时 T 变成 &T，在这种情况下， clone 方法的签名如下： fn clone(&&T) -> &T，接着我们现在对 value 进行了引用。
编译器发现 &T 实现了 Clone 类型(所有的引用类型都可以被复制，因为其实就是复制一份地址)，因此可以推出 cloned 也是 &T 类型。
最终，我们复制出一份引用指针，这很合理，因为值类型 T 没有实现 Clone，只能去复制一个指针了。
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_2_pointer() {
        address_2_pointer();
    }

    #[test]
    fn test_add_2_pointer() {
        add_2_pointer();
    }
}