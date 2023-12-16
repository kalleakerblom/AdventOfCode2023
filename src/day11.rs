use itertools::Itertools;
use std::collections::HashSet;

type Pos = (i64, i64);
struct Map {
    occupied_columns: HashSet<i64>,
    occupied_rows: HashSet<i64>,
    galaxies: Vec<Pos>,
    expansion_factor: u64,
}

enum Part {
    One,
    Two,
}

impl Map {
    fn from_str(s: &str, part: Part) -> Self {
        let mut occupied_columns = HashSet::<i64>::new();
        let mut occupied_rows = HashSet::<i64>::new();
        let mut galaxies = vec![];
        s.lines()
            .enumerate()
            .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, ch)| (x, y, ch)))
            .filter(|(_, _, ch)| *ch == '#')
            .for_each(|(x, y, _ch)| {
                occupied_columns.insert(x as i64);
                occupied_rows.insert(y as i64);
                galaxies.push((x as i64, y as i64))
            });
        let expansion_factor = if matches!(part, Part::One) {
            2
        } else {
            1_000_000
        };
        Map {
            occupied_columns,
            occupied_rows,
            galaxies,
            expansion_factor,
        }
    }
    fn distance(&self, p1: Pos, p2: Pos) -> u64 {
        let mut dist = 0;
        let dx = (p2.0 - p1.0).signum();
        let dy = (p2.1 - p1.1).signum();
        let mut p_x = p1.0;
        while p_x != p2.0 {
            p_x += dx;
            if self.occupied_columns.contains(&p_x) {
                dist += 1;
            } else {
                dist += self.expansion_factor;
            }
        }
        let mut p_y = p1.1;
        while p_y != p2.1 {
            p_y += dy;
            if self.occupied_rows.contains(&p_y) {
                dist += 1;
            } else {
                dist += self.expansion_factor;
            }
        }
        dist
    }
    fn distance_sum(&self) -> u64 {
        self.galaxies
            .iter()
            .tuple_combinations()
            .map(|(p1, p2)| self.distance(*p1, *p2))
            .sum()
    }
}

pub fn part_1(input: &str) -> u64 {
    let map = Map::from_str(input, Part::One);
    map.distance_sum()
}
pub fn part_2(input: &str) -> u64 {
    let map = Map::from_str(input, Part::Two);
    map.distance_sum()
}

#[cfg(test)]
mod tests {
    use crate::day11::*;
    use std::fs;
    #[test]
    fn example11_part1() {
        let input = fs::read_to_string("input/example11").unwrap();
        assert_eq!(part_1(&input), 374);
    }
    #[test]
    fn day11_part1() {
        let input = fs::read_to_string("input/day11").unwrap();
        assert_eq!(part_1(&input), 9521776);
    }
    #[test]
    fn example11_part2() {
        let input = fs::read_to_string("input/example11").unwrap();
        assert_eq!(part_2(&input), 82000210);
    }
    #[test]
    fn day11_part2() {
        let input = fs::read_to_string("input/day11").unwrap();
        assert_eq!(part_2(&input), 553224415344);
    }
}
