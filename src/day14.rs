enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Air,
    Ball,
    Cube,
}
struct Panel {
    tiles: Vec<Vec<Tile>>,
}

impl Panel {
    fn from_str(s: &str) -> Self {
        let tiles = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|ch| match ch {
                        '.' => Tile::Air,
                        '#' => Tile::Cube,
                        'O' => Tile::Ball,
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect();
        Self { tiles }
    }

    fn tilt_north(&mut self) {
        let height = self.tiles.len();
        let width = self.tiles[0].len();
        for row in 0..height {
            for col in 0..width {
                if !matches!(self.tiles[row][col], Tile::Ball) {
                    continue;
                }
                let mut stop = row;
                while stop != 0 && self.tiles[stop - 1][col] == Tile::Air {
                    stop -= 1;
                }
                self.tiles[row][col] = Tile::Air;
                self.tiles[stop][col] = Tile::Ball;
            }
        }
    }

    fn tilt_south(&mut self) {
        let height = self.tiles.len();
        let width = self.tiles[0].len();
        for row in (0..height).rev() {
            for col in 0..width {
                if !matches!(self.tiles[row][col], Tile::Ball) {
                    continue;
                }
                let mut stop = row;
                while stop != height - 1 && self.tiles[stop + 1][col] == Tile::Air {
                    stop += 1;
                }
                self.tiles[row][col] = Tile::Air;
                self.tiles[stop][col] = Tile::Ball;
            }
        }
    }

    fn tilt_west(&mut self) {
        let height = self.tiles.len();
        let width = self.tiles[0].len();
        for col in 0..width {
            for row in 0..height {
                if !matches!(self.tiles[row][col], Tile::Ball) {
                    continue;
                }
                let mut stop = col;
                while stop != 0 && self.tiles[row][stop - 1] == Tile::Air {
                    stop -= 1;
                }
                self.tiles[row][col] = Tile::Air;
                self.tiles[row][stop] = Tile::Ball;
            }
        }
    }

    fn tilt_east(&mut self) {
        let height = self.tiles.len();
        let width = self.tiles[0].len();
        for col in (0..width).rev() {
            for row in 0..height {
                if !matches!(self.tiles[row][col], Tile::Ball) {
                    continue;
                }
                let mut stop = col;
                while stop != width - 1 && self.tiles[row][stop + 1] == Tile::Air {
                    stop += 1;
                }
                self.tiles[row][col] = Tile::Air;
                self.tiles[row][stop] = Tile::Ball;
            }
        }
    }

    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn north_load(&self) -> usize {
        let height = self.tiles.len();
        self.tiles
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, t)| (x, y, t)))
            .filter(|(_, _, t)| **t == Tile::Ball)
            .map(|(_, y, _)| height - y)
            .sum()
    }

    fn print(&self) {
        for row in &self.tiles {
            let line: String = row
                .iter()
                .map(|t| match t {
                    Tile::Air => '.',
                    Tile::Ball => 'O',
                    Tile::Cube => '#',
                })
                .collect();
            println!("{line}");
        }
        println!();
    }
}

pub fn part_1(input: &str) -> usize {
    let mut panel = Panel::from_str(input);
    panel.tilt_north();
    panel.north_load()
}

pub fn part_2(input: &str) -> usize {
    let mut panel = Panel::from_str(input);
    let mut cycles_remaining = 1000000000;
    for _ in 0..100 {
        panel.cycle();
    }
    cycles_remaining -= 100;
    let mut measured_loads = vec![panel.north_load()];
    panel.cycle();
    cycles_remaining -= 1;
    measured_loads.push(panel.north_load());
    let mut len = 2;
    while measured_loads[..len / 2] != measured_loads[len / 2..] {
        panel.cycle();
        measured_loads.push(panel.north_load());
        panel.cycle();
        measured_loads.push(panel.north_load());
        len += 2;
        cycles_remaining -= 2;
    }
    // found repeating pattern
    let period = len / 2;
    let remaining = cycles_remaining % period;
    for _ in 0..remaining {
        panel.cycle();
    }
    panel.north_load()
}

#[cfg(test)]
mod tests {
    use crate::day14::*;
    use std::fs;
    #[test]
    fn example14_part1() {
        let input = fs::read_to_string("input/example14").unwrap();
        assert_eq!(part_1(&input), 136);
    }
    #[test]
    fn day14_part1() {
        let input = fs::read_to_string("input/day14").unwrap();
        assert_eq!(part_1(&input), 107053);
    }
    #[test]
    fn example14_part2() {
        let input = fs::read_to_string("input/example14").unwrap();
        assert_eq!(part_2(&input), 64);
    }
    #[test]
    fn day14_part2() {
        let input = fs::read_to_string("input/day14").unwrap();
        assert_eq!(part_2(&input), 88371);
    }
}
