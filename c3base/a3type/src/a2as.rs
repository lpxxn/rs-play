fn add() {
    let arr: [u64; 13] = [0; 13];
    assert_eq!(std::mem::size_of_val(&arr), 13 * 8);

    let a: *const [u64] = &arr;
    let b = a as *const [u8];
    unsafe {
        assert_eq!(std::mem::size_of_val(&*b), 13);
    }
}
#[allow(overflowing_literals)]
fn as1() {
    assert_eq!(u8::MAX, 255);
    let v = 10000 as u8;
    assert_eq!(v, 16);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as() {
        as1();
    }

    #[test]
    fn it_works() {
        add();
    }
}