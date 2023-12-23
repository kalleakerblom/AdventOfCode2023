use std::collections::HashMap;

enum Op {
    X(fn(i64, i64) -> bool, i64),
    M(fn(i64, i64) -> bool, i64),
    A(fn(i64, i64) -> bool, i64),
    S(fn(i64, i64) -> bool, i64),
    Else,
}

impl Op {
    fn from_str(s: &str) -> Self {
        //"m>2090"
        let bytes = s.as_bytes();
        let n: i64 = s[2..].parse().unwrap();
        let less_than = |n_: i64, m: i64| n_ < m;
        let more_than = |n_: i64, m: i64| n_ > m;
        match (bytes[0], bytes[1]) {
            (b'x', b'<') => Op::X(less_than, n),
            (b'x', b'>') => Op::X(more_than, n),
            (b'm', b'<') => Op::M(less_than, n),
            (b'm', b'>') => Op::M(more_than, n),
            (b'a', b'<') => Op::A(less_than, n),
            (b'a', b'>') => Op::A(more_than, n),
            (b's', b'<') => Op::S(less_than, n),
            (b's', b'>') => Op::S(more_than, n),
            _ => panic!(),
        }
    }
}

fn parse_workflows(s: &str) -> HashMap<String, Vec<(Op, String)>> {
    //px{a<2006:qkq,m>2090:A,rfg}
    let mut res = HashMap::new();
    for l in s.lines() {
        let (key, rules) = l.split_once("{").unwrap();
        let rules = rules
            .trim_end_matches('}')
            .split(',')
            .map(|split| {
                if let Some((op, target)) = split.split_once(':') {
                    (Op::from_str(op), target.to_owned())
                } else {
                    (Op::Else, split.to_owned())
                }
            })
            .collect();
        res.insert(key.into(), rules);
    }
    res
}

struct Tool {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Tool {
    fn from_str(s: &str) -> Self {
        //{x=787,m=2655,a=1222,s=2876}
        let (x, rest) = s.trim_start_matches("{x=").split_once(',').unwrap();
        let (m, rest) = rest.trim_start_matches("m=").split_once(',').unwrap();
        let (a, rest) = rest.trim_start_matches("a=").split_once(',').unwrap();
        let s = rest.trim_start_matches("s=").trim_end_matches('}');
        let x = x.parse().unwrap();
        let m = m.parse().unwrap();
        let a = a.parse().unwrap();
        let s = s.parse().unwrap();
        Self { x, m, a, s }
    }
    fn rating(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
    fn check(&self, op: &Op) -> bool {
        match op {
            Op::X(f, n) => f(self.x, *n),
            Op::M(f, n) => f(self.m, *n),
            Op::A(f, n) => f(self.a, *n),
            Op::S(f, n) => f(self.s, *n),
            Op::Else => true,
        }
    }
}
fn parse_tools(s: &str) -> Vec<Tool> {
    s.lines().map(Tool::from_str).collect()
}

fn rate_tool(t: &Tool, flow: &HashMap<String, Vec<(Op, String)>>) -> i64 {
    let mut pos = "in";
    loop {
        if pos == "A" {
            return t.rating();
        }
        if pos == "R" {
            return 0;
        }
        let rules = flow.get(pos).unwrap();
        let target = &rules.iter().find(|r| t.check(&r.0)).unwrap().1;
        pos = target;
    }
}

fn rate_tools(tools: Vec<Tool>, workflows: HashMap<String, Vec<(Op, String)>>) -> i64 {
    tools.iter().map(|t| rate_tool(t, &workflows)).sum()
}

pub fn part_1(input: &str) -> i64 {
    let (workflows, tools) = input.split_once("\n\n").unwrap();
    let workflows = parse_workflows(workflows);
    let tools = parse_tools(tools);
    rate_tools(tools, workflows)
}

pub fn part_2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::day19::*;
    use std::fs;
    #[test]
    fn example19_part1() {
        let input = fs::read_to_string("input/example19").unwrap();
        assert_eq!(part_1(&input), 19114);
    }
    #[test]
    fn day19_part1() {
        let input = fs::read_to_string("input/day19").unwrap();
        assert_eq!(part_1(&input), 495298);
    }
    #[test]
    fn example19_part2() {
        let input = fs::read_to_string("input/example19").unwrap();
        assert_eq!(part_2(&input), 0);
    }
    #[test]
    fn day19_part2() {
        let input = fs::read_to_string("input/day19").unwrap();
        assert_eq!(part_2(&input), 0);
    }
}
