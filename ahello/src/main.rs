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
    reassign();
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

fn reassign() {
    let x = 5;
    let x = x +1;
    {
        let x = x * 2;
        println!("The value of x is :{}", x);
    }

    println!("The value of x is :{}", x);

    let spaces: &str = "   ";
    let spaces: usize= spaces.len();
    println!("The value of spaces is :{}", spaces);

    // let mut spaces = "   ";
    // spaces = spaces.len(); // expected `&str`, found `usize`
    /*
    这和 mut 变量的使用是不同的，第二个 let 生成了完全不同的新变量，
    两个变量只是恰好拥有同样的名称，涉及一次内存对象的再分配 ，
    而 mut 声明的变量，可以修改同一个内存地址上的值 
    */
}