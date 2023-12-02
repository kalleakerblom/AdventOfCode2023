pub fn part_1(input: &str) -> u32 {
    todo!()
}
pub fn part_2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::dayXX::*;
    use std::fs;
    #[test]
    fn exampleXX_part1() {
        let input = fs::read_to_string("input/exampleXX").unwrap();
        assert_eq!(part_1(&input), 0);
    }
    #[test]
    fn dayXX_part1() {
        let input = fs::read_to_string("input/dayXX").unwrap();
        assert_eq!(part_1(&input), 0);
    }
    #[test]
    fn exampleXX_part2() {
        let input = fs::read_to_string("input/exampleXX").unwrap();
        assert_eq!(part_2(&input), 0);
    }
    #[test]
    fn dayXX_part2() {
        let input = fs::read_to_string("input/dayXX").unwrap();
        assert_eq!(part_2(&input), 0);
    }
}
