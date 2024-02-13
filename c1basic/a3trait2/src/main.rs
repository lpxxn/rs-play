mod bird;
mod safe_trait;
mod advance1_same_name;
mod advance2_sup_trait;
mod advance3;
mod advance4;
mod screen;

fn main() {
    let x = 1u8;
    let y = 1.0f64;
    let z = "hello".to_string();

    draw1(Box::new(x));
    draw1(Box::new(y));
    draw2(&x);
    draw2(&y);
    println!("z: {}", z);

    let screen = Screen {
        components: vec![
            Box::new(x),
            Box::new(y),
            Box::new(z),
        ],
    };
    screen.run();
}


trait Draw {
    fn draw(&self) -> String;
}


impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

impl Draw for String {
    fn draw(&self) -> String {
        format!("String: {}", *self)
    }
}

// 若 T 实现了 Draw 特征，则调用该函数传入的Box<dyn Draw>类型的参数x，可以调用x.draw()方法
// dyn Draw 表示这是一个特征对象，它是一个动态类型，它的大小在编译期是不确定的，只有在运行时才能确定。
// dyn 的意思是“动态的”（dynamic），它是一个特殊的类型，它的大小在编译期是不确定的，只有在运行时才能确定。
// dyn 只能用于特征
fn draw1(x: Box<dyn Draw>) {
    // 由
    println!("{}", x.draw());
}

// &dyn 的意思是“动态引用”（dynamic reference），它是一个特殊的引用类型，它的大小在编译期是不确定的，只有在运行时才能确定。
// &dyn Draw 形式的特征对象，该特征对象是通过 &x 的方式创建的
fn draw2(x: &dyn Draw) {
    println!("{}", x.draw());
}

/*
注意 dyn 不能单独作为特征对象的定义，例如下面的代码编译器会报错，原因是特征对象可以是任意实现了某个特征的类型，编译器在编译期不知道该类型的大小，不同的类型大小是不同的。
而 &dyn 和 Box<dyn> 在编译期都是已知大小，所以可以用作特征对象的定义。

fn draw2(x: dyn Draw) {
    x.draw();
}
 */

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            println!("{}", component.draw());
        }
    }
}

