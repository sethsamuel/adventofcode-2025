use std::collections::HashMap;

use crate::file::read_file;

type Id = usize;
type Ids = Vec<Id>;
type IdPair = (Id, Id);
type IdPairs = (Ids, Ids);

pub fn parse_file(text: &str) -> IdPairs {
    let mut left = Ids::new();
    let mut right = Ids::new();
    text.split('\n').map(parse_line).for_each(|p| {
        left.push(p.0);
        right.push(p.1);
    });
    left.sort();
    right.sort();

    (left, right)
}

pub fn parse_line(line: &str) -> IdPair {
    let ids: Vec<usize> = line
        .split_whitespace()
        .map(|d| d.parse::<usize>().unwrap())
        .collect();

    (*ids.first().unwrap(), *ids.last().unwrap())
}

pub fn find_id_distance(pairs: IdPairs) -> usize {
    let mut distance = 0;
    let left = pairs.0;
    let right = pairs.1;
    for i in 0..left.len() {
        match left[i] > right[i] {
            true => distance += left[i] - right[i],
            _ => distance += right[i] - left[i],
        }
    }
    distance
}

pub fn find_id_similarity(pairs: IdPairs) -> usize {
    let mut counts: HashMap<Id, usize> = HashMap::new();
    let mut score = 0;
    pairs.1.iter().for_each(|id| {
        counts.insert(*id, counts.get(id).unwrap_or(&0) + 1);
    });

    pairs
        .0
        .iter()
        .for_each(|id| score += *id * counts.get(id).unwrap_or(&0));

    score
}

#[allow(dead_code)]
pub fn part1() {
    let input = read_file(module_path!());
    println!("{}", find_id_distance(parse_file(input.as_str())));
}

#[allow(dead_code)]
pub fn part2() {
    let input = read_file(module_path!());
    println!("{}", find_id_similarity(parse_file(input.as_str())));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("1  2"), (1, 2));
        assert_eq!(parse_line(" 12  2"), (12, 2));
        assert_eq!(parse_line(" 1   202 "), (1, 202));
    }

    #[test]
    fn test_parse_file() {
        assert_eq!(parse_file("3 4\n1  2\n2 2"), (vec![1, 2, 3], vec![2, 2, 4]))
    }

    #[test]
    fn test_find_id_distance() {
        assert_eq!(find_id_distance(parse_file(TEST_STR)), 11)
    }

    #[test]
    fn test_find_id_similarity() {
        assert_eq!(find_id_similarity(parse_file(TEST_STR)), 31)
    }
}
