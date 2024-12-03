#[cfg(test)]
mod tests {
    use crate::day03;

    #[test]
    fn test_success_solve() {
        let res = day03::solve("inputs/day03.txt");
        assert!(res.is_ok());
    }
}
