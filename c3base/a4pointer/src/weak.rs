use std::cell::RefCell;
use std::rc::Rc;
use crate::weak::List::Cons;

#[derive(Debug)]
enum List {
    Cons(u32, RefCell<Rc<List>>),
    // Nil: 链表中的最后一个节点，用于说明链表的结束
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            List::Cons(_, item) => Some(item),
            List::Nil => None,
        }
    }
}

#[test]
fn test_list() {
    let a = Rc::new(Cons(1, RefCell::new(Rc::new(List::Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    // create b from a
    let b = Rc::new(Cons(2, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        println!("a next item link = {:?}", link);
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    // 下面将发生错误，因为会形成循环引用
    // 我们可怜的8字节内存将会被无限的引用，直到内存耗尽，造成栈溢出
    // println!("a next item after changing a = {:?}", a.tail());
}