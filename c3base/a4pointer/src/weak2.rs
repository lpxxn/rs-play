use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[test]
fn fn_test_weak1() {
    let five: Rc<i32> = Rc::new(5);
    println!("five = {:?}", five);
    let weak_five = Rc::downgrade(&five);

    // 有个疑问，这里如果注释掉，下面 drop(five) 后， weak_five.upgrade() 还能获取到值，为什么？
    // 因为如果还有Rc的引用，那么weak_five.upgrade()就能获取到值
    // 如果还有Rc的引用， drop(five) 也不会释放内存
    let strong_five: Option<Rc<_>> = weak_five.upgrade();
    println!("strong_five = {:?}", strong_five);
    // assert_eq!(Rc::new(5), strong_five.unwrap());
    // assert_eq!(5, *strong_five.unwrap());
    let _ = strong_five.unwrap();

    let strong_five: Option<Rc<_>> = weak_five.upgrade();
    println!("strong_five = {:?}", strong_five);
    assert_eq!(Some(&5), strong_five.as_deref());
    // println!("ref count = {}", Rc::strong_count(strong_five.as_ref().unwrap()));
    match strong_five.as_ref() {
        None => { println!("None can't be counted!") }
        Some(rc) => { println!("2 ref count = {}", Rc::strong_count(rc)); }
    }
    let _ = strong_five.unwrap();
    drop(five);


    let strong_five: Option<Rc<_>> = weak_five.upgrade();
    println!("strong_five = {:?}", strong_five);
    match strong_five.as_ref() {
        None => { println!("None can't be counted!") }
        Some(rc) => {
            println!("3 ref count = {}", Rc::strong_count(rc));
            assert_eq!(5, *strong_five.unwrap());
        }
    }

    let strong_five: Option<Rc<_>> = weak_five.upgrade();
    println!("strong_five = {:?}", strong_five);
    assert_eq!(None, strong_five);
}


#[derive(Debug)]
enum List {
    Cons(u32, RefCell<Weak<List>>),
    // Nil: 链表中的最后一个节点，用于说明链表的结束
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Weak<List>>> {
        match self {
            List::Cons(_, item) => Some(item),
            List::Nil => None,
        }
    }
}

#[test]
fn test_list() {
    let a = Rc::new(List::Cons(1, RefCell::new(Weak::new())));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(List::Cons(2, RefCell::new(Rc::downgrade(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        println!("a next item link = {:?}", link);
        *link.borrow_mut() = Rc::downgrade(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    println!("a next item after changing a = {:?}", a.tail());
}