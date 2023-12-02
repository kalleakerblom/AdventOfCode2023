const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

struct Reveal {
    red: usize,
    green: usize,
    blue: usize,
}

impl Reveal {
    fn from_str(s: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for part in s.split(", ") {
            let (count, color) = part.split_once(' ').unwrap();
            match color {
                "red" => red = count.parse().unwrap(),
                "green" => green = count.parse().unwrap(),
                "blue" => blue = count.parse().unwrap(),
                _ => panic!(),
            }
        }
        Self { red, green, blue }
    }
}

struct Game {
    reveals: Vec<Reveal>,
}

impl Game {
    fn from_str(line: &str) -> Self {
        let (_, rest) = line.split_once(": ").unwrap();
        let reveals: Vec<_> = rest.split("; ").map(Reveal::from_str).collect();
        Game { reveals }
    }
    fn is_valid(&self) -> bool {
        self.reveals
            .iter()
            .all(|rev| rev.red <= MAX_RED && rev.green <= MAX_GREEN && rev.blue <= MAX_BLUE)
    }
    fn power(&self) -> usize {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for rev in &self.reveals {
            max_red = max_red.max(rev.red);
            max_green = max_green.max(rev.green);
            max_blue = max_blue.max(rev.blue)
        }
        max_red * max_green * max_blue
    }
}

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(Game::from_str)
        .enumerate()
        .filter_map(|(i, game)| game.is_valid().then_some(i + 1))
        .sum::<usize>()
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(Game::from_str)
        .map(|game| game.power())
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day02::*;
    use std::fs;
    #[test]
    fn example02_part1() {
        let input = fs::read_to_string("input/example02").unwrap();
        assert_eq!(part_1(&input), 8);
    }
    #[test]
    fn day02_part1() {
        let input = fs::read_to_string("input/day02").unwrap();
        assert_eq!(part_1(&input), 2551);
    }
    #[test]
    fn example02_part2() {
        let input = fs::read_to_string("input/example02").unwrap();
        assert_eq!(part_2(&input), 2286);
    }
    #[test]
    fn day02_part2() {
        let input = fs::read_to_string("input/day02").unwrap();
        assert_eq!(part_2(&input), 62811);
    }
}
