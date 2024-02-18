fn main() {}

fn fn_once<F>(func: F)
//where F: FnOnce(usize) -> bool {
/* func 的类型 F 实现了 Copy 特征，调用时使用的将是它的拷贝，所以并没有发生所有权的转移。
    因为`func`的类型是没有实现`Copy`特性的 `F`，所以发生了所有权的转移
    */
    where F: FnOnce(usize) -> bool + Copy {
    println!("{}", func(1));
    println!("{}", func(2));
}

fn closure_fn<F>(f: F)
    where F: Fn(usize) -> bool {
    println!("{}", f(1));
    println!("{}", f(2));
}

#[test]
fn test_once() {
    fn_once(|x| x > 0);
}

#[test]
fn test_closure1() {
    closure_fn(|x| x > 0);
    closure_fn(|x| x > 6);
}

#[test]
fn test_closure2() {
    exec(|x| println!("{}", x));
    let mut s1 = "hello".to_string();
    let update_s1 = |str| s1.push_str(str);
    exec(update_s1);
    // 下面这个会报错，因为update_s1已经被move了
    // exec(update_s1);
    println!("{}", s1);

    // create a fnMut closure, and it can run multiple times, to set the value of s1
    let mut s2 = "world".to_string();
    // update_s2 is a closure, and it can be called multiple times, so it is FnMut
    // s2 is a mutable reference, so it is FnMut
    let mut update_s2 = |str| s2.push_str(str);
    // 如果去掉mut, 下面这个会报错,
    exec(&mut update_s2);
    exec(&mut update_s2);
    // let u = &mut update_s2;
    // exec(u);
    println!("s2: {}", s2);

    let s3 = s2;
    println!("s3: {}", s3);
}

fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
    f("hello");
    f("world");
}

#[test]
fn test_closure3() {
    let s2 = "world".to_string();
    let update_s2 = |str| {
        println!("str: {} s: {}", str, s2); // 闭包中只对s进行了不可变引用，所以可以打印，所以exec2中的闭包是Fn
    };
    exec2(update_s2);
    exec2(update_s2);
    println!("s2: {}", s2);
}

fn exec2<'a, F: Fn(&'a str)>(mut f: F) {
    f("hello");
    f("world");
}


#[test]
fn test_closure4() {
    let mut s1 = "hello".to_string();
    // let mut update_s1 = move |str| s1.push_str(str);
    let mut update_s1 = |str| s1.push_str(str);
    update_s1(" world");
    update_s1("!");
    // 如果上面写了move，下面这个会报错
    println!("{}", s1);
}

/*
一个闭包并不仅仅实现某一种 Fn 特征，规则如下：

1. 所有的闭包都自动实现了 FnOnce 特征，因此任何一个闭包都至少可以被调用一次
2. 没有移出所捕获变量的所有权的闭包自动实现了 FnMut 特征
3. 不需要对捕获变量进行改变的闭包自动实现了 Fn 特征
 */
//第二条
#[test]
fn test_closure5() {
    let mut s1 = "hello".to_string();
    let update_s1 = |str| -> String{
        s1.push_str(str);
        s1 // 闭包实现了`FnOnce`，因为它从捕获环境中移出了变量`s`
    };
    exec3(update_s1);
}

// fn exec3<F: FnMut(&str) -> String>(mut f: F) {
fn exec3<'a, F: FnOnce(&'a str) -> String>(mut f: F) {
    f(" world");
}