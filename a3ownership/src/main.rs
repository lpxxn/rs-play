fn main() {
    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
    //println!("s: {}", s);                        // ... 所以到这里不再有效

    let x = 5; // x 进入作用域
    makes_copy(x);

    println!("{}", x)
}


fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // some_string 移出作用域，drop 函数被调用，内存释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // some_integer 移出作用域，不会有特殊操作

