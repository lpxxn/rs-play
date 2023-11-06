fn main() {
    basic_tup();
    tup_too_long();
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