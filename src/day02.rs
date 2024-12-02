use std::{fs, ops::Deref};

use crate::error::Error;

pub struct SolverResult {
    pub num_safe_reports: usize,
    pub num_safe_reports_p2: usize,
}

#[derive(Debug, Clone)]
struct Levels(Vec<i64>);

impl Deref for Levels {
    type Target = Vec<i64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for Levels {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        let list: Vec<&str> = val.split_ascii_whitespace().collect();
        let mut res = Vec::with_capacity(list.len());
        for v in list {
            match v.parse::<i64>() {
                Ok(r) => res.push(r),
                Err(e) => return Err(Error::from(e)),
            }
        }
        Ok(Self(res))
    }
}

impl FromIterator<i64> for Levels {
    fn from_iter<I: IntoIterator<Item = i64>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

const fn is_safe_decrease(v: i64) -> bool {
    v < 0 && v >= -3
}

const fn is_safe_increase(v: i64) -> bool {
    v > 0 && v <= 3
}

fn get_diffs(levels: &Levels) -> Levels {
    levels
        .clone()
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect::<Levels>()
}

fn is_safe(l: &Levels) -> bool {
    let safe_increasing = l.iter().all(|v| is_safe_increase(*v));
    let safe_decreasing = l.iter().all(|v| is_safe_decrease(*v));
    safe_increasing || safe_decreasing
}

fn is_safe_p1(l: &Levels) -> bool {
    let diffs: Levels = get_diffs(l);
    is_safe(&diffs)
}

fn combinations(l: &Levels) -> Vec<Levels> {
    let mut result: Vec<Levels> = Vec::new();
    for i in 0..l.len() {
        let mut res = Vec::with_capacity(l.len() - 1);
        for j in 0..l.len() {
            if i != j {
                res.push(l[j]);
            }
        }
        result.push(Levels(res));
    }
    result
}

fn is_safe_p2(l: &Levels) -> bool {
    let r: Vec<Levels> = combinations(l);
    let diffs: Vec<Levels> = r.iter().map(get_diffs).collect();
    let increasing = diffs.iter().any(is_safe);
    let decreasing = diffs.iter().any(is_safe);
    increasing || decreasing
}

pub fn solve(path: &str) -> Result<SolverResult, Error> {
    // Load the input list file
    let list = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return Err(Error::from(e)),
    };

    // Each report is a list of numbers called levels that are separated by spaces
    let mut levels: Vec<Levels> = Vec::new();
    for line in list.lines() {
        levels.push(Levels::try_from(line)?);
    }

    let num_safe_reports = levels.iter().filter(|l| is_safe_p1(l)).count();
    let num_safe_reports_p2 = levels.iter().filter(|l| is_safe_p2(l)).count();

    Ok(SolverResult {
        num_safe_reports,
        num_safe_reports_p2,
    })
}
