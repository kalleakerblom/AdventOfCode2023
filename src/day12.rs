fn check_record(count: usize, instruction: &[char], criteria_groupings: &[usize]) -> u64 {
    if criteria_groupings.is_empty() {
        if instruction.contains(&'#') {
            return 0;
        } else {
            return 1;
        }
    }

    if instruction.is_empty() {
        if criteria_groupings.len() == 1 && count == criteria_groupings[0] {
            return 1;
        } else {
            return 0;
        }
    }

    match instruction[0] {
        '#' => {
            if count + 1 > criteria_groupings[0] {
                return 0;
            }
            check_record(count + 1, &instruction[1..], criteria_groupings)
        }
        '.' => {
            if count == 0 {
                check_record(0, &instruction[1..], criteria_groupings)
            } else if count == criteria_groupings[0] {
                check_record(0, &instruction[1..], &criteria_groupings[1..])
            } else {
                0
            }
        }
        '?' => {
            let mut res = 0;
            if count < criteria_groupings[0] {
                // treat as #
                res += check_record(count + 1, &instruction[1..], criteria_groupings)
            }
            if count == criteria_groupings[0] {
                // treat as .
                res += check_record(0, &instruction[1..], &criteria_groupings[1..]);
            }
            if count == 0 {
                // treat as .
                res += check_record(0, &instruction[1..], criteria_groupings);
            }
            res
        }
        _ => panic!(),
    }
}

pub fn part_1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let (instruction, criteria) = l.split_once(' ').unwrap();
            let instruction: Vec<_> = instruction.chars().collect();
            let criteria: Vec<_> = criteria.split(',').map(|n| n.parse().unwrap()).collect();
            check_record(0, &instruction, &criteria)
        })
        .sum()
}
pub fn part_2(input: &str) -> u64 {
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
            dbg!(check_record(0, &instruction, &criteria))
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
