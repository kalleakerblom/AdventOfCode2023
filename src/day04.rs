use std::collections::HashSet;

struct Card {
    winning_numbers: HashSet<u32>,
    lotto_numbers: Vec<u32>,
}

impl Card {
    fn from_str(line: &str) -> Self {
        // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        let line = line.split_once(": ").unwrap().1;
        let (winners, lotto) = line.split_once(" | ").unwrap();
        let winning_numbers = winners
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let lotto_numbers = lotto
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        Card {
            winning_numbers,
            lotto_numbers,
        }
    }

    fn winners(&self) -> usize {
        self.lotto_numbers
            .iter()
            .filter(|l| self.winning_numbers.contains(l))
            .count()
    }
    
    fn score(&self) -> u32 {
        let winners = self.winners();
        if winners == 0 {
            return 0;
        }
        2_u32.pow(winners as u32 - 1)
    }
}

pub fn part_1(input: &str) -> u32 {
    let cards: Vec<Card> = input.lines().map(Card::from_str).collect();
    cards.iter().map(|c| c.score()).sum()
}

pub fn part_2(input: &str) -> u32 {
    let cards: Vec<Card> = input.lines().map(Card::from_str).collect();
    let mut card_count = vec![1; cards.len()];
    for i in 0..cards.len() {
        let count = card_count[i];
        let winners = cards[i].winners();
        for to_increment in card_count.iter_mut().skip(i + 1).take(winners) {
            *to_increment += count;
        }
    }
    card_count.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::day04::*;
    use std::fs;
    #[test]
    fn example04_part1() {
        let input = fs::read_to_string("input/example04").unwrap();
        assert_eq!(part_1(&input), 13);
    }
    #[test]
    fn day04_part1() {
        let input = fs::read_to_string("input/day04").unwrap();
        assert_eq!(part_1(&input), 24706);
    }
    #[test]
    fn example04_part2() {
        let input = fs::read_to_string("input/example04").unwrap();
        assert_eq!(part_2(&input), 30);
    }
    #[test]
    fn day04_part2() {
        let input = fs::read_to_string("input/day04").unwrap();
        assert_eq!(part_2(&input), 13114317);
    }
}
