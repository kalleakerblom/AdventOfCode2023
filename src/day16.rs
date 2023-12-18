use std::collections::HashSet;

struct Map {
    tiles: Vec<Vec<char>>,
    width: usize,
    height: usize,
}
impl Map {
    fn from_str(input: &str) -> Self {
        let width = input.lines().next().unwrap().chars().count();
        let height = input.lines().count();
        let tiles = input.lines().map(|l| l.chars().collect()).collect();
        Self {
            tiles,
            width,
            height,
        }
    }
}
type Pos = (i32, i32);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct Beam {
    dir: Dir,
    pos: Pos,
}

impl Beam {
    fn next_pos(&mut self, map: &Map) -> bool {
        let next_pos = match self.dir {
            Dir::Up => (self.pos.0, self.pos.1 - 1),
            Dir::Down => (self.pos.0, self.pos.1 + 1),
            Dir::Left => (self.pos.0 - 1, self.pos.1),
            Dir::Right => (self.pos.0 + 1, self.pos.1),
        };
        if next_pos.0 < 0
            || next_pos.1 < 0
            || next_pos.0 == map.width as i32
            || next_pos.1 == map.height as i32
        {
            return false;
        }
        self.pos = next_pos;
        true
    }
}

fn run_beam(start_pos: Pos, start_dir: Dir, map: &Map) -> usize {
    let mut visited = HashSet::<Pos>::new();
    let mut visited_with_dir = HashSet::<(Pos, Dir)>::new();
    let mut beams = vec![Beam {
        dir: start_dir,
        pos: start_pos,
    }];
    while let Some(mut beam) = beams.pop() {
        while beam.next_pos(map) {
            if !visited_with_dir.insert((beam.pos, beam.dir)) {
                break;
            }
            visited.insert(beam.pos);
            match (
                map.tiles[beam.pos.1 as usize][beam.pos.0 as usize],
                beam.dir,
            ) {
                ('|', Dir::Left | Dir::Right) => {
                    beams.push(Beam {
                        dir: Dir::Up,
                        pos: beam.pos,
                    });
                    beams.push(Beam {
                        dir: Dir::Down,
                        pos: beam.pos,
                    });
                    break;
                }
                ('-', Dir::Up | Dir::Down) => {
                    beams.push(Beam {
                        dir: Dir::Left,
                        pos: beam.pos,
                    });
                    beams.push(Beam {
                        dir: Dir::Right,
                        pos: beam.pos,
                    });
                    break;
                }
                ('/', dir) => {
                    beam.dir = match dir {
                        Dir::Up => Dir::Right,
                        Dir::Down => Dir::Left,
                        Dir::Left => Dir::Down,
                        Dir::Right => Dir::Up,
                    }
                }
                ('\\', dir) => {
                    beam.dir = match dir {
                        Dir::Up => Dir::Left,
                        Dir::Down => Dir::Right,
                        Dir::Left => Dir::Up,
                        Dir::Right => Dir::Down,
                    }
                }
                _ => (),
            }
        }
    }
    visited.len()
}
pub fn part_1(input: &str) -> usize {
    let map = Map::from_str(input);
    run_beam((-1, 0), Dir::Right, &map)
}

pub fn part_2(input: &str) -> usize {
    let map = Map::from_str(input);
    let mut max = 0;
    // going down
    for start_x in 0..map.width {
        let energy = run_beam((start_x as i32, -1), Dir::Down, &map);
        max = max.max(energy);
    }
    // going up
    for start_x in 0..map.width {
        let energy = run_beam((start_x as i32, map.height as i32), Dir::Up, &map);
        max = max.max(energy);
    }
    // going right
    for start_y in 0..map.height {
        let energy = run_beam((-1, start_y as i32), Dir::Right, &map);
        max = max.max(energy);
    }
    // going left
    for start_y in 0..map.height {
        let energy = run_beam((map.width as i32, start_y as i32), Dir::Left, &map);
        max = max.max(energy);
    }
    max
}

#[cfg(test)]
mod tests {
    use crate::day16::*;
    use std::fs;
    #[test]
    fn example16_part1() {
        let input = fs::read_to_string("input/example16").unwrap();
        assert_eq!(part_1(&input), 46);
    }
    #[test]
    fn day16_part1() {
        let input = fs::read_to_string("input/day16").unwrap();
        assert_eq!(part_1(&input), 7482);
    }
    #[test]
    fn example16_part2() {
        let input = fs::read_to_string("input/example16").unwrap();
        assert_eq!(part_2(&input), 51);
    }
    #[test]
    fn day16_part2() {
        let input = fs::read_to_string("input/day16").unwrap();
        assert_eq!(part_2(&input), 7896);
    }
}
