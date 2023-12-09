fn predict(vals: &[i32]) -> i32 {
    if vals.iter().all(|v| *v == 0) {
        return 0;
    }
    let diffs: Vec<_> = vals.windows(2).map(|w| w[1] - w[0]).collect();
    let predicted_diff = predict(&diffs);
    let predicted = vals.last().unwrap() + predicted_diff;
    predicted
}

fn predict_back(vals: &[i32]) -> i32 {
    if vals.iter().all(|v| *v == 0) {
        return 0;
    }
    let diffs: Vec<_> = vals.windows(2).map(|w| w[1] - w[0]).collect();
    let predicted_diff = predict_back(&diffs);
    let predicted = vals.first().unwrap() - predicted_diff;
    predicted
}

pub fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            let vals: Vec<_> = l.split_whitespace().map(|n| n.parse().unwrap()).collect();
            predict(&vals)
        })
        .sum()
}

pub fn part_2(input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            let vals: Vec<_> = l.split_whitespace().map(|n| n.parse().unwrap()).collect();
            predict_back(&vals)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day09::*;
    use std::fs;
    #[test]
    fn example09_part1() {
        let input = fs::read_to_string("input/example09").unwrap();
        assert_eq!(part_1(&input), 114);
    }
    #[test]
    fn day09_part1() {
        let input = fs::read_to_string("input/day09").unwrap();
        assert_eq!(part_1(&input), 1702218515);
    }
    #[test]
    fn example09_part2() {
        let input = fs::read_to_string("input/example09").unwrap();
        assert_eq!(part_2(&input), 2);
    }
    #[test]
    fn day09_part2() {
        let input = fs::read_to_string("input/day09").unwrap();
        assert_eq!(part_2(&input), 925);
    }
}
