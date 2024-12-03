use regex::Regex;
use std::fs;

use crate::error::Error;

pub struct SolverResult {
    pub part1: i64,
    pub part2: i64,
}

pub fn solve(path: &str) -> Result<SolverResult, Error> {
    // Load the input list file
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return Err(Error::from(e)),
    };

    let mut mul_enabled = true;
    let mut part1: i64 = 0;
    let mut part2: i64 = 0;

    let exp = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),\s*(\d{1,3})\)")?;
    for m in exp.captures_iter(&content) {
        if m[0].to_string() == "do()" {
            mul_enabled = true;
        } else if m[0].to_string() == "don't()" {
            mul_enabled = false;
        } else {
            let a = m[1].parse::<i64>().unwrap_or_default();
            let b = m[2].parse::<i64>().unwrap_or_default();
            let c = a * b;
            part1 += c;
            if mul_enabled {
                part2 += c;
            }
        }
    }

    Ok(SolverResult { part1, part2 })
}
