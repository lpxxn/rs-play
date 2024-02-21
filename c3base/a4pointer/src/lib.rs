mod a2deref;
mod a1_box;
mod a3deref;
mod a4rc;
mod pin_t;
mod weak;
mod weak2;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
