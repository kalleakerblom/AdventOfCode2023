use std::cell::Cell;

#[derive(Clone, Copy)]
enum PredictDirection {
    Forward,
    Backward,
}
fn predict_inplace(vals: &mut [i32], predict_dir: PredictDirection) -> i32 {
    if vals.iter().all(|v| *v == 0) {
        return 0;
    }
    let first_or_last = match predict_dir {
        PredictDirection::Forward => *vals.last().unwrap(),
        PredictDirection::Backward => *vals.first().unwrap(),
    };
    // Rust doesn't like windows with mutability, need to use this workaround.
    Cell::from_mut(vals)
        .as_slice_of_cells()
        .windows(2)
        .for_each(|w| {
            let diff = w[1].get() - w[0].get();
            w[0].set(diff);
        });
    let len = vals.len();
    let predicted_diff = predict_inplace(&mut vals[..len - 1], predict_dir);

    match predict_dir {
        PredictDirection::Forward => first_or_last + predicted_diff,
        PredictDirection::Backward => first_or_last - predicted_diff,
    }
}

pub fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            let mut vals: Vec<_> = l.split_whitespace().map(|n| n.parse().unwrap()).collect();
            predict_inplace(&mut vals, PredictDirection::Forward)
        })
        .sum()
}

pub fn part_2(input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            let mut vals: Vec<_> = l.split_whitespace().map(|n| n.parse().unwrap()).collect();
            predict_inplace(&mut vals, PredictDirection::Backward)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day09::*;
    use std::fs;
    #[test]
    fn example09_part1_inplace() {
        let input = fs::read_to_string("input/example09").unwrap();
        assert_eq!(part_1(&input), 114);
    }
    #[test]
    fn day09_part1_inplace() {
        let input = fs::read_to_string("input/day09").unwrap();
        assert_eq!(part_1(&input), 1702218515);
    }
    #[test]
    fn example09_part2_inplace() {
        let input = fs::read_to_string("input/example09").unwrap();
        assert_eq!(part_2(&input), 2);
    }
    #[test]
    fn day09_part2_inplace() {
        let input = fs::read_to_string("input/day09").unwrap();
        assert_eq!(part_2(&input), 925);
    }
}
