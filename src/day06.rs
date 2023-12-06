fn parse_times_distances(input: &str) -> (Vec<u64>, Vec<u64>) {
    let (times, distances) = input.split_once('\n').unwrap();
    let times: Vec<u64> = times
        .trim_start_matches("Time:")
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let distances: Vec<u64> = distances
        .trim_start_matches("Distance:")
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    (times, distances)
}

fn ways_to_win(time: u64, distance: u64) -> u64 {
    (1..time)
        .map(|wait| {
            let travel = (time - wait) * wait;
            u64::from(travel > distance)
        })
        .sum()
}

pub fn part_1(input: &str) -> u64 {
    let (times, distances) = parse_times_distances(input);
    times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| ways_to_win(*t, *d))
        .product()
}

fn parse_times_distances_part2(input: &str) -> (u64, u64) {
    let (times, distances) = input.split_once('\n').unwrap();
    let time: u64 = times
        .trim_start_matches("Time:")
        .split_whitespace()
        .collect::<String>()
        .parse()
        .unwrap();
    let distance: u64 = distances
        .trim_start_matches("Distance:")
        .split_whitespace()
        .collect::<String>()
        .parse()
        .unwrap();
    (time, distance)
}

pub fn part_2(input: &str) -> u64 {
    let (time, distance) = parse_times_distances_part2(input);
    ways_to_win(time, distance)
}

#[cfg(test)]
mod tests {
    use crate::day06::*;
    use std::fs;
    #[test]
    fn example06_part1() {
        let input = fs::read_to_string("input/example06").unwrap();
        assert_eq!(part_1(&input), 288);
    }
    #[test]
    fn day06_part1() {
        let input = fs::read_to_string("input/day06").unwrap();
        assert_eq!(part_1(&input), 440000);
    }
    #[test]
    fn example06_part2() {
        let input = fs::read_to_string("input/example06").unwrap();
        assert_eq!(part_2(&input), 71503);
    }
    #[test]
    fn day06_part2() {
        let input = fs::read_to_string("input/day06").unwrap();
        assert_eq!(part_2(&input), 26187338);
    }
}
