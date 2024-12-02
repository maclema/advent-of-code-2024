#[cfg(test)]
mod tests {
    use crate::day02;

    #[test]
    fn test_success_solve() {
        let res = day02::solve("inputs/day02-list.txt");
        assert!(res.is_ok());
    }

    #[test]
    fn test_file_load_error() {
        if let Err(e) = day02::solve("inputs/does-not-exist") {
            assert_eq!(e.to_string(), "No such file or directory (os error 2)");
        } else {
            assert_eq!(false, true, "did not fail");
        }
    }

    #[test]
    fn test_invalid_pairs_a() {
        if let Err(e) = day02::solve("inputs/day1-list-invalid-pairs.txt") {
            assert_eq!(e.to_string(), "invalid digit found in string");
        } else {
            assert_eq!(false, true, "did not fail");
        }
    }
}
