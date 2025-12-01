use crate::file::read_file;

type Grid = Vec<Vec<char>>;

pub fn parse_file(text: &str) -> Grid {
    text.split('\n').map(|l| l.chars().collect()).collect()
}

pub enum Direction {
    N,
    S,
    E,
    W,
    Nw,
    Ne,
    Sw,
    Se,
}

impl Direction {
    fn get_dx_dy(&self) -> (isize, isize) {
        match self {
            Direction::N => (0, -1),
            Direction::S => (0, 1),
            Direction::E => (1, 0),
            Direction::W => (-1, 0),
            Direction::Nw => (-1, -1),
            Direction::Ne => (1, -1),
            Direction::Sw => (-1, 1),
            Direction::Se => (1, 1),
        }
    }

    fn orthogonal(&self) -> Direction {
        match self {
            Direction::N => Direction::E,
            Direction::S => Direction::W,
            Direction::E => Direction::S,
            Direction::W => Direction::N,
            Direction::Nw => Direction::Ne,
            Direction::Ne => Direction::Nw,
            Direction::Sw => Direction::Se,
            Direction::Se => Direction::Sw,
        }
    }
}

pub fn get_char(
    grid: &Grid,
    from_x: usize,
    from_y: usize,
    direction: &Direction,
    distance: isize,
) -> Option<(char, usize, usize)> {
    let (mut dx, mut dy) = direction.get_dx_dy();
    dx *= distance;
    dy *= distance;
    let x = from_x.checked_add_signed(dx)?;
    let y = from_y.checked_add_signed(dy)?;
    if y >= grid.len() {
        return None;
    }
    if x >= grid[0].len() {
        return None;
    }

    Some((grid[y][x], x, y))
}

pub fn is_char(
    grid: &Grid,
    from_x: usize,
    from_y: usize,
    direction: &Direction,
    distance: isize,
    character: char,
) -> bool {
    get_char(grid, from_x, from_y, direction, distance)
        .map(|c| c.0 == character)
        .is_some_and(|b| b)
}

pub fn count_connected(grid: &Grid, from_x: usize, from_y: usize) -> usize {
    let mut count = 0;
    for direction in [
        Direction::N,
        Direction::S,
        Direction::W,
        Direction::E,
        Direction::Nw,
        Direction::Ne,
        Direction::Sw,
        Direction::Se,
    ] {
        if is_char(grid, from_x, from_y, &direction, 1, 'M')
            && is_char(grid, from_x, from_y, &direction, 2, 'A')
            && is_char(grid, from_x, from_y, &direction, 3, 'S')
        {
            count += 1
        }
    }
    count
}

pub fn count_crossed(grid: &Grid, from_x: usize, from_y: usize) -> usize {
    for direction in [Direction::Nw, Direction::Ne, Direction::Sw, Direction::Se] {
        let orthogonal = direction.orthogonal();
        if is_char(grid, from_x, from_y, &direction, 1, 'S')
            && is_char(grid, from_x, from_y, &direction, -1, 'M')
            && ((is_char(grid, from_x, from_y, &orthogonal, 1, 'S')
                && is_char(grid, from_x, from_y, &orthogonal, -1, 'M'))
                || (is_char(grid, from_x, from_y, &orthogonal, 1, 'M')
                    && is_char(grid, from_x, from_y, &orthogonal, -1, 'S')))
        {
            return 1;
        }
    }
    0
}

pub fn count_xmas(grid: Grid) -> usize {
    let mut count = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'X' {
                count += count_connected(&grid, x, y)
            }
        }
    }
    count
}

pub fn count_x_mas(grid: Grid) -> usize {
    let mut count = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'A' {
                count += count_crossed(&grid, x, y)
            }
        }
    }
    count
}

#[allow(dead_code)]
pub fn part1() {
    let input = read_file(module_path!());
    println!("{}", count_xmas(parse_file(input.as_str())));
}

#[allow(dead_code)]
pub fn part2() {
    let input = read_file(module_path!());
    println!("{}", count_x_mas(parse_file(input.as_str())));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_parse_file() {
        let grid = parse_file(TEST_STR);
        assert_eq!(grid.len(), 10);
        assert_eq!(grid[0].len(), 10);
        assert_eq!(grid[1][2], 'A');
    }

    #[test]
    fn test_count_xmas() {
        assert_eq!(count_xmas(parse_file(TEST_STR)), 18)
    }
    #[test]
    fn test_count_x_mas() {
        assert_eq!(count_x_mas(parse_file(TEST_STR)), 9)
    }
}
