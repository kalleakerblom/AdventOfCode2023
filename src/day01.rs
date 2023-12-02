fn first_and_last_digit(line: &str) -> (char, char) {
    let digits = line.chars().filter(|c| c.is_ascii_digit());
    let first = digits.clone().next().unwrap();
    let last = digits.last().unwrap();
    println!("{first}:{last}");
    (first, last)
}

pub fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(first_and_last_digit)
        .map(|(f, l)| {
            let number: String = [f, l].iter().collect();
            number.parse::<u32>().unwrap()
        })
        .sum()
}

const DIGITS: [(&str, &str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

fn first_spelled_out(line: &str) -> Option<(usize, &str)> {
    DIGITS
        .iter()
        .map(|(needle, ch)| (line.find(needle), ch))
        .filter_map(|(pos, ch)| pos.map(|pos| (pos, *ch)))
        .min_by_key(|(pos, _)| *pos)
}

fn last_spelled_out(line: &str) -> Option<(usize, &str)> {
    DIGITS
        .iter()
        .map(|(needle, ch)| (line.rfind(needle), ch))
        .filter_map(|(pos, ch)| pos.map(|pos| (pos, *ch)))
        .max_by_key(|(pos, _)| *pos)
}

fn first_and_last_digit_part2(line: &str) -> (&str, &str) {
    let first_spelled_out = first_spelled_out(line);
    let first_digit = line.match_indices(char::is_numeric).next();
    let first = [first_spelled_out, first_digit]
        .iter()
        .filter_map(|candidate| *candidate)
        .min_by_key(|(pos, _)| *pos)
        .unwrap()
        .1;
    let last_spelled_out = last_spelled_out(line);
    let last_digit = line.rmatch_indices(char::is_numeric).next();
    let last = [last_spelled_out, last_digit]
        .iter()
        .filter_map(|candidate| *candidate)
        .max_by_key(|(pos, _)| *pos)
        .unwrap()
        .1;
    (first, last)
}

pub fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(first_and_last_digit_part2)
        .map(|(f, l)| {
            let number: String = [f, l].into_iter().collect();
            number.parse::<u32>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day01::*;
    use std::fs;
    #[test]
    fn example01_part1() {
        let input = fs::read_to_string("input/example01").unwrap();
        assert_eq!(part_1(&input), 142);
    }
    #[test]
    fn day01_part1() {
        let input = fs::read_to_string("input/day01").unwrap();
        assert_eq!(part_1(&input), 54573);
    }
    #[test]
    fn example01_part2() {
        let input = fs::read_to_string("input/example01_part2").unwrap();
        assert_eq!(part_2(&input), 281);
    }
    #[test]
    fn day01_part2() {
        let input = fs::read_to_string("input/day01").unwrap();
        //54086 too low
        assert_eq!(part_2(&input), 54591);
    }
}
