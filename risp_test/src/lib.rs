extern crate risp_code;
use risp_code::make_answer;

make_answer!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_answer() {
        assert_eq!(answer(), 42);
    }
}
