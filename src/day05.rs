struct Mapper {
    destination_start: i64,
    source_start: i64,
    len: i64,
}

impl Mapper {
    fn from_str(line: &str) -> Self {
        let numbers: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        Mapper {
            destination_start: numbers[0],
            source_start: numbers[1],
            len: numbers[2],
        }
    }

    fn try_map(&self, source: i64) -> Option<i64> {
        if source < self.source_start || self.source_start + self.len <= source {
            return None;
        }
        let destination = source - self.source_start + self.destination_start;
        Some(destination)
    }
}

fn parse_block(block: &str) -> Vec<Mapper> {
    block.lines().skip(1).map(Mapper::from_str).collect()
}

fn seed_to_location(mut seed: i64, maps: &[Vec<Mapper>]) -> i64 {
    for map in maps {
        for mapper in map {
            if let Some(mapped) = mapper.try_map(seed) {
                seed = mapped;
                break;
            }
        }
    }
    seed
}

pub fn part_1(input: &str) -> i64 {
    let (seeds, rest) = input.split_once("\n\n").unwrap();
    let seeds: Vec<i64> = seeds
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let maps: Vec<Vec<Mapper>> = rest.split("\n\n").map(parse_block).collect();
    let min_location = seeds
        .iter()
        .map(|s| seed_to_location(*s, &maps))
        .min()
        .unwrap();
    min_location
}
pub fn part_2(input: &str) -> i64 {
    let (seeds, rest) = input.split_once("\n\n").unwrap();
    let seeds: Vec<i64> = seeds
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let maps: Vec<Vec<Mapper>> = rest.split("\n\n").map(parse_block).collect();
    let min_location = seeds
        .chunks_exact(2)
        .map(|chunk| {
            (chunk[0]..chunk[0] + chunk[1])
                .map(|s| seed_to_location(s, &maps))
                .min()
                .unwrap()
        })
        .min()
        .unwrap();
    min_location
}

#[cfg(test)]
mod tests {
    use crate::day05::*;
    use std::fs;
    #[test]
    fn example05_part1() {
        let input = fs::read_to_string("input/example05").unwrap();
        assert_eq!(part_1(&input), 35);
    }
    #[test]
    fn day05_part1() {
        let input = fs::read_to_string("input/day05").unwrap();
        assert_eq!(part_1(&input), 3374647);
    }
    #[test]
    fn example05_part2() {
        let input = fs::read_to_string("input/example05").unwrap();
        assert_eq!(part_2(&input), 46);
    }
    #[test]
    fn day05_part2() {
        let input = fs::read_to_string("input/day05").unwrap();
        assert_eq!(part_2(&input), 6082852);
    }
}
