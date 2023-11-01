// use std::any::type_name;

fn main() {
    flow_add();
    compare_float();
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
    let b: f64 = 0.3;
    println!("compare {}", a == b);
    println!("compare {}", (a - b).abs() <= std::f64::EPSILON);
}