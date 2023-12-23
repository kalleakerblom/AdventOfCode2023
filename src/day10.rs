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

fn is_inside(moat: &[Pos], o: Pos) -> bool {
    let mut winding = 0;
    let len = moat.len();
    for i in 0..len {
        let p = moat[i];
        let q = moat[(i + 1) % len];
        if p == o {
            return false;
        }
        let delta = (p.0 - o.0) * (q.1 - o.1) - (p.1 - o.1) * (q.0 - o.0);
        if p.0 <= o.0 && o.0 < q.0 && delta > 0 {
            winding += 1;
        } else if q.0 <= o.0 && o.0 < p.0 && delta < 0 {
            winding -= 1;
        }
    }
    winding != 0
}

fn count_enclosed_tiles(map: &Map, loop_tiles: &[Pos]) -> usize {
    map.keys().filter(|&&p| is_inside(loop_tiles, p)).count()
}

fn travel_the_loop_part2(start: Pos, map: &Map) -> Vec<Pos> {
    let mut visited = vec![];
    visited.push(start);
    let start_neighbors = [
        (start.0 + 1, start.1),
        (start.0 - 1, start.1),
        (start.0, start.1 + 1),
        (start.0, start.1 - 1),
    ];
    let mut pos = start_neighbors
        .iter()
        .find(|&&sn| map.contains_key(&sn) && get_neighbors(sn, map).contains(&start))
        .cloned()
        .unwrap();
    let mut prev = start;
    while pos != start {
        visited.push(pos);
        let next = get_neighbors(pos, map)
            .iter()
            .find(|&&nei| nei != prev)
            .cloned()
            .unwrap();
        prev = pos;
        pos = next;
    }
    visited
}

pub fn part_2(input: &str) -> usize {
    let (map, start): (Map, Pos) = parse_map_n_start(input);
    let loop_tiles = travel_the_loop_part2(start, &map);
    count_enclosed_tiles(&map, &loop_tiles)
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
