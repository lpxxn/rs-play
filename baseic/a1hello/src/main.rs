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
    format_test();
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

fn format_test() {
    println!("{1}{0}{1}", 1, 2);
    println!("{a}{b}{a}", a = 1, b = 2);
    println!("{:b}", 2);

    let v = 3.1415926;
    // Display => 3.14
    println!("{:.2}", v);
    // Debug => 3.14
    println!("{:.2?}", v);


    //---------字符串填充--------------------------
    // 以下全部输出 "Hello x    !"
    // 为"x"后面填充空格，补齐宽度5
    println!("Hello {:5}!", "x");
    // 使用参数5来指定宽度
    println!("Hello {:1$}!", "x", 5);
    // 使用x作为占位符输出内容，同时使用5作为宽度
    println!("Hello {1:0$}!", 5, "x");
    // 使用有名称的参数作为宽度
    println!("Hello {:width$}!", "x", width = 5);
    //-----------------------------------

    // 使用参数5为参数x指定宽度，同时在结尾输出参数5 => Hello x    !5
    println!("Hello {:1$}!{}", "x", 5);

    //---------数字串填充--------------------------
    // 宽度是5 => Hello     5!
    println!("Hello {:5}!", 5);
    // 显式的输出正号 => Hello +5!
    println!("Hello {:+}!", 5);
    // 宽度5，使用0进行填充 => Hello 00005!
    println!("Hello {:05}!", 5);
    // 负号也要占用一位宽度 => Hello -0005!
    println!("Hello {:05}!", -5);

    //---------对齐--------------------------
    // 以下全部都会补齐5个字符的长度
    // 左对齐 => Hello x    !
    println!("Hello {:<5}!", "x");
    // 右对齐 => Hello     x!
    println!("Hello {:>5}!", "x");
    // 居中对齐 => Hello   x  !
    println!("Hello {:^5}!", "x");

    // 对齐并使用指定符号填充 => Hello x&&&&!
    // 指定符号填充的前提条件是必须有对齐字符
    println!("Hello {:&<5}!", "x");


    // 精度
    let v = 3.1415926;
    // 保留小数点后两位 => 3.14
    println!("{:.2}", v);
    // 带符号保留小数点后两位 => +3.14
    println!("{:+.2}", v);
    // 不带小数 => 3
    println!("{:.0}", v);
    // 通过参数来设定精度 => 3.1416，相当于{:.4}
    println!("{:.1$}", v, 4);

    let s = "hi我是Sunface孙飞";
    // 保留字符串前三个字符 => hi我
    println!("{:.3}", s);
    // {:.*}接收两个参数，第一个是精度，第二个是被格式化的值 => Hello abc!
    println!("Hello {:.*}!", 3, "abcdefg");

    /*
    进制
可以使用 # 号来控制数字的进制输出：

#b, 二进制
#o, 八进制
#x, 小写十六进制
#X, 大写十六进制
x, 不带前缀的小写十六进制
     */
    // 二进制 => 0b11011!
    println!("{:#b}!", 27);
    // 八进制 => 0o33!
    println!("{:#o}!", 27);
    // 十进制 => 27!
    println!("{}!", 27);
    // 小写十六进制 => 0x1b!
    println!("{:#x}!", 27);
    // 大写十六进制 => 0x1B!
    println!("{:#X}!", 27);

    // 不带前缀的十六进制 => 1b!
    println!("{:x}!", 27);

    // 使用0填充二进制，宽度为10 => 0b00011011!
    println!("{:#010b}!", 27);

    // 指针地址
    let v= vec![1, 2, 3];
    println!("{:p}", v.as_ptr()); // => 0x600002324050

    println!("-----------在格式化字符串时捕获环境中的值（Rust 1.58 新增）---------");
    let person = get_person();
    println!("Hello, {person}!");
}

fn get_person() -> String {
    String::from("sunface")
}

