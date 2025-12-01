use crate::file::read_file;

type Multiplicand = usize;
type Multiplicands = Vec<(Multiplicand, Multiplicand)>;

pub fn parse_file(text: &str) -> Multiplicands {
    let mut multiplicands = vec![];

    let mut buffer;
    let mut is_enabled = true;

    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        match c {
            'm' => {
                if is_enabled && text[i..i + 4] == *"mul(" {
                    i += 4;
                    buffer = 0;
                    let mut left: Multiplicand = 0;
                    while i < chars.len() {
                        let d = chars[i];
                        if d.is_ascii_digit() {
                            buffer = buffer * 10 + d.to_string().parse::<Multiplicand>().unwrap()
                        } else if d == ',' {
                            if left > 0 {
                                // Bad comma
                                break;
                            }
                            left = buffer;
                            buffer = 0;
                        } else if d == ')' {
                            multiplicands.push((left, buffer));
                            break;
                        } else {
                            // Invalid character inside mul()
                            break;
                        }
                        i += 1;
                    }
                } else {
                    i += 1
                }
            }
            'd' => {
                if text[i..i + 4] == *"do()" {
                    is_enabled = true;
                    i += 4;
                } else if text[i..i + 7] == *"don't()" {
                    is_enabled = false;
                    i += 7;
                } else {
                    i += 1;
                }
            }
            _ => i += 1,
        }
    }

    multiplicands
}

pub fn sum_and_multiply(values: Multiplicands) -> usize {
    values.iter().fold(0, |acc, v| acc + v.0 * v.1)
}

#[allow(dead_code)]
pub fn part1() {
    let input = read_file(module_path!());
    println!("{}", sum_and_multiply(parse_file(input.as_str())));
}

#[allow(dead_code)]
pub fn part2() {
    let input = read_file(module_path!());
    println!("{}", sum_and_multiply(parse_file(input.as_str())));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    static TEST_STR2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))\nmul(8,5)";

    #[test]
    fn test_parse_file() {
        assert_eq!(parse_file("mul(1,23)"), vec![(1, 23)]);
        assert_eq!(parse_file(TEST_STR), vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
        assert_eq!(parse_file(TEST_STR2), vec![(2, 4), (8, 5), (8, 5)]);
    }

    #[test]
    fn test_sum_and_multiply() {
        assert_eq!(sum_and_multiply(parse_file("mul(123,456)")), 123 * 456);
        assert_eq!(
            sum_and_multiply(parse_file("mul(123,456)mul(789,1)")),
            123 * 456 + 789
        );
        assert_eq!(
            sum_and_multiply(parse_file("mul(123,456don't()do()mul(789,1)")),
            789
        );
        assert_eq!(sum_and_multiply(parse_file(TEST_STR)), 161);
        assert_eq!(sum_and_multiply(parse_file(TEST_STR2)), 88);
    }
}
