use std::mem;

use crate::file::read_file;

type Line = (usize, Vec<usize>);

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Operator {
    Add,
    Multiply,
    Concat,
}
const OPERATOR_COUNT: usize = mem::variant_count::<Operator>();

pub fn parse_file(text: &str) -> Vec<Line> {
    text.split('\n').map(parse_line).collect()
}

pub fn parse_line(text: &str) -> Line {
    let parts: Vec<&str> = text.split(":").collect();
    (
        parts[0].parse().unwrap(),
        parts[1]
            .split_whitespace()
            .map(|c| c.parse().unwrap())
            .collect(),
    )
}

pub fn find_operators(line: &Line) -> Option<Vec<Operator>> {
    let operators_len = line.1.len() - 1;
    let combinations: usize = OPERATOR_COUNT.pow(operators_len as u32);
    for combination in 0..combinations {
        let mut operators = vec![Operator::Add; operators_len];
        for (i, op) in operators.iter_mut().enumerate() {
            let value = (combination / (OPERATOR_COUNT.pow(i as u32))) % OPERATOR_COUNT;
            *op = match value {
                0 => Operator::Add,
                1 => Operator::Multiply,
                2 => Operator::Concat,
                _ => panic!("got value out of range"),
            };
        }
        if check_operators(line, &operators) {
            return Some(operators);
        }
    }
    None
}

pub fn check_operators(line: &Line, operators: &[Operator]) -> bool {
    let mut result = line.1[0];
    for (i, op) in operators.iter().enumerate() {
        result = match op {
            Operator::Add => result + line.1[i + 1],
            Operator::Multiply => result * line.1[i + 1],
            Operator::Concat => (result.to_string() + line.1[i + 1].to_string().as_str())
                .parse()
                .unwrap(),
        }
    }
    result == line.0
}

pub fn sum_values(lines: Vec<Line>) -> usize {
    lines.iter().fold(0, |acc, l| match find_operators(l) {
        Some(_) => acc + l.0,
        _ => acc,
    })
}

#[allow(dead_code)]
pub fn part1() {
    let input = read_file(module_path!());
    let lines = parse_file(&input);
    println!("{}", sum_values(lines));
}

#[allow(dead_code)]
pub fn part2() {
    let input = read_file(module_path!());
    let lines = parse_file(&input);
    println!("{}", sum_values(lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_parse_file() {
        let lines = parse_file(TEST_STR);
        assert_eq!(lines[1].0, 3267);
        assert_eq!(lines[1].1[1], 40);
    }

    #[test]
    fn test_check_operators() {
        assert!(check_operators(&(190, vec![10, 19]), &[Operator::Multiply]));
        assert!(!check_operators(&(190, vec![10, 19]), &[Operator::Add]));
        assert!(check_operators(
            &(202, vec![10, 19, 12]),
            &[Operator::Multiply, Operator::Add]
        ));
        assert!(!check_operators(
            &(202, vec![10, 19, 12]),
            &[Operator::Multiply, Operator::Multiply]
        ));
    }

    #[test]
    fn test_find_operators() {
        assert_eq!(
            find_operators(&(190, vec![10, 19])),
            Some(vec![Operator::Multiply])
        );
    }

    #[test]
    fn test_sum_values() {
        let lines = parse_file(TEST_STR);
        // assert_eq!(sum_values(lines), 3749);
        assert_eq!(sum_values(lines), 11387);
    }
}
