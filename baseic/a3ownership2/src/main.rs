use std::fmt::format;

fn main() {
    basic();
    no_change();
    change_value();
    change_i32();
    ref_test();
}

fn basic() {
    let x = 5;
    let y = &x;
    println!("x: {}, y: {:p} *y: {}", x, y, *y);
    // let address = y as *const _; //或者
    let address = y as *const i32;
    println!("address: {:?}", address);
}

// 不可变引用
fn no_change() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// 可变引用
fn change_value() {
    let mut s = String::from("hello");
    change_str(&mut s);
    change_str2(&mut s);
    println!("s: {}", s);
}

fn change_str(s: &mut String) {
    s.push_str(", world");
}

fn change_str2(ref mut s: &mut String) {
    s.push_str(", ref change 2");
}

fn change_i32() {
    let mut x = 5;
    println!("origin x: {}", x);
    change_i321(&mut x);
    println!("x: {}", x);
    change_i322(x);
    println!("x: {}", x);
}

fn change_i321(x: &mut i32) {
    *x += 1;
}

fn change_i322(ref mut x: i32) {
    *x += 2;
}

// dangling references 悬垂引用，也叫悬垂指针，指针指向一个值后，这个值被释放了，而这个指针还在使用，他指向的内存可能不存任何值了或已经被其他变量使用了
// 在rust中编译器会阻止这种情况的发生，编译器可以确保数据不会在引用结束前被释放，如果想要释放，必须先停止引用
// 因为 s 是在 dangle 函数内创建的，当 dangle 的代码执行完毕后，s 将被释放，但是此时我们又尝试去返回它的引用。这意味着这个引用会指向一个无效的 String，这可不对！
// fn dangling() -> &String {// dangle 返回一个字符串的引用
//     let s = String::from("hello");
//     // &s // this function's return type contains a borrowed value, but there is no value for it to be borrowed from 该函数返回了一个借用的值，但是已经找不到它所借用值的来源
// } // 这里 s 离开作用域并被丢弃。其内存被释放。

// 这样就没有任何错误了，最终 String 的 所有权被转移给外面的调用者。
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

// ref
fn ref_test() {
    let c = '中';
    let r1 = &c;
    let ref r2 = c;
    assert_eq!(*r1, *r2);
    assert_eq!(get_addr(r1), get_addr(r1));
}

fn get_addr(r: &char) -> String {
    let a = format!("{:p}", r);
    println!("addr: {}", a);
    a
}