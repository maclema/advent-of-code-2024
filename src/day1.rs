use std::{collections::HashMap, fs};

use crate::error::Error;

pub struct SolverResult {
    pub distance: i64,
    pub similarity: i64,
}

/// Converts a Vec<&str> to a sorted Vec<i32>
fn to_sorted_int_vec(list: Vec<&str>) -> Result<Vec<i64>, Error> {
    let mut res = Vec::with_capacity(list.len());
    for v in list {
        match v.parse::<i64>() {
            Ok(r) => res.push(r),
            Err(e) => return Err(Error::from(e)),
        }
    }
    res.sort_unstable();
    Ok(res)
}

/// Counts the frequency of values in the given list
fn count_frequency(list: &Vec<i64>) -> HashMap<i64, i64> {
    let mut res: HashMap<i64, i64> = HashMap::new();
    for k in list {
        match res.get(k) {
            Some(v) => res.insert(*k, v + 1),
            None => res.insert(*k, 1),
        };
    }
    res
}

pub fn solve() -> Result<SolverResult, Error> {
    // Load the input list file
    let list = match fs::read_to_string("day1-list.txt") {
        Ok(c) => c,
        Err(e) => return Err(Error::from(e)),
    };

    // Split by lines, and then each line by whitespace
    let pairs: Vec<Vec<&str>> = list
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect();

    // Every pair is expected to contain 2 values
    if 0 != pairs.iter().filter(|p| p.len() != 2).count() {
        return Err(Error::new(
            "Invalid input list, not all lines contain exactly two pairs.",
        ));
    }

    // Unzip the pairs into their own lists
    let (l1s, l2s): (Vec<&str>, Vec<&str>) = pairs.iter().map(|v| (v[0], v[1])).unzip();

    // Parse the strings to i32 and sort the lists
    let l1 = to_sorted_int_vec(l1s)?;
    let l2 = to_sorted_int_vec(l2s)?;

    // Calculate the distance
    let distance: i64 = l1.iter().zip(l2.iter()).map(|(a, b)| (a - b).abs()).sum();

    // Calculate the similarity
    let counts = count_frequency(&l2);
    let similarity = l1.iter().map(|a| a * counts.get(a).unwrap_or(&0)).sum();

    Ok(SolverResult {
        distance,
        similarity,
    })
}
