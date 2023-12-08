use std::collections::HashMap;

type Map<'a> = HashMap<&'a str, (&'a str, &'a str)>;
fn parse_map<'a>(s: &'a str) -> Map<'a> {
    let parse_line = |l: &'a str| {
        // AAA = (BBB, BBB)
        let (start, left_right) = l.split_once(" = ").unwrap();
        let (left, right) = left_right
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(", ")
            .unwrap();
        (start, (left, right))
    };
    s.lines().map(parse_line).collect()
}

fn calc_steps(
    instructions: Vec<char>,
    map: HashMap<&str, (&str, &str)>,
    start: &str,
    goal: &str,
) -> u32 {
    let mut pos = start;
    for (steps, ins) in instructions.iter().cycle().enumerate() {
        if pos == goal {
            return steps as u32;
        }
        let (l, r) = map.get(pos).unwrap();
        pos = if *ins == 'L' { l } else { r };
    }
    unreachable!()
}

pub fn part_1(input: &str) -> u32 {
    let (instructions, map): (Vec<char>, Map) = input
        .split_once("\n\n")
        .map(|(i, m)| (i.chars().collect(), parse_map(m)))
        .unwrap();
    calc_steps(instructions, map, "AAA", "ZZZ")
}

fn calc_steps_part2(
    instructions: Vec<char>,
    map: HashMap<&str, (&str, &str)>,
    starts: &[&str],
) -> usize {
    let mut positions: Vec<&str> = starts.to_owned();
    for (steps, ins) in instructions.iter().cycle().enumerate() {
        if positions.iter().all(|p| p.ends_with('Z')) {
            return steps;
        }
        for pos in positions.iter_mut() {
            let (l, r) = map.get(pos).unwrap();
            *pos = if *ins == 'L' { l } else { r };
        }
    }
    unreachable!()
}

fn calc_steps_part2_fast(
    instructions: Vec<char>,
    map: HashMap<&str, (&str, &str)>,
    starts: &[&str],
) -> usize {
    // the ghosts travel in circles, need to find each period and then calculate the shared LCM.
    let mut periods: Vec<_> = vec![];
    for start in starts {
        let mut pos = start;
        for (steps, ins) in instructions.iter().cycle().enumerate() {
            if pos.ends_with('Z') {
                periods.push(steps);
                break;
            }
            let (l, r) = map.get(pos).unwrap();
            pos = if *ins == 'L' { l } else { r };
        }
    }
    periods.iter().cloned().reduce(num::integer::lcm).unwrap()
}

pub fn part_2(input: &str) -> usize {
    let (instructions, map): (Vec<char>, Map) = input
        .split_once("\n\n")
        .map(|(i, m)| (i.chars().collect(), parse_map(m)))
        .unwrap();
    let starts: Vec<&str> = map.keys().filter(|k| k.ends_with('A')).cloned().collect();
    calc_steps_part2(instructions, map, &starts)
}

pub fn part_2_fast(input: &str) -> usize {
    let (instructions, map): (Vec<char>, Map) = input
        .split_once("\n\n")
        .map(|(i, m)| (i.chars().collect(), parse_map(m)))
        .unwrap();
    let starts: Vec<&str> = map.keys().filter(|k| k.ends_with('A')).cloned().collect();
    calc_steps_part2_fast(instructions, map, &starts)
}

#[cfg(test)]
mod tests {
    use crate::day08::*;
    use std::fs;
    #[test]
    fn example08_part1() {
        let input = fs::read_to_string("input/example08").unwrap();
        assert_eq!(part_1(&input), 2);
    }
    #[test]
    fn example08_2_part1() {
        let input = fs::read_to_string("input/example08_2").unwrap();
        assert_eq!(part_1(&input), 6);
    }
    #[test]
    fn day08_part1() {
        let input = fs::read_to_string("input/day08").unwrap();
        assert_eq!(part_1(&input), 13939);
    }
    #[test]
    fn example08_part2() {
        let input = fs::read_to_string("input/example08_part2").unwrap();
        assert_eq!(part_2(&input), 6);
    }
    #[test]
    fn day08_part2() {
        let input = fs::read_to_string("input/day08").unwrap();
        assert_eq!(part_2_fast(&input), 8906539031197);
    }
}
