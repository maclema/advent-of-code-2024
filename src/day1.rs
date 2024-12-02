use std::fs;

pub fn solve() -> (i32, i32) {
    let list = fs::read_to_string("day1-list.txt").unwrap();
    let (mut l1, mut l2): (Vec<i32>, Vec<i32>) = list
        .lines()
        .map(|v: &str| v.split_whitespace())
        .map(|v| v.map(|a| a.parse::<i32>().unwrap()))
        .map(|mut v| (v.next().unwrap(), v.next().unwrap()))
        .unzip();

    l1.sort();
    l2.sort();

    let distance: i32 = l1.iter().zip(l2.iter()).map(|(a, b)| (a - b).abs()).sum();

    let similarity = l1
        .iter()
        .map(|a| a * l2.iter().filter(|b| b == &a).count() as i32)
        .sum::<i32>();

    (distance, similarity)
}
