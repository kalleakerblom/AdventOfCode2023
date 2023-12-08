use std::collections::HashMap;

const CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_card_counts(counts: &[u8]) -> Self {
        match counts {
            [5] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => panic!(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Hand {
    hand_type: HandType,
    cards: Vec<u8>,
}

impl Hand {
    fn from_str(s: &str) -> Self {
        let mut char_count: HashMap<char, u8> = HashMap::new();
        let cards: Vec<u8> = s
            .chars()
            .map(|c| {
                *char_count.entry(c).or_default() += 1;
                CARDS.iter().position(|card| c == *card).unwrap() as u8
            })
            .collect();
        let mut counts: Vec<_> = char_count.values().cloned().collect();
        counts.sort();
        let hand_type = HandType::from_card_counts(&counts);
        Self { hand_type, cards }
    }
}

pub fn part_1(input: &str) -> u32 {
    let mut hands_n_bids: Vec<(Hand, u32)> = input
        .lines()
        .map(|l| {
            l.split_once(' ')
                .map(|(h, b)| (Hand::from_str(h), b.parse().unwrap()))
                .unwrap()
        })
        .collect();
    hands_n_bids.sort();
    hands_n_bids
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as u32 * bid)
        .sum()
}

const CARDS_PART2: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

impl Hand {
    fn from_str_part2(s: &str) -> Self {
        let mut char_count: HashMap<char, u8> = HashMap::new();
        let cards: Vec<u8> = s
            .chars()
            .map(|c| {
                *char_count.entry(c).or_default() += 1;
                CARDS_PART2.iter().position(|card| c == *card).unwrap() as u8
            })
            .collect();

        let mut counts: Vec<_> = char_count
            .iter()
            .filter_map(|(&ch, &count)| (ch != 'J').then_some(count))
            .collect();
        counts.sort();

        let jokers = char_count.get(&'J').cloned().unwrap_or(0);
        if jokers == 5 {
            return Self {
                hand_type: HandType::FiveOfAKind,
                cards,
            };
        }
        *counts.last_mut().unwrap() += jokers;

        let hand_type = HandType::from_card_counts(&counts);
        Self { hand_type, cards }
    }
}

pub fn part_2(input: &str) -> u32 {
    let mut hands_n_bids: Vec<(Hand, u32)> = input
        .lines()
        .map(|l| {
            l.split_once(' ')
                .map(|(h, b)| (Hand::from_str_part2(h), b.parse().unwrap()))
                .unwrap()
        })
        .collect();
    hands_n_bids.sort();
    hands_n_bids
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as u32 * bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day07::*;
    use std::fs;
    #[test]
    fn example07_part1() {
        let input = fs::read_to_string("input/example07").unwrap();
        assert_eq!(part_1(&input), 6440);
    }
    #[test]
    fn day07_part1() {
        let input = fs::read_to_string("input/day07").unwrap();
        assert_eq!(part_1(&input), 251216224);
    }
    #[test]
    fn example07_part2() {
        let input = fs::read_to_string("input/example07").unwrap();
        assert_eq!(part_2(&input), 5905);
    }
    #[test]
    fn day07_part2() {
        let input = fs::read_to_string("input/day07").unwrap();
        assert_eq!(part_2(&input), 250825971);
    }
}
