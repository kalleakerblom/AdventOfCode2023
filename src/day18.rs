use std::collections::HashSet;

type Pos = (i64, i64);

fn build_moat(input: &str) -> Vec<Pos> {
    let mut pos = (0, 0);
    let mut res = vec![];
    res.push(pos);
    for dig in input.lines() {
        //R 6 (#70c710)
        let (dir, rest) = dig.split_once(' ').unwrap();
        let (distance, _color) = rest.split_once(' ').unwrap();
        let step = match dir {
            "U" => |p: Pos| (p.0, p.1 - 1),
            "D" => |p: Pos| (p.0, p.1 + 1),
            "R" => |p: Pos| (p.0 + 1, p.1),
            "L" => |p: Pos| (p.0 - 1, p.1),
            _ => panic!(),
        };
        let distance: usize = distance.parse().unwrap();
        for _ in 0..distance {
            pos = step(pos);
            res.push(pos);
        }
    }
    res
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

fn count_fill(moat: &[Pos], min_x: i64, min_y: i64, max_x: i64, max_y: i64) -> usize {
    (min_x..=max_x)
        .flat_map(|x| (min_y..=max_y).map(move |y| (x, y)))
        .filter(|p| is_inside(moat, *p))
        .count()
}

pub fn part_1(input: &str) -> usize {
    let moat = build_moat(input);
    let min_x = moat.iter().map(|p| p.0).min().unwrap();
    let max_x = moat.iter().map(|p| p.0).max().unwrap();
    let min_y = moat.iter().map(|p| p.1).min().unwrap();
    let max_y = moat.iter().map(|p| p.1).max().unwrap();
    count_fill(&moat, min_x, min_y, max_x, max_y) + moat.len() - 1
}

// i64::from_str_radix("1f", 16);

fn build_moat_part2(input: &str) -> (Vec<Pos>, HashSet<Pos>) {
    let mut pos = (0, 0);
    let mut res = vec![];
    let mut set = HashSet::new();
    res.push(pos);
    set.insert(pos);
    for dig in input.lines() {
        let (dir, rest) = dig.split_once(' ').unwrap();
        let (_, hex) = rest.split_once(' ').unwrap();
        let step = match dir {
            "U" => |p: Pos, dist: i64| (p.0, p.1 - dist),
            "D" => |p: Pos, dist: i64| (p.0, p.1 + dist),
            "R" => |p: Pos, dist: i64| (p.0 + dist, p.1),
            "L" => |p: Pos, dist: i64| (p.0 - dist, p.1),
            _ => panic!(),
        };
        let hex = hex.strip_prefix("(#").unwrap().strip_suffix(')').unwrap();
        let distance: i64 = i64::from_str_radix(hex, 16).unwrap();
        pos = step(pos, distance);
        res.push(pos);
        set.insert(pos);
    }
    (res, set)
}

fn find_inside(moat: &[Pos], min_x: i64, min_y: i64, max_x: i64, max_y: i64) -> Pos {
    (min_x..=max_x)
        .flat_map(|x| (min_y..=max_y).map(move |y| (x, y)))
        .find(|p| is_inside(moat, *p))
        .unwrap()
}

pub fn part_2(input: &str) -> usize {
    let (moat, moat_set) = build_moat_part2(input);
    let min_x = moat.iter().map(|p| p.0).min().unwrap();
    let max_x = moat.iter().map(|p| p.0).max().unwrap();
    let min_y = moat.iter().map(|p| p.1).min().unwrap();
    let max_y = moat.iter().map(|p| p.1).max().unwrap();
    let inside = find_inside(&moat, min_x, min_y, max_x, max_y);
    
    0
}

#[cfg(test)]
mod tests {
    use crate::day18::*;
    use std::fs;
    #[test]
    fn example18_part1() {
        let input = fs::read_to_string("input/example18").unwrap();
        assert_eq!(part_1(&input), 62);
    }
    #[test]
    fn day18_part1() {
        let input = fs::read_to_string("input/day18").unwrap();
        assert_eq!(part_1(&input), 92758);
    }
    #[test]
    fn example18_part2() {
        let input = fs::read_to_string("input/example18").unwrap();
        assert_eq!(part_2(&input), 952408144115);
    }
    #[test]
    fn day18_part2() {
        let input = fs::read_to_string("input/day18").unwrap();
        assert_eq!(part_2(&input), 0);
    }
}
