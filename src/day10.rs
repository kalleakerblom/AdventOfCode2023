use std::collections::{HashMap, HashSet, VecDeque};

type Pos = (i32, i32);
type Map = HashMap<Pos, char>;
fn parse_map_n_start(input: &str) -> (Map, Pos) {
    let mut start = None;
    let mut map = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let pos = (col as i32, row as i32);
            if ch == 'S' {
                start = Some(pos);
            }
            map.insert(pos, ch);
        }
    }
    (map, start.unwrap())
}

fn get_neighbors(pos: Pos, map: &Map) -> Vec<Pos> {
    let (x, y) = pos;
    match map[&pos] {
        '.' => vec![],
        '|' => [(x, y + 1), (x, y - 1)].into_iter().collect(),
        '-' => [(x - 1, y), (x + 1, y)].into_iter().collect(),
        'L' => [(x + 1, y), (x, y - 1)].into_iter().collect(),
        'J' => [(x - 1, y), (x, y - 1)].into_iter().collect(),
        '7' => [(x - 1, y), (x, y + 1)].into_iter().collect(),
        'F' => [(x + 1, y), (x, y + 1)].into_iter().collect(),
        bad => panic!("bad:{bad}"),
    }
}

fn travel_the_loop(start: Pos, map: &Map) -> HashMap<Pos, usize> {
    let mut visited: HashMap<Pos, usize> = HashMap::new();
    visited.insert(start, 0);
    let mut to_visit = VecDeque::new();
    let start_neighbors = [
        (start.0 + 1, start.1),
        (start.0 - 1, start.1),
        (start.0, start.1 + 1),
        (start.0, start.1 - 1),
    ];
    for sn in start_neighbors {
        if map.contains_key(&sn) && get_neighbors(sn, map).contains(&start) {
            to_visit.push_back((sn, 1));
        }
    }
    while let Some((next, steps)) = to_visit.pop_front() {
        visited.insert(next, steps);
        to_visit.extend(
            get_neighbors(next, map)
                .iter()
                .filter(|nei| !visited.contains_key(nei))
                .map(|nei| (*nei, steps + 1)),
        );
    }
    visited
}

pub fn part_1(input: &str) -> usize {
    let (map, start): (Map, Pos) = parse_map_n_start(input);
    *travel_the_loop(start, &map).values().max().unwrap()
}

fn count_unenclosed_tiles(map: &Map, loop_tiles: &HashMap<Pos, usize>) -> usize {
    let mut count = 0;
    for tile in map.keys() {
        if loop_tiles.contains_key(tile) {
            continue;
        }
        let mut intersections = 0;
        for x in -1..tile.0 {
            let ray_pos = (x, tile.1);
            if loop_tiles.contains_key(&ray_pos) && matches!(map[&ray_pos], '|' | 'L' | 'J' | 'S') {
                intersections += 1;
            }
        }
        if intersections % 2 != 0 {
            count += 1;
        }
    }
    count
}

pub fn part_2(input: &str) -> usize {
    let (map, start): (Map, Pos) = parse_map_n_start(input);
    let loop_tiles = travel_the_loop(start, &map);
    let count = count_unenclosed_tiles(&map, &loop_tiles);
    count
}

#[cfg(test)]
mod tests {
    use crate::day10::*;
    use std::fs;
    #[test]
    fn example10_part1() {
        let input = fs::read_to_string("input/example10").unwrap();
        assert_eq!(part_1(&input), 8);
    }
    #[test]
    fn day10_part1() {
        let input = fs::read_to_string("input/day10").unwrap();
        assert_eq!(part_1(&input), 6846);
    }
    #[test]
    fn example10_part2() {
        let input = fs::read_to_string("input/example10_part2").unwrap();
        assert_eq!(part_2(&input), 4);
    }
    #[test]
    fn example10_part2_big() {
        let input = fs::read_to_string("input/example10_part2_big").unwrap();
        assert_eq!(part_2(&input), 8);
    }
    #[test]
    fn example10_part2_big2() {
        let input = fs::read_to_string("input/example10_part2_big2").unwrap();
        assert_eq!(part_2(&input), 10);
    }
    #[test]
    fn day10_part2() {
        // 325 too low
        let input = fs::read_to_string("input/day10").unwrap();
        assert_eq!(part_2(&input), 0);
    }
}
