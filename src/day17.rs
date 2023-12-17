use std::collections::{hash_map::Entry, HashMap};

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

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct State {
    dir: Dir,
    steps: u8,
}
//(pos, cost, state)
fn next_steps(map: &Map, cost: u32, (x, y): Pos, steps: u8, dir: Dir) -> Vec<(Pos, u32, State)> {
    let mut next = vec![];
    let up = (x, y - 1);
    let down = (x, y + 1);
    let right = (x + 1, y);
    let left = (x - 1, y);
    let up_cost = map.cost(up).map(|c| c + cost);
    let down_cost = map.cost(down).map(|c| c + cost);
    let left_cost = map.cost(left).map(|c| c + cost);
    let right_cost = map.cost(right).map(|c| c + cost);
    #[rustfmt::skip]
    let next_up = up_cost.map(|c| {(up, c, State {dir: Dir::Up, steps: 1})});
    #[rustfmt::skip]
    let next_down = down_cost.map(|c| {(down, c, State {dir: Dir::Down, steps: 1})});
    #[rustfmt::skip]
    let next_left = left_cost.map(|c| {(left, c, State {dir: Dir::Left, steps: 1})});
    #[rustfmt::skip]
    let next_right = right_cost.map(|c| {(right, c, State {dir: Dir::Right, steps: 1}) });

    match dir {
        Dir::Up => {
            if steps < 3 && up_cost.is_some() {
                next.push((
                    up,
                    up_cost.unwrap(),
                    State {
                        dir: Dir::Up,
                        steps: steps + 1,
                    },
                ));
            }
            next.extend(next_right);
            next.extend(next_left);
        }
        Dir::Down => {
            if steps < 3 && down_cost.is_some() {
                next.push((
                    down,
                    down_cost.unwrap(),
                    State {
                        dir: Dir::Down,
                        steps: steps + 1,
                    },
                ));
            }
            next.extend(next_right);
            next.extend(next_left);
        }
        Dir::Left => {
            if steps < 3 && left_cost.is_some() {
                next.push((
                    left,
                    left_cost.unwrap(),
                    State {
                        dir: Dir::Left,
                        steps: steps + 1,
                    },
                ));
            }
            next.extend(next_up);
            next.extend(next_down);
        }
        Dir::Right => {
            if steps < 3 && right_cost.is_some() {
                next.push((
                    right,
                    right_cost.unwrap(),
                    State {
                        dir: Dir::Right,
                        steps: steps + 1,
                    },
                ));
            }
            next.extend(next_up);
            next.extend(next_down);
        }
    }
    next
}

fn next_steps_part2(
    map: &Map,
    cost: u32,
    (x, y): Pos,
    steps: u8,
    dir: Dir,
) -> Vec<(Pos, u32, State)> {
    let mut next = vec![];
    let up = (x, y - 1);
    let down = (x, y + 1);
    let right = (x + 1, y);
    let left = (x - 1, y);
    let up_cost = map.cost(up).map(|c| c + cost);
    let down_cost = map.cost(down).map(|c| c + cost);
    let left_cost = map.cost(left).map(|c| c + cost);
    let right_cost = map.cost(right).map(|c| c + cost);
    #[rustfmt::skip]
    let next_up = up_cost.map(|c| {(up, c, State {dir: Dir::Up, steps: 1})});
    #[rustfmt::skip]
    let next_down = down_cost.map(|c| {(down, c, State {dir: Dir::Down, steps: 1})});
    #[rustfmt::skip]
    let next_left = left_cost.map(|c| {(left, c, State {dir: Dir::Left, steps: 1})});
    #[rustfmt::skip]
    let next_right = right_cost.map(|c| {(right, c, State {dir: Dir::Right, steps: 1}) });

    match dir {
        Dir::Up => {
            if steps < 4 && up_cost.is_none() {
                return vec![];
            }
            if steps < 10 && up_cost.is_some() {
                next.push((
                    up,
                    up_cost.unwrap(),
                    State {
                        dir: Dir::Up,
                        steps: steps + 1,
                    },
                ));
            }
            if steps >= 4 {
                next.extend(next_right);
                next.extend(next_left);
            }
        }
        Dir::Down => {
            if steps < 4 && down_cost.is_none() {
                return vec![];
            }
            if steps < 10 && down_cost.is_some() {
                next.push((
                    down,
                    down_cost.unwrap(),
                    State {
                        dir: Dir::Down,
                        steps: steps + 1,
                    },
                ));
            }
            if steps >= 4 {
                next.extend(next_right);
                next.extend(next_left);
            }
        }
        Dir::Left => {
            if steps < 4 && left_cost.is_none() {
                return vec![];
            }
            if steps < 10 && left_cost.is_some() {
                next.push((
                    left,
                    left_cost.unwrap(),
                    State {
                        dir: Dir::Left,
                        steps: steps + 1,
                    },
                ));
            }
            if steps >= 4 {
                next.extend(next_up);
                next.extend(next_down);
            }
        }
        Dir::Right => {
            if steps < 4 && right_cost.is_none() {
                return vec![];
            }
            if steps < 10 && right_cost.is_some() {
                next.push((
                    right,
                    right_cost.unwrap(),
                    State {
                        dir: Dir::Right,
                        steps: steps + 1,
                    },
                ));
            }
            if steps >= 4 {
                next.extend(next_up);
                next.extend(next_down);
            }
        }
    }
    next
}

enum Part {
    One,
    Two,
}

fn search(map: Map, part: Part) -> u32 {
    let mut min_cost = HashMap::new();
    let mut to_visit = vec![
        (
            (0, 0),
            0,
            State {
                dir: Dir::Right,
                steps: 0,
            },
        ),
        (
            (0, 0),
            0,
            State {
                dir: Dir::Down,
                steps: 0,
            },
        ),
    ];
    let mut best = u32::MAX;
    const MAX_COST: u32 = 1000;
    while let Some((pos, cost, state)) = to_visit.pop() {
        if cost > best || cost > 1000 {
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
                to_visit.extend(next_steps(&map, cost, pos, state.steps, state.dir));
            }
            Part::Two => {
                if pos == (map.width as i16 - 1, map.height as i16 - 1) && state.steps >= 4 {
                    best = best.min(cost);
                    continue;
                }
                to_visit.extend(next_steps_part2(&map, cost, pos, state.steps, state.dir));
            }
        }
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
