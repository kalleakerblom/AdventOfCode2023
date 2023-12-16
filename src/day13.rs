use std::{collections::HashMap, mem};

struct Pattern {
    rows: Vec<Vec<bool>>,
    columns: Vec<Vec<bool>>,
    width: usize,
    height: usize,
    memo_row_diff: HashMap<(usize, usize), usize>,
    memo_col_diff: HashMap<(usize, usize), usize>,
}

#[derive(Debug)]
enum Reflection {
    Row(usize),
    Column(usize),
}

impl Pattern {
    fn from_str(s: &str) -> Self {
        let width = s.lines().next().unwrap().chars().count();
        let height = s.lines().count();
        let mut rows = vec![vec![false; width]; height];
        let mut columns = vec![vec![false; height]; width];
        s.lines()
            .enumerate()
            .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, ch)| (x, y, ch)))
            .filter(|(_, _, ch)| *ch == '#')
            .for_each(|(x, y, _ch)| {
                rows[y][x] = true;
                columns[x][y] = true;
            });

        Pattern {
            rows,
            columns,
            width,
            height,
            memo_row_diff: HashMap::new(),
            memo_col_diff: HashMap::new(),
        }
    }
    fn find_reflection(&self) -> Reflection {
        // column-wise
        'outer: for _left in 0..self.width - 1 {
            let mut left = _left;
            let mut right = _left + 1;
            loop {
                if self.columns[left] != self.columns[right] {
                    continue 'outer;
                }
                if left == 0 || right == self.width - 1 {
                    break;
                }
                left -= 1;
                right += 1;
            }
            return Reflection::Column(_left + 1);
        }
        // row-wise
        'outer: for _up in 0..self.height - 1 {
            let mut up = _up;
            let mut down = _up + 1;
            loop {
                if self.rows[up] != self.rows[down] {
                    continue 'outer;
                }
                if up == 0 || down == self.height - 1 {
                    break;
                }
                up -= 1;
                down += 1;
            }
            return Reflection::Row(_up + 1);
        }
        panic!("No reflection!")
    }

    fn find_reflection_part2(&mut self) -> Reflection {
        // column-wise
        for _left in 0..self.width - 1 {
            let mut left = _left;
            let mut right = _left + 1;
            let mut total_diff = 0;
            loop {
                total_diff += self.col_diff(left, right);
                if left == 0 || right == self.width - 1 {
                    break;
                }
                left -= 1;
                right += 1;
            }
            if total_diff == 1 {
                return Reflection::Column(_left + 1);
            }
        }
        // row-wise
        for _up in 0..self.height - 1 {
            let mut up = _up;
            let mut down = _up + 1;
            let mut total_diff = 0;
            loop {
                total_diff += self.row_diff(up, down);

                if up == 0 || down == self.height - 1 {
                    break;
                }
                up -= 1;
                down += 1;
            }
            if total_diff == 1 {
                return Reflection::Row(_up + 1);
            }
        }
        panic!("No reflection!")
    }

    fn row_diff(&mut self, mut r1: usize, mut r2: usize) -> usize {
        if r1 > r2 {
            mem::swap(&mut r1, &mut r2);
        }
        if let Some(memo) = self.memo_row_diff.get(&(r1, r2)) {
            return *memo;
        }
        let diff = self.rows[r1]
            .iter()
            .zip(self.rows[r2].iter())
            .filter(|(b1, b2)| b1 != b2)
            .count();
        self.memo_row_diff.insert((r1, r2), diff);
        diff
    }

    fn col_diff(&mut self, mut c1: usize, mut c2: usize) -> usize {
        if c1 > c2 {
            mem::swap(&mut c1, &mut c2);
        }
        if let Some(memo) = self.memo_col_diff.get(&(c1, c2)) {
            return *memo;
        }
        let diff = self.columns[c1]
            .iter()
            .zip(self.columns[c2].iter())
            .filter(|(b1, b2)| b1 != b2)
            .count();
        self.memo_col_diff.insert((c1, c2), diff);
        diff
    }
}

pub fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|p| Pattern::from_str(p).find_reflection())
        .map(|refl| match refl {
            Reflection::Row(r) => 100 * r,
            Reflection::Column(c) => c,
        })
        .sum()
}
pub fn part_2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|p| Pattern::from_str(p).find_reflection_part2())
        .map(|refl| match refl {
            Reflection::Row(r) => 100 * r,
            Reflection::Column(c) => c,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day13::*;
    use std::fs;
    #[test]
    fn example13_part1() {
        let input = fs::read_to_string("input/example13").unwrap();
        assert_eq!(part_1(&input), 405);
    }
    #[test]
    fn day13_part1() {
        let input = fs::read_to_string("input/day13").unwrap();
        assert_eq!(part_1(&input), 35360);
    }
    #[test]
    fn example13_part2() {
        let input = fs::read_to_string("input/example13").unwrap();
        assert_eq!(part_2(&input), 400);
    }
    #[test]
    fn day13_part2() {
        let input = fs::read_to_string("input/day13").unwrap();
        assert_eq!(part_2(&input), 0);
    }
}
