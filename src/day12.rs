fn match_pattern(periods: usize, hashes: usize, instruction: &[char]) -> bool {
    if periods + hashes > instruction.len() {
        return false;
    }
    let mut iter = instruction.iter();
    let periods_ok = iter
        .by_ref()
        .take(periods)
        .all(|&ch| ch == '.' || ch == '?');
    if !periods_ok {
        return false;
    }
    // hashes ok
    iter.take(hashes).all(|&ch| ch == '#' || ch == '?')
}

fn count_variations(
    extra_periods: usize,
    instruction: &[char],
    criteria_groupings: &[usize],
    prev_hash: bool,
) -> usize {
    if criteria_groupings.is_empty() {
        if instruction.contains(&'#') {
            return 0;
        } else {
            return 1;
        }
    }
    let mut sum = 0;
    for p in 0..=extra_periods {
        let periods = if prev_hash { p + 1 } else { p };
        if match_pattern(periods, criteria_groupings[0], instruction) {
            let pattern_len = periods + criteria_groupings[0];
            sum += count_variations(
                extra_periods - p,
                &instruction[pattern_len..],
                &criteria_groupings[1..],
                true,
            );
        }
    }
    sum
}

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (instruction, criteria) = l.split_once(' ').unwrap();
            let instruction: Vec<_> = instruction.chars().collect();
            let criteria: Vec<_> = criteria.split(',').map(|n| n.parse().unwrap()).collect();
            let min_len: usize = criteria.iter().sum::<usize>() + criteria.len() - 1;
            let extra_spaces = instruction.len() - min_len;
            count_variations(extra_spaces, &instruction, &criteria, false)
        })
        .sum()
}
pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (instruction, criteria) = l.split_once(' ').unwrap();
            let mut instruction: Vec<_> = instruction.chars().collect();
            instruction.push('?');
            instruction = instruction.repeat(5);
            instruction.pop();
            let criteria: Vec<_> = criteria.split(',').map(|n| n.parse().unwrap()).collect();
            let criteria = criteria.repeat(5);
            let min_len: usize = criteria.iter().sum::<usize>() + criteria.len() - 1;
            let extra_spaces = instruction.len() - min_len;
            count_variations(extra_spaces, &instruction, &criteria, false)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day12::*;
    use std::fs;
    #[test]
    fn example12_part1() {
        let input = fs::read_to_string("input/example12").unwrap();
        assert_eq!(part_1(&input), 21);
    }
    #[test]
    fn day12_part1() {
        let input = fs::read_to_string("input/day12").unwrap();
        assert_eq!(part_1(&input), 8193);
    }
    #[test]
    fn example12_part2() {
        let input = fs::read_to_string("input/example12").unwrap();
        assert_eq!(part_2(&input), 525152);
    }
    #[test]
    fn day12_part2() {
        let input = fs::read_to_string("input/day12").unwrap();
        assert_eq!(part_2(&input), 0);
    }
}
