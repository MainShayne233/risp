use risp::risp;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_level_expr() {
        let actual = risp! {
            // 2 + 3
            (+ 2 3)
        };

        let expected = 5;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multi_level_expr() {
        let actual = risp! {
            // (4 + 5) + (6 + 6)
            (+
              (+ 4 5)
              (+ 6 7)
            )
        };

        let expected = 22;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_other_operators() {
        let actual = risp! {
            // (7 - 3) * ((4 + 6) / (1 * 2))
            (*
             (- 7 3)
             (/ (+ 4 6) (* 1 2))
            )
        };

        let expected = 20;

        assert_eq!(actual, expected);
    }
}
