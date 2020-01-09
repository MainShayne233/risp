extern crate risp_code;
use risp_code::add_one;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro() {
        let actual = add_one!(2);
        let expected = 3;
        assert_eq!(actual, expected);
    }
}


