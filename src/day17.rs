use std::{
    cmp::Reverse,
    collections::{hash_map::Entry, BinaryHeap, HashMap},
};

struct Map {
    tiles: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl Map {
    fn from_str(input: &str) -> Self {
        let width = input.lines().next().unwrap().chars().count();
        let height = input.lines().count();
        let tiles = input
            .lines()
            .map(|l| l.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
            .collect();
        Self {
            tiles,
            width,
            height,
        }
    }

    fn cost(&self, (x, y): Pos) -> Option<u32> {
        if x < 0 || y < 0 || x >= self.width as i16 || y >= self.height as i16 {
            None
        } else {
            Some(self.tiles[y as usize][x as usize])
        }
    }
}
type Pos = (i16, i16);

#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct State {
    dir: Dir,
    steps: u8,
}

fn next_steps(
    map: &Map,
    cost: u32,
    (x, y): Pos,
    State { dir, steps }: State,
    to_visit: &mut BinaryHeap<(Reverse<u32>, Pos, State)>,
    part: Part,
) {
    let (min_steps, max_steps) = match part {
        Part::One => (0, 3),
        Part::Two => (4, 10),
    };
    let up = (x, y - 1);
    let down = (x, y + 1);
    let right = (x + 1, y);
    let left = (x - 1, y);
    let up_cost = map.cost(up).map(|c| c + cost);
    let down_cost = map.cost(down).map(|c| c + cost);
    let left_cost = map.cost(left).map(|c| c + cost);
    let right_cost = map.cost(right).map(|c| c + cost);
    #[rustfmt::skip]
    let next_up = up_cost.map(|c| {( Reverse(c), up, State {dir: Dir::Up, steps: 1})});
    #[rustfmt::skip]
    let next_down = down_cost.map(|c| {( Reverse(c), down, State {dir: Dir::Down, steps: 1})});
    #[rustfmt::skip]
    let next_left = left_cost.map(|c| {(Reverse(c), left,  State {dir: Dir::Left, steps: 1})});
    #[rustfmt::skip]
    let next_right = right_cost.map(|c| {(Reverse(c), right,  State {dir: Dir::Right, steps: 1}) });

    match dir {
        Dir::Up => {
            if steps < min_steps && up_cost.is_none() {
                return;
            }
            if steps < max_steps {
                if let Some(mut n) = next_up {
                    n.2.steps = 1 + steps;
                    to_visit.push(n);
                }
            }
            if steps >= min_steps {
                to_visit.extend(next_right);
                to_visit.extend(next_left);
            }
        }
        Dir::Down => {
            if steps < min_steps && down_cost.is_none() {
                return;
            }
            if steps < max_steps {
                if let Some(mut n) = next_down {
                    n.2.steps = 1 + steps;
                    to_visit.push(n);
                }
            }
            if steps >= min_steps {
                to_visit.extend(next_right);
                to_visit.extend(next_left);
            }
        }
        Dir::Left => {
            if steps < min_steps && left_cost.is_none() {
                return;
            }
            if steps < max_steps {
                if let Some(mut n) = next_left {
                    n.2.steps = 1 + steps;
                    to_visit.push(n);
                }
            }
            if steps >= min_steps {
                to_visit.extend(next_up);
                to_visit.extend(next_down);
            }
        }
        Dir::Right => {
            if steps < min_steps && right_cost.is_none() {
                return;
            }
            if steps < max_steps {
                if let Some(mut n) = next_right {
                    n.2.steps = 1 + steps;
                    to_visit.push(n);
                }
            }
            if steps >= min_steps {
                to_visit.extend(next_up);
                to_visit.extend(next_down);
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Part {
    One,
    Two,
}

fn search(map: Map, part: Part) -> u32 {
    let mut min_cost = HashMap::new();
    let mut to_visit_ = BinaryHeap::new();
    to_visit_.push((
        Reverse(0),
        (0, 0),
        State {
            dir: Dir::Right,
            steps: 0,
        },
    ));
    to_visit_.push((
        Reverse(0),
        (0, 0),
        State {
            dir: Dir::Down,
            steps: 0,
        },
    ));
    let mut best = u32::MAX;
    while let Some((Reverse(cost), pos, state)) = to_visit_.pop() {
        if cost > best {
            continue;
        }
        match min_cost.entry((pos, state)) {
            Entry::Occupied(mut entry) => {
                let min = entry.get_mut();
                if *min <= cost {
                    continue;
                }
                *min = cost;
            }
            Entry::Vacant(vacant) => {
                vacant.insert(cost);
            }
        };

        match part {
            Part::One => {
                if pos == (map.width as i16 - 1, map.height as i16 - 1) {
                    best = best.min(cost);
                    continue;
                }
            }
            Part::Two => {
                if pos == (map.width as i16 - 1, map.height as i16 - 1) && state.steps >= 4 {
                    best = best.min(cost);
                    continue;
                }
            }
        }
        next_steps(&map, cost, pos, state, &mut to_visit_, part);
    }
    best
}

pub fn part_1(input: &str) -> u32 {
    let map = Map::from_str(input);
    search(map, Part::One)
}
pub fn part_2(input: &str) -> u32 {
    let map = Map::from_str(input);
    search(map, Part::Two)
}

#[cfg(test)]
mod tests {
    use crate::day17::*;
    use std::fs;
    #[test]
    fn example17_part1() {
        let input = fs::read_to_string("input/example17").unwrap();
        assert_eq!(part_1(&input), 102);
    }
    #[test]
    fn day17_part1() {
        let input = fs::read_to_string("input/day17").unwrap();
        assert_eq!(part_1(&input), 742);
    }
    #[test]
    fn example17_part2() {
        let input = fs::read_to_string("input/example17").unwrap();
        assert_eq!(part_2(&input), 94);
    }
    #[test]
    fn example17_part2_2() {
        let input = fs::read_to_string("input/example17_2").unwrap();
        assert_eq!(part_2(&input), 71);
    }

    #[test]
    fn day17_part2() {
        let input = fs::read_to_string("input/day17").unwrap();
        assert_eq!(part_2(&input), 918);
    }
}
