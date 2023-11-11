fn main() {
    basic_tup();
    tup_too_long();

    let tup = ("hello".to_string(), 1);
    // 下面的就是这样 （a: &String, b: &i32）= &tup
    let (a, b) = &tup;
    println!("a: {}, b: {}", a, b);
    println!("tup: {:?}", tup);
    // 下面的是这样 &(a: String, b: i32) = &tup
    // let &(a, b) = &tup;
    // println!("a: {}, b: {}", a, b);

    // let c = &tup;
    // let d = &tup.0;
    // let e = &tup.0;
    // println!("address p.0 {:p} e: {:p}", &tup.0, &e);
    // println!("c: {:?}", c);
    //println!("tup: {:?}", tup);
}


fn basic_tup() {
    let tup: (i32, f64, u8) = (1, 3.14, 1);

    println!("tup: {:?}, index: {}, idx2: {}", tup, tup.0, tup.2);
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);

    let (a, b, c);
    // fill the blank
    (a, b, c) = tup;
    println!("a: {}, b: {}, c: {}", a, b, c);
}

// length greater than 12, error
fn tup_too_long() {
    let too_long_tup = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12 /*, 13*/);
    println!("too_long_tup: {:?}", too_long_tup);
//     // println!("too_long_tup: {:#?}", too_long_tup);
}