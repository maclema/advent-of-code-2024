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
mod day02;
mod day02_test;
mod day03;
mod day03_test;
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
        Err(e) => println!("Day 1: Error: {e}"),
    }

    // Day 2
    match day02::solve("inputs/day02-list.txt") {
        Ok(result) => {
            println!("Day 2: Num Safe Reports: {}", result.num_safe_reports);
            println!(
                "Day 2: Num Safe Reports (p2): {}",
                result.num_safe_reports_p2
            );
            assert_eq!(result.num_safe_reports, 502);
            assert_eq!(result.num_safe_reports_p2, 544);
        }
        Err(e) => println!("Day 1: Error: {e}"),
    }

    // Day 3
    match day03::solve("inputs/day03.txt") {
        Ok(result) => {
            println!("Day 3: Part1: {}", result.part1);
            println!("Day 3: Part2: {}", result.part2);
            assert_eq!(result.part1, 156_388_521);
            assert_eq!(result.part2, 75_920_122);
        }
        Err(e) => println!("Day 1: Error: {e}"),
    }
}
