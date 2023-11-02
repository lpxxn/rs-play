// use std::any::type_name;

fn main() {
    flow_add();
    compare_float();
    char_size();
}

/*
使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理，例如 wrapping_add
如果使用 checked_* 方法时发生溢出，则返回 None 值
使用 overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
使用 saturating_* 方法使值达到最小值或最大值
 */

fn flow_add() {
    let a: u8 = 255;
    let b = a.wrapping_add(1);
    println!("{}", b);
}

// 浮点数比较
fn compare_float() {
    let a: f64 = 0.1 + 0.2;
    let b = 0.3_f64;
    println!("compare {}", a == b);
    f32::EPSILON;
    println!("compare {}", (a - b).abs() <= f64::EPSILON);
}

fn char_size() {
    let a = 'a';
    println!("char size: {}", std::mem::size_of_val(&a));
    let a = '中';
    println!("char size: {}", std::mem::size_of_val(&a));
}

fn statement_or_expression() -> i32 {
    let a = 1;
    let b: Vec<f64> = Vec::new();
    // 上面的都是语句，他们完成了一个具体的操作，但是没有返回值。


    5 + 7 // 表达式会进行计算，并返回一个值
    // 表达式可以成为语句的一部分，例如 let y = 6 中，6 就是一个表达式，它在求值后返回一个值 6（有些反直觉，但是确实是表达式）
}

// ! 表示一个永远不会返回的类型，例如 panic! 宏
fn for_loop()  -> ! {
    let a = [1, 2, 3, 4, 5];
    for i in a.iter() {
        println!("{}", i);
    }
}