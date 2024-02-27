use std::collections::HashMap;
enum Op {
    Less(i64),
    More(i64),
}

impl Op {
    fn check(&self, val: i64) -> bool {
        match self {
            Op::Less(n) => val < *n,
            Op::More(n) => val > *n,
        }
    }
}
enum Rule {
    X(Op),
    M(Op),
    A(Op),
    S(Op),
    Else,
}

impl Rule {
    fn from_str(s: &str) -> Self {
        //"m>2090"
        let bytes = s.as_bytes();
        let n: i64 = s[2..].parse().unwrap();
        match (bytes[0], bytes[1]) {
            (b'x', b'<') => Rule::X(Op::Less(n)),
            (b'x', b'>') => Rule::X(Op::More(n)),
            (b'm', b'<') => Rule::M(Op::Less(n)),
            (b'm', b'>') => Rule::M(Op::More(n)),
            (b'a', b'<') => Rule::A(Op::Less(n)),
            (b'a', b'>') => Rule::A(Op::More(n)),
            (b's', b'<') => Rule::S(Op::Less(n)),
            (b's', b'>') => Rule::S(Op::More(n)),
            _ => panic!(),
        }
    }
}

fn parse_workflows(s: &str) -> HashMap<String, Vec<(Rule, String)>> {
    //px{a<2006:qkq,m>2090:A,rfg}
    let mut res = HashMap::new();
    for l in s.lines() {
        let (key, rules) = l.split_once('{').unwrap();
        let rules = rules
            .trim_end_matches('}')
            .split(',')
            .map(|split| {
                if let Some((op, target)) = split.split_once(':') {
                    (Rule::from_str(op), target.to_owned())
                } else {
                    (Rule::Else, split.to_owned())
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
        Self {
            x: x.parse().unwrap(),
            m: m.parse().unwrap(),
            a: a.parse().unwrap(),
            s: s.parse().unwrap(),
        }
    }
    fn rating(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
    fn check(&self, rule: &Rule) -> bool {
        match rule {
            Rule::X(op) => op.check(self.x),
            Rule::M(op) => op.check(self.m),
            Rule::A(op) => op.check(self.a),
            Rule::S(op) => op.check(self.s),
            Rule::Else => true,
        }
    }
}
fn parse_tools(s: &str) -> Vec<Tool> {
    s.lines().map(Tool::from_str).collect()
}

fn rate_tool(t: &Tool, flow: &HashMap<String, Vec<(Rule, String)>>) -> i64 {
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

fn rate_tools(tools: Vec<Tool>, workflows: HashMap<String, Vec<(Rule, String)>>) -> i64 {
    tools.iter().map(|t| rate_tool(t, &workflows)).sum()
}

pub fn part_1(input: &str) -> i64 {
    let (workflows, tools) = input.split_once("\n\n").unwrap();
    let workflows = parse_workflows(workflows);
    let tools = parse_tools(tools);
    rate_tools(tools, workflows)
}
///////////////////// Part 2
type MatchingAndRemainder = (Option<(i64, i64)>, Option<(i64, i64)>);
impl Op {
    fn split_range(&self, (min, max): (i64, i64)) -> MatchingAndRemainder {
        match self {
            Op::Less(split) if *split <= min => (None, Some((min, max))),
            Op::Less(split) if *split > max => (Some((min, max)), None),
            Op::Less(split) => (Some((min, split - 1)), Some((*split, max))),
            Op::More(split) if *split >= max => (None, Some((min, max))),
            Op::More(split) if *split < min => (Some((min, max)), None),
            Op::More(split) => (Some((split + 1, max)), Some((min, *split))),
        }
    }
}

#[derive(Clone, Debug)]
struct ToolRange {
    x: (i64, i64),
    m: (i64, i64),
    a: (i64, i64),
    s: (i64, i64),
}

impl ToolRange {
    fn apply_rule(&self, rule: &Rule) -> (Option<ToolRange>, Option<ToolRange>) {
        let make_range =
            |new_x: Option<_>, new_m: Option<_>, new_a: Option<_>, new_s: Option<_>| ToolRange {
                x: new_x.unwrap_or(self.x),
                m: new_m.unwrap_or(self.m),
                a: new_a.unwrap_or(self.a),
                s: new_s.unwrap_or(self.s),
            };
        match rule {
            Rule::X(op) => {
                let (matching, remainder) = op.split_range(self.x);
                (
                    matching.map(|xr| make_range(Some(xr), None, None, None)),
                    remainder.map(|xr| make_range(Some(xr), None, None, None)),
                )
            }
            Rule::M(op) => {
                let (matching, remainder) = op.split_range(self.m);
                (
                    matching.map(|mr| make_range(None, Some(mr), None, None)),
                    remainder.map(|mr| make_range(None, Some(mr), None, None)),
                )
            }
            Rule::A(op) => {
                let (matching, remainder) = op.split_range(self.a);
                (
                    matching.map(|ar| make_range(None, None, Some(ar), None)),
                    remainder.map(|ar| make_range(None, None, Some(ar), None)),
                )
            }
            Rule::S(op) => {
                let (matching, remainder) = op.split_range(self.s);
                (
                    matching.map(|sr| make_range(None, None, None, Some(sr))),
                    remainder.map(|sr| make_range(None, None, None, Some(sr))),
                )
            }
            Rule::Else => (Some(self.clone()), None),
        }
    }
}

fn count_valid_ranges(
    mut tr: ToolRange,
    pos: &str,
    flow: &HashMap<String, Vec<(Rule, String)>>,
) -> i64 {
    if pos == "A" {
        let size = |(s, e)| 1 + e - s;
        return size(tr.x) * size(tr.m) * size(tr.a) * size(tr.s);
    }
    if pos == "R" {
        return 0;
    }
    let mut sum = 0;
    for (rule, target) in flow.get(pos).unwrap().iter() {
        let (sub_tr, complement) = tr.apply_rule(rule);
        if let Some(sub_tr) = sub_tr {
            sum += count_valid_ranges(sub_tr, target, flow);
        }
        if let Some(complement) = complement {
            tr = complement;
        } else {
            break;
        }
    }
    sum
}

pub fn part_2(input: &str) -> i64 {
    let (workflows, _tools) = input.split_once("\n\n").unwrap();
    let workflows = parse_workflows(workflows);
    let full_range = ToolRange {
        x: (1, 4000),
        m: (1, 4000),
        a: (1, 4000),
        s: (1, 4000),
    };
    count_valid_ranges(full_range, "in", &workflows)
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
        assert_eq!(part_2(&input), 167409079868000);
    }

    #[test]
    fn day19_part2() {
        let input = fs::read_to_string("input/day19").unwrap();
        assert_eq!(part_2(&input), 132186256794011);
    }
}
