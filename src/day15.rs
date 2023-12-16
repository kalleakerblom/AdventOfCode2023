use std::array;

fn HASH(s: &str) -> usize {
    s.as_bytes().iter().fold(0, |mut acc, b| {
        acc += *b as usize;
        acc *= 17;
        acc %= 256;
        acc
    })
}

struct HASHMAP {
    boxes: [Vec<(String, u32)>; 256],
}

impl HASHMAP {
    fn new() -> Self {
        Self {
            boxes: array::from_fn(|_| vec![]),
        }
    }
    fn operate(&mut self, op: &str) {
        if let Some((label, f)) = op.split_once('=') {
            let hash = HASH(label);
            let bx = self.boxes.get_mut(hash).unwrap();
            if let Some(lens) = bx.iter_mut().find(|(k, _v)| k == label) {
                lens.1 = f.parse().unwrap();
            } else {
                bx.push((label.to_owned(), f.parse().unwrap()));
            }
        } else {
            let op = op.strip_suffix('-').unwrap();
            let hash = HASH(op);
            let bx = self.boxes.get_mut(hash).unwrap();
            bx.retain(|lens| lens.0 != op);
        }
    }
    fn focusing_power(&self) -> u32 {
        self.boxes
            .iter()
            .enumerate()
            .flat_map(|(i, bx)| {
                bx.iter()
                    .enumerate()
                    .map(move |(slot, (_, f))| (i + 1) as u32 * (slot + 1) as u32 * f)
            })
            .sum()
    }
}

pub fn part_1(input: &str) -> usize {
    input.split(',').map(|s| s.trim()).map(HASH).sum()
}
pub fn part_2(input: &str) -> u32 {
    let mut hm = HASHMAP::new();
    input
        .split(',')
        .map(|s| s.trim())
        .for_each(|op| hm.operate(op));
    hm.focusing_power()
}

#[cfg(test)]
mod tests {
    use crate::day15::*;
    use std::fs;
    #[test]
    fn example15_part1() {
        let input = fs::read_to_string("input/example15").unwrap();
        assert_eq!(part_1(&input), 1320);
    }
    #[test]
    fn day15_part1() {
        let input = fs::read_to_string("input/day15").unwrap();
        assert_eq!(part_1(&input), 513172);
    }
    #[test]
    fn example15_part2() {
        let input = fs::read_to_string("input/example15").unwrap();
        assert_eq!(part_2(&input), 145);
    }
    #[test]
    fn day15_part2() {
        let input = fs::read_to_string("input/day15").unwrap();
        assert_eq!(part_2(&input), 237806);
    }
}
