fn main() {
    println!("Hello, world!");

    let mut x = 5;
    println!("The value of x is :{}", x);
    x = 10;
    println!("The value of x is :{}", x);

    let (a, mut b): (bool, bool) = (true, true);

    println!("a = {:?}, b = {:?}", a, b);
    b = false;
    assert_eq!(b, false);

    assign_a1();
}

struct Student {
    e: i32,
    n: String,
}


fn assign_a1() {
    let (a, b, c, d ,e);

    (a, b) = (1, 2);
    // _ 表示一个值，但我们不关心它的值，和go的_一样
    [c, .., d, _] = [1, 2, 3, 4, 5];
    // ..忽略其他的值
    Student {e, ..} = Student {e: 1, n: "a".to_string()};
    // 只初始化e，忽略n
    println!("a = {:?}, b = {:?}, c = {:?}, d = {:?}, e = {:?}", a, b, c, d, e);
}
