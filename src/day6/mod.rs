use std::collections::HashSet;

use crate::file::read_file;

type Position = (usize, usize);
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum MapTile {
    Obstacle,
    Open,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    fn get_dx_dy(&self) -> (isize, isize) {
        match self {
            Direction::N => (0, -1),
            Direction::S => (0, 1),
            Direction::E => (1, 0),
            Direction::W => (-1, 0),
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::N => Direction::E,
            Direction::S => Direction::W,
            Direction::E => Direction::S,
            Direction::W => Direction::N,
        }
    }
}
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Guard {
    position: Position,
    direction: Direction,
}

type MapGrid = Vec<Vec<MapTile>>;

pub fn parse_file(text: &str) -> (MapGrid, Guard) {
    let lines = text.split('\n');
    let guard = find_guard(lines.clone().collect());
    let grid: MapGrid = lines.map(parse_line).collect();
    (grid, guard)
}

pub fn parse_line(line: &str) -> Vec<MapTile> {
    line.chars()
        .map(|c| match c {
            '#' => MapTile::Obstacle,
            _ => MapTile::Open,
        })
        .collect()
}

pub fn find_guard(lines: Vec<&str>) -> Guard {
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '^' => {
                    return Guard {
                        position: (x, y),
                        direction: Direction::N,
                    }
                }
                _ => continue,
            }
        }
    }
    panic!("couldn't find guard!")
}

pub fn map_with_obstacle(grid: &MapGrid, position: Position) -> MapGrid {
    let mut new_grid = grid.clone();
    new_grid[position.1][position.0] = MapTile::Obstacle;
    new_grid
}

pub fn get_tile(
    grid: &MapGrid,
    position: Position,
    direction: &Direction,
    distance: isize,
) -> Option<(MapTile, Position)> {
    let (mut dx, mut dy) = direction.get_dx_dy();
    dx *= distance;
    dy *= distance;
    let x = position.0.checked_add_signed(dx)?;
    let y = position.1.checked_add_signed(dy)?;
    if y >= grid.len() {
        return None;
    }
    if x >= grid[0].len() {
        return None;
    }

    Some((grid[y][x], (x, y)))
}

pub fn step(guard: Guard, grid: &MapGrid) -> Option<Guard> {
    match get_tile(grid, guard.position, &guard.direction, 1) {
        Some(tile) => match tile.0 {
            MapTile::Obstacle => Some(Guard {
                position: guard.position,
                direction: guard.direction.turn_right(),
            }),
            MapTile::Open => Some(Guard {
                position: tile.1,
                direction: guard.direction,
            }),
        },
        None => None,
    }
}

pub fn count_positions(grid: &MapGrid, guard: Guard) -> usize {
    let mut position_set: HashSet<Position> = HashSet::new();
    let mut guard = guard;
    loop {
        position_set.insert(guard.position);
        let new_guard = step(guard, grid);
        match new_guard {
            Some(new_guard) => guard = new_guard,
            None => break,
        }
    }
    position_set.len()
}

pub fn is_loop(grid: &MapGrid, guard: Guard) -> bool {
    let mut guard_set: HashSet<Guard> = HashSet::new();
    let mut guard = guard;
    loop {
        if guard_set.contains(&guard) {
            return true;
        }
        guard_set.insert(guard);
        let new_guard = step(guard, grid);
        match new_guard {
            Some(new_guard) => guard = new_guard,
            None => break,
        }
    }
    false
}

pub fn find_loops(grid: &MapGrid, guard: Guard) -> usize {
    let mut loops = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if guard.position.0 == y && guard.position.1 == x {
                continue;
            }
            let new_grid = map_with_obstacle(grid, (x, y));
            if is_loop(&new_grid, guard) {
                loops += 1
            }
        }
    }
    loops
}

#[allow(dead_code)]
pub fn part1() {
    let input = read_file(module_path!());
    let (map, guard) = parse_file(&input);
    println!("{}", count_positions(&map, guard))
}

#[allow(dead_code)]
pub fn part2() {
    let input = read_file(module_path!());
    let (map, guard) = parse_file(&input);
    println!("{}", find_loops(&map, guard))
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_parse_file() {
        let (map, guard) = parse_file(TEST_STR);
        assert_eq!(guard.position, (4, 6));
        assert_eq!(guard.direction, Direction::N);
        assert_eq!(
            get_tile(&map, (9, 1), &Direction::N, 0),
            Some((MapTile::Obstacle, (9, 1)))
        );
        assert_eq!(
            get_tile(&map, (4, 0), &Direction::N, 0),
            Some((MapTile::Obstacle, (4, 0)))
        );
        assert_eq!(
            get_tile(&map, (4, 6), &Direction::N, 0),
            Some((MapTile::Open, (4, 6)))
        );
    }

    #[test]
    fn test_count_positions() {
        let (map, guard) = parse_file(TEST_STR);
        assert_eq!(count_positions(&map, guard), 41)
    }

    #[test]
    fn test_find_loops() {
        let (map, guard) = parse_file(TEST_STR);
        assert_eq!(find_loops(&map, guard), 6)
    }
}
