use std::fs::read_to_string;
use std::cmp::{min, max};
use std::collections::HashMap;

fn calibration_value(line: &[u8]) -> u32 {
    let mut digits = line.into_iter().filter(|c| c.is_ascii_digit());
    let first = digits.next().unwrap();
    let last = digits.last().unwrap_or(first);

    (*first as u32 - 48) * 10 + (*last as u32 - 48)
}

fn opt_min<T: Ord + Copy>(v1: Option<T>, v2: Option<T>) -> Option<T> {
    match (v1, v2) {
        (Some(_), Some(_)) => min(v1, v2),
        _ => max(v1, v2)
    }
}

fn calibration_value2(s: &str) -> i32 {
    let first_digit_candidates = HashMap::from([
        (0, opt_min(s.find("zero"), s.find("0"))),
        (1, opt_min(s.find("one"), s.find("1"))),
        (2, opt_min(s.find("two"), s.find("2"))),
        (3, opt_min(s.find("three"), s.find("3"))),
        (4, opt_min(s.find("four"), s.find("4"))),
        (5, opt_min(s.find("five"), s.find("5"))),
        (6, opt_min(s.find("six"), s.find("6"))),
        (7, opt_min(s.find("seven"), s.find("7"))),
        (8, opt_min(s.find("eight"), s.find("8"))),
        (9, opt_min(s.find("nine"), s.find("9"))),
    ]);
    let first_digit = first_digit_candidates.iter()
        .filter(|x| x.1.is_some()).min_by_key(|x| x.1).unwrap().0;
    let second_digit_candidates = HashMap::from([
        (0, max(s.rfind("zero"), s.rfind("0"))),
        (1, max(s.rfind("one"), s.rfind("1"))),
        (2, max(s.rfind("two"), s.rfind("2"))),
        (3, max(s.rfind("three"), s.rfind("3"))),
        (4, max(s.rfind("four"), s.rfind("4"))),
        (5, max(s.rfind("five"), s.rfind("5"))),
        (6, max(s.rfind("six"), s.rfind("6"))),
        (7, max(s.rfind("seven"), s.rfind("7"))),
        (8, max(s.rfind("eight"), s.rfind("8"))),
        (9, max(s.rfind("nine"), s.rfind("9"))),
    ]);
    let second_digit = second_digit_candidates.iter()
        .max_by_key(|x| x.1).unwrap().0;

    10 * first_digit + second_digit
}

fn main() {
    println!("Part 1: {}", read_to_string("day1input.txt").unwrap().lines().map(
        |l| calibration_value(l.as_bytes())).sum::<u32>());
    println!("Part 2: {}", read_to_string("day1input.txt").unwrap().lines().map(
        |l| calibration_value2(l)).sum::<i32>());
}
