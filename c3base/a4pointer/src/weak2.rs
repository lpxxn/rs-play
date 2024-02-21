use std::rc::Rc;

#[test]
fn fn_test_weak1() {
    let five = Rc::new(5);
    println!("five = {:?}", five);
    let weak_five = Rc::downgrade(&five);

    // 有个疑问，这里如果注释掉，下面 drop(five) 后， weak_five.upgrade() 还能获取到值，为什么？
    // 因为如果还有Rc的引用，那么weak_five.upgrade()就能获取到值
    // 如果还有Rc的引用， drop(five) 也不会释放内存

    let strong_five: Option<Rc<_>> = weak_five.upgrade();
    println!("strong_five = {:?}", strong_five);
    assert_eq!(5, *strong_five.unwrap());


    let strong_five: Option<Rc<_>> = weak_five.upgrade();
    println!("strong_five = {:?}", strong_five);
    assert_eq!(Some(&5), strong_five.as_deref());
    // println!("ref count = {}", Rc::strong_count(strong_five.as_ref().unwrap()));
    match strong_five.as_ref() {
        None => { println!("None can't be counted!") }
        Some(rc) => { println!("ref count = {}", Rc::strong_count(rc)); }
    }
    drop(five);


    let strong_five: Option<Rc<_>> = weak_five.upgrade();
    println!("strong_five = {:?}", strong_five);
    match strong_five.as_ref() {
        None => { println!("None can't be counted!") }
        Some(rc) => { println!("ref count = {}", Rc::strong_count(rc)); }
    }
    assert_eq!(5, *strong_five.unwrap());

    let strong_five: Option<Rc<_>> = weak_five.upgrade();
    println!("strong_five = {:?}", strong_five);
    assert_eq!(None, strong_five);
}