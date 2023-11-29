#[cfg(test)]
mod tests {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_rc1() {
        let a = Rc::new(String::from("hello"));
        let b = a.clone();
        let c = Rc::clone(&a);

        assert_eq!(Rc::strong_count(&a), 3);
        assert_eq!(Rc::strong_count(&b), 3);
        assert_eq!(Rc::strong_count(&c), 3);
    }

    #[test]
    fn test_rc2() {
        // Arc 是 Atomic Rc 的缩写，顾名思义：原子化的 Rc<T> 智能指针
        let s = Arc::new(String::from("多线程"));
        for _ in 0..10 {
            let s = Arc::clone(&s);
            let handle = thread::spawn(move || {
                println!("s: {:?}", s)
            });
        }
    }

    #[test]
    fn test_cell() {
        // use std::cell::Cell;
        // let x = Cell::new(1);
        // let y = &x;
        // let z = &x;
        // x.set(2);
        // y.set(3);
        // z.set(4);
        // assert_eq!(4, x.get());

        // Cell 和 RefCell 在功能上没有区别，区别在于 Cell<T> 适用于 T 实现 Copy 的情况
        let x = Cell::new(1);
        let one = x.get();
        assert_eq!(1, one);
        let y = &x;
        x.set(2);
        assert_eq!(2, y.get());
        let two = y.get();
        y.set(3);
        assert_eq!(3, x.get());
    }

    #[test]
    fn test_refcell() {
        let x = RefCell::new(1);
        println!("x: {:?}", x);
        // let y = x.borrow_mut();
        {
            let mut z = x.borrow_mut();
            *z = 3;
            //println!("x: {:?}", x.borrow());
            println!("x: {:?}", x); // x: RefCell { value: <borrowed> }
            assert_eq!(3, *z);
        }
        println!("x: {:?}", x);// x: RefCell { value: 3 }
        println!("x: {:?}", x.borrow()); // x: 3
        println!("x: {:?}", *x.borrow()); // x: 3
        println!("x: {:?}", x);
        // assert_eq!(3, x);
    }
}