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
    fn from_card_counts(
        two_ofs: usize,
        three_ofs: usize,
        four_ofs: usize,
        five_ofs: usize,
    ) -> Self {
        match (two_ofs, three_ofs, four_ofs, five_ofs) {
            (0, 0, 0, 1) => HandType::FiveOfAKind,
            (0, 0, 1, 0) => HandType::FourOfAKind,
            (1, 1, 0, 0) => HandType::FullHouse,
            (0, 1, 0, 0) => HandType::ThreeOfAKind,
            (2, 0, 0, 0) => HandType::TwoPair,
            (1, 0, 0, 0) => HandType::OnePair,
            (0, 0, 0, 0) => HandType::HighCard,
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
        let five_ofs = char_count.values().filter(|count| **count == 5).count();
        let four_ofs = char_count.values().filter(|count| **count == 4).count();
        let three_ofs = char_count.values().filter(|count| **count == 3).count();
        let two_ofs = char_count.values().filter(|count| **count == 2).count();

        let hand_type = HandType::from_card_counts(two_ofs, three_ofs, four_ofs, five_ofs);
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
        let five_ofs = char_count
            .iter()
            .filter(|&(&ch, &count)| ch != 'J' && count == 5)
            .count();
        let four_ofs = char_count
            .iter()
            .filter(|&(&ch, &count)| ch != 'J' && count == 4)
            .count();
        let three_ofs = char_count
            .iter()
            .filter(|&(&ch, &count)| ch != 'J' && count == 3)
            .count();
        let two_ofs = char_count
            .iter()
            .filter(|&(&ch, &count)| ch != 'J' && count == 2)
            .count();
        let one_ofs = char_count
            .iter()
            .filter(|&(&ch, &count)| ch != 'J' && count == 1)
            .count();
        let jokers = char_count.get(&'J').cloned().unwrap_or(0);

        if jokers == 5 {
            return Self {
                hand_type: HandType::FiveOfAKind,
                cards,
            };
        }

        let mut x_ofs = [five_ofs, four_ofs, three_ofs, two_ofs, one_ofs];
        if jokers != 0 {
            let to_raise = x_ofs.iter().skip(1).position(|count| *count != 0).unwrap() + 1;
            x_ofs[to_raise] -= 1;
            x_ofs[to_raise - jokers as usize] += 1;
        }
        let hand_type = HandType::from_card_counts(x_ofs[3], x_ofs[2], x_ofs[1], x_ofs[0]);
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
