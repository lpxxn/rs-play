#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s1 = Some("some1");
        let s2 = Some("some2");

        let fn_some = || {
            println!("some2");
            Some("some2")
        };

        let n: Option<&str> = None;
        let fn_none = || None;
        assert_eq!(s1.or_else(fn_some), s1);

        assert_eq!(n.or_else(fn_none), None);

        #[test]
        fn r2() {
            const ERR_DEFAULT: &str = "default error";

            let s = Some("some");
            let n: Option<&str> = None;

            let o: Result<&str, &str> = Ok("ok");
            let e: Result<&str, &str> = Err(ERR_DEFAULT);

        }
    }
}