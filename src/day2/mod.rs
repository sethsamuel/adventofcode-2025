use crate::file::read_file;

type Level = usize;
type Report = Vec<Level>;
type Reports = Vec<Report>;

pub fn parse_file(text: &str) -> Reports {
    text.split('\n').map(parse_line).collect()
}

pub fn parse_line(line: &str) -> Report {
    line.split_whitespace()
        .map(|d| d.parse::<usize>().unwrap())
        .collect()
}

/// Returns the index of the unsafe level, or None if the report is safe
pub fn find_unsafe_level(report: &Report) -> Option<usize> {
    if report.len() < 2 {
        return None;
    }
    let mut last = &report[0];
    let is_increasing = report[1].ge(last);
    for (i, current) in report.iter().enumerate().skip(1) {
        if !(1..=3).contains(&current.abs_diff(*last)) {
            return Some(i);
        }
        if is_increasing && current.lt(last) {
            return Some(i);
        }
        if !is_increasing && current.gt(last) {
            return Some(i);
        }

        last = current
    }
    None
}

pub fn is_safe(report: &Report, can_skip: bool) -> bool {
    let unsafe_level = find_unsafe_level(report);
    if can_skip && unsafe_level.is_some() {
        if find_unsafe_level(&report.iter().skip(1).copied().collect()).is_none() {
            return true;
        }
        for skip in unsafe_level.unwrap() - 1..=unsafe_level.unwrap() {
            if find_unsafe_level(
                &report
                    .iter()
                    .enumerate()
                    .filter_map(|(i, v)| (i != skip).then_some(*v))
                    .collect(),
            )
            .is_none()
            {
                return true;
            }
        }
    }
    unsafe_level.is_none()
}

pub fn find_safe_count(reports: Reports, can_skip: bool) -> usize {
    reports.iter().filter(|r| is_safe(r, can_skip)).count()
}

#[allow(dead_code)]
pub fn part1() {
    let input = read_file(module_path!());
    println!("{}", find_safe_count(parse_file(input.as_str()), false));
}

#[allow(dead_code)]
pub fn part2() {
    let input = read_file(module_path!());
    let reports = parse_file(input.as_str());
    println!("{}", find_safe_count(reports, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("1  2"), vec![1, 2]);
        assert_eq!(parse_line(" 12  2"), vec![12, 2]);
        assert_eq!(parse_line(" 1   202 3"), vec![1, 202, 3]);
    }

    #[test]
    fn test_parse_file() {
        assert_eq!(
            parse_file("3 3 4\n1  2\n21 1 1  12"),
            vec![vec![3, 3, 4], vec![1, 2], vec![21, 1, 1, 12]]
        )
    }

    #[test]
    fn test_find_unsafe_level() {
        assert_eq!(find_unsafe_level(&vec![]), None);
        assert_eq!(find_unsafe_level(&vec![1]), None);
        assert_eq!(find_unsafe_level(&vec![1, 2, 4]), None);
        assert_eq!(find_unsafe_level(&vec![1, 0, 1]), Some(2));
        assert_eq!(find_unsafe_level(&vec![1, 101, 1]), Some(1));
        assert_eq!(find_unsafe_level(&vec![1, 3, 2, 4, 5]), Some(2));
        assert_eq!(find_unsafe_level(&vec![1, 3, 2, 4, 5]), Some(2));
        assert_eq!(find_unsafe_level(&vec![2, 5, 4, 3, 2]), Some(2));
        assert_eq!(find_unsafe_level(&vec![2, 1, 3, 4, 5]), Some(2));
    }

    #[test]
    fn test_is_safe() {
        // assert!(is_safe(&vec![], false));
        // assert!(is_safe(&vec![1], false));
        // assert!(is_safe(&vec![1, 2, 4], false));
        // assert!(!is_safe(&vec![1, 0, 1], false));
        // assert!(!is_safe(&vec![1, 101, 1], false));
        // assert!(!is_safe(&vec![1, 3, 2, 4, 5], false));
        // assert!(!is_safe(&vec![2, 1, 1, 4, 5], true));
        // assert!(is_safe(&vec![1, 3, 2, 4, 5], true));
        // assert!(is_safe(&vec![2, 1, 3, 4, 5], true));
        assert!(is_safe(&vec![2, 5, 4, 3, 2], true));
    }

    #[test]
    fn test_find_safe_count() {
        assert_eq!(find_safe_count(parse_file(TEST_STR), false), 2);
        assert_eq!(find_safe_count(parse_file(TEST_STR), true), 4);
    }
}
