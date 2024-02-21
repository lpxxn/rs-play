trait A {
    fn a(&self) {
        println!("trait default method");
    }
}

struct B;

impl B {
    fn a(&self) {
        println!("overridden method");
        // call default method here
        A::a(self);
    }
}

impl A for B {}
// https://stackoverflow.com/questions/31461902/is-it-possible-to-extend-a-default-method-implementation-of-a-trait-in-a-struct/56311216#56311216

#[test]
fn test_default_demo() {
    let a = B;
    a.a();
}

fn the_default() {
    println!("default implementation");
}

trait Foo {
    fn method(&self) {
        the_default()
    }
}

struct Bar;

impl Foo for Bar {
    fn method(&self) {
        the_default();
        println!("Hey, I'm doing something entirely different!");
    }
}

#[test]
fn test_default_demo2() {
    let b = Bar;
    b.method();
}