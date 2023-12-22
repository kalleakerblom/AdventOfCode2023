use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

type Range = (i32, i32);
struct Block {
    x_range: Range,
    y_range: Range,
    z_range: Range,
}

impl Block {
    fn from_str(s: &str) -> Self {
        // 1,0,1~1,2,1
        let (start, end) = s.split_once('~').unwrap();
        let (xs, ys, zs) = start
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect_tuple()
            .unwrap();
        let (xe, ye, ze) = end
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect_tuple()
            .unwrap();

        Self {
            x_range: (xs, xe),
            y_range: (ys, ye),
            z_range: (zs, ze),
        }
    }
}

fn place_blocks_n_map_supports(mut blocks: Vec<Block>) -> HashMap<usize, Vec<usize>> {
    let mut max_height: HashMap<(i32, i32), (i32, usize)> = HashMap::new();
    let mut supported_by = HashMap::new();
    blocks.sort_by_key(|b| b.z_range.0);
    for (block_id, b) in blocks.iter().enumerate() {
        let xy_range: Vec<_> = if b.x_range.0 != b.x_range.1 {
            (b.x_range.0..=b.x_range.1)
                .map(move |x| (x, b.y_range.0))
                .collect_vec()
        } else {
            (b.y_range.0..=b.y_range.1)
                .map(move |y| (b.x_range.0, y))
                .collect_vec()
        };
        let z_max = xy_range
            .iter()
            .map(|xy| max_height.get(xy).unwrap_or(&(0, 0)).0)
            .max()
            .unwrap();
        // find the supports
        let mut supports = xy_range
            .iter()
            .filter_map(|xy| {
                max_height
                    .get(xy)
                    .filter(|get| get.0 == z_max)
                    .map(|get| get.1)
            })
            .collect_vec();
        supports.sort();
        supports.dedup();
        supported_by.insert(block_id, supports);
        // fill in new max heights
        let new_height = z_max + b.z_range.1 - b.z_range.0 + 1;
        for xy in &xy_range {
            max_height.insert(*xy, (new_height, block_id));
        }
    }
    supported_by
}

pub fn part_1(input: &str) -> usize {
    let blocks = input.lines().map(Block::from_str).collect_vec();
    let supported_by = place_blocks_n_map_supports(blocks);
    let single_supporters: HashSet<usize> = supported_by
        .values()
        .filter_map(|supports| {
            if supports.len() == 1 {
                Some(supports[0])
            } else {
                None
            }
        })
        .collect();
    supported_by.len() - single_supporters.len()
}

fn calculate_falls(
    i: usize,
    supported_by: &HashMap<usize, Vec<usize>>,
    supports_blocks: &HashMap<usize, Vec<usize>>,
) -> usize {
    let mut fallen = HashSet::new();
    let mut to_fall: VecDeque<usize> = [i].into();
    while let Some(fall) = to_fall.pop_front() {
        fallen.insert(fall);
        if !supports_blocks.contains_key(&fall) {
            continue;
        }
        for affected in &supports_blocks[&fall] {
            if supported_by[affected]
                .iter()
                .all(|sup| fallen.contains(sup))
            {
                to_fall.push_back(*affected);
            }
        }
    }
    fallen.len() - 1
}

pub fn part_2(input: &str) -> usize {
    let blocks = input.lines().map(Block::from_str).collect_vec();
    let n_blocks = blocks.len();
    let supported_by = place_blocks_n_map_supports(blocks);
    let mut supports_blocks: HashMap<usize, Vec<usize>> = HashMap::new();
    for (k, v) in supported_by.iter() {
        for supporter in v {
            supports_blocks.entry(*supporter).or_default().push(*k);
        }
    }
    let mut sum = 0;
    for i in 0..n_blocks {
        sum += calculate_falls(i, &supported_by, &supports_blocks);
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::day22::*;
    use std::fs;
    #[test]
    fn example22_part1() {
        let input = fs::read_to_string("input/example22").unwrap();
        assert_eq!(part_1(&input), 5);
    }
    #[test]
    fn day22_part1() {
        let input = fs::read_to_string("input/day22").unwrap();
        assert_eq!(part_1(&input), 416);
    }
    #[test]
    fn example22_part2() {
        let input = fs::read_to_string("input/example22").unwrap();
        assert_eq!(part_2(&input), 7);
    }
    #[test]
    fn day22_part2() {
        let input = fs::read_to_string("input/day22").unwrap();
        assert_eq!(part_2(&input), 0);
    }
}
