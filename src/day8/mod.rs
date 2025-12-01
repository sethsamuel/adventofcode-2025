use crate::file::read_file;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(isize, isize);

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl std::ops::Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Position(self.0 - rhs.0, self.1 - rhs.1)
    }
}

type Antenna = char;
type Antennas = HashMap<Antenna, HashSet<Position>>;
type AntennaPositions = HashMap<Position, Antenna>;

/// Returns a map of antenna positions and the grid width/height
pub fn parse_file(text: &str) -> (Antennas, AntennaPositions, usize, usize) {
    let lines = text.split('\n');
    let width = lines.clone().count();
    let height = lines.clone().next().unwrap().len();

    let mut antennas: Antennas = HashMap::new();

    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas
                    .entry(c)
                    .or_default()
                    .insert(Position(x.try_into().unwrap(), y.try_into().unwrap()));
            }
        }
    }

    let mut antenna_positions: AntennaPositions = HashMap::new();
    for (a, ps) in antennas.clone() {
        for p in ps {
            antenna_positions.insert(p, a);
        }
    }

    (antennas, antenna_positions, width, height)
}

pub fn get_antinodes(
    antennas: &Antennas,
    _antenna_positions: &AntennaPositions,
    width: usize,
    height: usize,
) -> HashSet<Position> {
    let mut antinodes: HashSet<Position> = HashSet::new();

    for positions in antennas.values() {
        for p1 in positions {
            for p2 in positions {
                if p1 == p2 {
                    continue;
                }
                antinodes.insert(*p1);
                let v = *p2 - *p1;
                let mut a1: Position = *p1 - v;
                while (0..width as isize).contains(&a1.0) && (0..height as isize).contains(&a1.1) {
                    antinodes.insert(a1);
                    a1 = a1 - v;
                }
            }
        }
    }

    antinodes
}

#[allow(dead_code)]
pub fn part1() {
    let input = read_file(module_path!());
    let (antennas, antenna_positions, width, height) = parse_file(&input);
    let antinodes = get_antinodes(&antennas, &antenna_positions, width, height);

    println!("{}", antinodes.len());
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

    static TEST_STR: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_parse_file() {
        let (antennas, positions, width, height) = parse_file(TEST_STR);
        assert_eq!(width, 12);
        assert_eq!(height, 12);
        assert_eq!(antennas.get(&'0').unwrap().len(), 4);
        assert!(antennas.get(&'0').unwrap().contains(&Position(8, 1)));
        assert_eq!(positions.get(&Position(8, 1)).unwrap(), &'0');
    }

    #[test]
    fn test_get_antinodes() {
        let (antennas, antenna_positions, width, height) = parse_file(TEST_STR);
        let antinodes = get_antinodes(&antennas, &antenna_positions, width, height);
        // assert_eq!(antinodes.len(), 14);
        assert_eq!(antinodes.len(), 34);
    }
}
