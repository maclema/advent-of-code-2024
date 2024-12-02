#[cfg(test)]
mod tests {
    use crate::day01;

    #[test]
    fn test_success_solve() {
        let res = day01::solve("inputs/day1-list.txt");
        assert!(res.is_ok());
    }

    #[test]
    fn test_file_load_error() {
        if let Err(e) = day01::solve("inputs/does-not-exist") {
            assert_eq!(e.to_string(), "No such file or directory (os error 2)");
        } else {
            assert_eq!(false, true, "did not fail");
        }
    }

    #[test]
    fn test_invalid_pairs_a() {
        if let Err(e) = day01::solve("inputs/day1-list-invalid-pairs.txt") {
            assert_eq!(e.to_string(), "invalid digit found in string");
        } else {
            assert_eq!(false, true, "did not fail");
        }
    }

    #[test]
    fn test_invalid_pairs_b() {
        if let Err(e) = day01::solve("inputs/day1-list-invalid-pairs-b.txt") {
            assert_eq!(e.to_string(), "invalid digit found in string");
        } else {
            assert_eq!(false, true, "did not fail");
        }
    }

    #[test]
    fn test_invalid_lines() {
        if let Err(e) = day01::solve("inputs/day1-list-invalid-lines.txt") {
            assert_eq!(
                e.to_string(),
                "Invalid input list, not all lines contain exactly two pairs."
            );
        } else {
            assert_eq!(false, true, "did not fail");
        }
    }
}
