#![forbid(clippy::all)]
#![forbid(clippy::pedantic)]
#![forbid(clippy::nursery)]
#![forbid(clippy::cargo)]
#![forbid(clippy::style)]
#![forbid(clippy::suspicious)]
#![forbid(clippy::correctness)]
#![forbid(clippy::allow_attributes)]
#![forbid(clippy::as_conversions)]
#![forbid(clippy::exit)]
#![forbid(clippy::expect_used)]
#![forbid(clippy::float_cmp_const)]
#![forbid(clippy::large_include_file)]
#![forbid(clippy::suspicious_xor_used_as_pow)]
#![forbid(clippy::match_result_ok)]
#![forbid(clippy::unwrap_used)]
#![forbid(clippy::panic)]

mod day01;
mod day01_test;
mod error;

fn main() {
    // Day 1
    match day01::solve("inputs/day1-list.txt") {
        Ok(result) => {
            println!("Day 1: Distance: {}", result.distance);
            println!("Day 1: Similarity: {}", result.similarity);
            assert_eq!(result.distance, 2_057_374);
            assert_eq!(result.similarity, 23_177_084);
        }
        Err(e) => {
            println!("Day 1: Error: {e}");
        }
    }
}
