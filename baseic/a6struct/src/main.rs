use std::fmt;
fn main() {
    basic_struct();
    tup_struct();
    unit_struct();
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn basic_struct() {
    // 初始化实例时，每个字段都需要进行初始化
    let user1 = User {
        active: false,
        username: "".to_string(),
        email: "".to_string(),
        sign_in_count: 0,
    };
    let user2 = User {
        email: "u2".to_string(),
        ..user1 // .. 语法表明凡是我们没有显式声明的字段，全部从 user1 中自动获取。需要注意的是 ..user1 必须在结构体的尾部使用。
        // 需要注意，这时候 username 发生了所有权转移，user1 不能再使用
    };
    // println!("user1: {:?}, user2: {:?}", user1, user2);
    //                                      ^^^^^ value borrowed here after partial move  value borrow
    println!("user1: {:?}, user2: {:?}", user1.active, user2);
}

fn build_user(email: String, username: String) -> User {
    // 使用字段初始化简写语法从其他实例创建新实例
    // 当函数参数和结构体字段同名时，可以直接使用缩略的方式进行初始化
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// 无组结构体
fn tup_struct() {
    #[derive(Debug)]
    struct Point(i32, i32, i32);
    let black = Color(1, 2, 3);
    let origin = Point(0, 0, 0);
    println!("black: {}, origin: {:?}", black, origin);
    check_color(black);
}
struct Color(i32, i32, i32);

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(你好：{2}, {0}, {1})", self.0, self.1, self.2)
    }
}
fn check_color(c: Color) {
    let Color(x, _, _) = c;
    println!("x: {}", x);
    println!("c: {}", c);
}



// 单元结构体 Unit-Like Structs Without Any Fields
fn unit_struct() {
    struct Unit;
    let unit = Unit;
    impl Unit {
        fn new() -> Unit {
            Unit
        }
    }
    let unit2 = Unit::new();
}