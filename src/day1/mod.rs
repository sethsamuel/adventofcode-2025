use crate::file::read_file;

type Rotation = isize;
type Rotations = Vec<Rotation>;

fn parse_file(text: &str) -> Rotations {
    text.split('\n').map(parse_line).collect()
}

fn parse_line(line: &str) -> Rotation {
    let mut chars = line.chars();
    let direction = match chars.next().unwrap() {
        'L' => -1,
        'R' => 1,
        u => panic!("Unknown direction{u}"),
    };

    let distance = chars.collect::<String>().parse::<isize>().unwrap();

    distance * direction
}

fn count_zeros(rotations: Rotations) -> usize {
    let mut current = 50;
    let mut zero_count = 0;
    for r in rotations {
        current = next_value(current as isize, r);
        if current == 0 {
            zero_count += 1;
        }
    }
    zero_count
}

fn count_zeros_passed(rotations: Rotations) -> usize {
    let mut current = 50;
    let mut zero_count = 0;
    for r in rotations {
        let (new, passed) = next_value_with_passed(current as isize, r);
        // if new == 0 || current == 0 {
        //     println!("start {current} rotation {r} new {new} passed {passed}");
        // }
        zero_count += passed;
        current = new;
    }
    zero_count
}

fn next_value(start: isize, rotation: Rotation) -> usize {
    let mut new = start + rotation;
    if new < 0 {
        new %= -100;
        new = 100_usize.overflowing_add_signed(new).0 as isize;
    }
    if new >= 100 {
        new %= 100;
    }

    new as usize
}

fn next_value_with_passed(start: isize, rotation: Rotation) -> (usize, usize) {
    let mut new = start + rotation;
    let mut passed = 0;
    if new < 0 {
        passed += new / -100;
        if start != 0 {
            passed += 1;
        }
        new %= -100;
        new = 100_usize.overflowing_add_signed(new).0 as isize;
    }
    if new >= 100 {
        if new > 100 {
            passed += new / 100;
        }
        new %= 100;
    }
    if new == 0 && rotation.abs() < 100 {
        passed += 1;
    }

    (new as usize, passed as usize)
}

#[allow(dead_code)]
pub fn part1() {
    let input = read_file(module_path!());
    let rotations = parse_file(&input);
    let zeros = count_zeros(rotations);
    println!("found {zeros} zeros")
}

#[allow(dead_code)]
pub fn part2() {
    let input = read_file(module_path!());
    let rotations = parse_file(&input);
    let zeros = count_zeros_passed(rotations);
    println!("found {zeros} zeros")
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
        assert_eq!(parse_line("L230"), -230);
        assert_eq!(parse_line("R4"), 4);
    }
    #[test]
    fn test_next_value() {
        assert_eq!(next_value(50, -10), 40);
        assert_eq!(next_value(1, -5), 96);
        assert_eq!(next_value(98, 4), 2);
    }

    #[test]
    fn test_next_value_with_passed() {
        assert_eq!(next_value_with_passed(50, 1000), (50, 10));
        assert_eq!(next_value_with_passed(50, -100), (50, 1));
        assert_eq!(next_value_with_passed(50, -1000), (50, 10));
        assert_eq!(next_value_with_passed(1, 99), (0, 1));
        assert_eq!(next_value_with_passed(19, -319), (0, 4));
    }

    #[test]
    fn test_count_zeros() {
        assert_eq!(count_zeros(parse_file(TEST_STR)), 3);
    }

    #[test]
    fn test_count_zero_with_passed() {
        assert_eq!(count_zeros_passed(parse_file(TEST_STR)), 6)
    }
}
