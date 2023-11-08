fn main() {
    matches_macro();

    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North => { // 在这里匹配 South 或 North
            println!("South or North");
        }
        _ => println!(""),
    };

    let msgs = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
        Message::Write("hello".to_string())
    ];

    for msg in &msgs {
        show_message(msg)
    }
    println!("{:?}", msgs);

    run_print_coordinates();
    at_test();
    mut_v();
}

enum Direction {
    East,
    West,
    North,
    South,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn show_message(msg: &Message) {
    match msg {
        Message::Move { x: a, y: b } => {
            println!("Move x: {}, y: {}", a, b)
        }
        Message::ChangeColor(_, a, b) => {
            println!("ChangeColor a: {}, b: {}", a, b)
        }
        Message::Write(ref str) => {
            println!("Write str: {}", str)
        }
        _ => println!("no match"),
    }
}


// Rust 标准库中提供了一个非常实用的宏：matches!，它可以将一个表达式跟模式进行匹配，然后返回匹配的结果 true or false。
fn matches_macro() {
    #[derive(Debug)]
    enum MyEnum {
        Foo,
        Bar,
    }

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];

    println!("v: {:#?}", v);
    // 但是，实际上这行代码会报错，因为你无法将 x 直接跟一个枚举成员进行比较。
    // 好在，你可以使用 match 来完成，但是会导致代码更为啰嗦，是否有更简洁的方式？答案是使用 matches!：
    // let r = v.iter()
    //     .filter(|x| x == MyEnum::Foo);

    let r = v.iter()
        .filter(|&x| matches!(x, MyEnum::Foo)).collect::<Vec<_>>();
    println!("r: {:#?}, v: {:#?}", r, v);

    let foo = 'f';
    assert!(matches!(foo, 'a'..='z' | 'A'..='Z'));

    let bar = Some(3);
    assert!(matches!(bar, Some(1..=5)));
    assert!(matches!(bar, Some(x) if x > 2));

    if let Some(x) = bar {
        println!("x: {}", x);
    }
}

//  &(x, y) 这里加上 & 是因为 match 语句会获取所有权，而我们只是想要借用元组中的值而已。
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})， address {:p}, {:p}", x, y, &x, &y);
}

fn print_coordinates2((x, y): &(String, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn run_print_coordinates() {
    let point = (String::from("hello world"), 5);
    print!("point: {:?}, str: {}", point, point.0);
    print_coordinates2(&point);
    println!("point: {:?}, str: {}", point, point.0);

    let point = (3, 5);
    println!("point: {:?}, address: {:p} {:p} {:p}", point, &point, &point.0, &point.1);
    // &(3, 5) 会匹配模式 &(x, y)，因此 x 得到了 3，y 得到了 5。
    print_coordinates(&point);


// 匹配守卫（match guard）是一个位于 match 分支模式之后的额外 if 条件，它能为分支模式提供更进一步的匹配条件。
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

}

fn at_test() {

    /*
    @绑定
@（读作 at）运算符允许为一个字段绑定另外一个变量。下面例子中，我们希望测试 Message::Hello 的 id 字段
是否位于 3..=7 范围内，同时也希望能将其值绑定到 id_variable 变量中以便此分支中相关的代码可以使用它。
我们可以将 id_variable 命名为 id，与字段同名，不过出于示例的目的这里选择了不同的名称。
     */
    enum Message {
        Hello { id: i32 }
    }
    // 当你既想要限定分支范围，又想要使用分支的变量时，就可以用 @ 来绑定到一个新的变量上，实现想要的功能。
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: a @ 3..=7 } => {
            println!("id in range: {}", a);
        },
        Message::Hello { id: 10..=12 } => {
            println!("id in another range");
        },
        Message::Hello { id } => {
            println!("id: {}", id);
        },
    }

    // 使用 @ 还可以在绑定新变量的同时，对目标进行解构：
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    // 绑定新变量 `p`，同时对 `Point` 进行解构
    let p @ Point {x: px, y: py } = Point {x: 10, y: 23};
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);


    let point = Point {x: 10, y: 5};
    if let p @ Point {x: 10, y} = point {
        println!("x is 10 and y is {} in {:?}", y, p);
    } else {
        println!("x was not 10 :(");
    }

    match 1 {
        num @ 1..=3 => {
            println!("num: {}", num)
        }
        _ => {}
    }
}

fn mut_v() {
    let mut v = "hello".to_string();
    let r = &mut v;
    match r {
        // The type of value is &mut String
        value => {
            value.push_str("world");
        }
    }
    println!("v: {}", v);
}

