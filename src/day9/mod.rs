use crate::file::read_file;

pub fn parse_file(text: &str) -> Vec<isize> {
    let mut disk: Vec<isize> = vec![];
    for (i, c) in text.chars().enumerate() {
        let count = c.to_digit(10).unwrap();
        let id = i / 2;
        match i % 2 {
            0 => {
                for _ in 0..count {
                    disk.push(id as isize);
                }
            }
            1 => {
                for _ in 0..count {
                    disk.push(-1);
                }
            }
            _ => panic!(),
        }
    }
    disk
}

pub fn compact(mut disk: Vec<isize>) -> Vec<isize> {
    let mut write = 0;
    let mut read = disk.len() - 1;

    while read > write {
        while disk[write] != -1 {
            write += 1;
        }

        disk[write] = disk[read];
        read -= 1;
        while disk[read] == -1 {
            read -= 1;
        }
    }

    disk.truncate(read + 2);

    disk
}

pub fn checksum(disk: Vec<isize>) -> isize {
    disk.iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + (i as isize) * x)
}

#[allow(dead_code)]
pub fn part1() {
    let input = read_file(module_path!());
    println!("{:?}", checksum(compact(parse_file(&input))));
}

#[allow(dead_code)]
pub fn part2() {
    // let input = read_file(module_path!());
    // let (rules, prints) = parse_file(input.as_str());
    // let requires = map_rules(&rules);

    // let invalid = prints.iter().filter(|p| !is_valid(p, &requires));
    // let sum: usize = invalid
    //     .map(|p| sort_print(&rules, p))
    //     .map(|p| get_middle(&p))
    //     .sum();

    // println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "2333133121414131402";

    #[test]
    fn test_parse_file() {
        let disk = parse_file(TEST_STR);
        assert_eq!(
            disk,
            "00...111...2...333.44.5555.6666.777.888899"
                .chars()
                .map(|c| match c {
                    '.' => -1,
                    _ => c.to_digit(10).unwrap() as isize,
                })
                .collect::<Vec<isize>>()
        );
    }

    #[test]
    fn test_compact() {
        let disk = parse_file(TEST_STR);

        assert_eq!(
            compact(disk),
            "0099811188827773336446555566"
                .chars()
                .map(|c| c.to_digit(10).unwrap() as isize)
                .collect::<Vec<isize>>()
        )
    }

    #[test]
    fn test_checksum() {
        let disk = parse_file(TEST_STR);
        assert_eq!(checksum(compact(disk)), 1928)
    }
}
