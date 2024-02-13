mod a {
    pub trait Draw {
        fn draw(&self);
    }
}

pub mod b {
    use crate::screen::a::Draw;

    // pub struct Screen<T: Draw> {
    pub struct Screen {
        // 这样就会是静态分发，所以会有问题
        // pub components: Vec<T>,
        pub components: Vec<Box::<dyn Draw>>,
    }

    // impl<T: Draw> Screen<T> {
    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    #[derive(Debug)]
    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            // 绘制按钮的代码
            println!("width: {}", self.width)
        }
    }

    pub struct SelectBox {
        pub width: u32,
        pub height: u32,
        pub options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            // 绘制SelectBox的代码
        }
    }
}

#[cfg(test)]
mod t {
    use crate::screen::b::*;
    use crate::screen::a::*;

    #[test]
    fn t_1() {
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No"),
                    ],
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("OK"),
                }),
            ],
        };

        screen.run();
    }
}
