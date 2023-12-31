use std::{
    collections::{HashMap, HashSet, VecDeque},
    vec,
};

#[derive(Debug, Clone, Copy)]
enum Pulse {
    Low,
    High,
}
type Id = usize;
#[derive(Debug)]
enum Type {
    FlipFlop(Pulse),
    Conj(Vec<(Id, Pulse)>),
    Broadcaster,
}

#[derive(Debug)]
struct Module {
    type_: Type,
    destinations: Vec<Id>,
}

impl Module {
    fn react(&mut self, source: Id, pulse: Pulse) -> Option<Pulse> {
        let out = match (&mut self.type_, pulse) {
            (Type::FlipFlop(mem), Pulse::Low) => {
                *mem = match mem {
                    Pulse::Low => Pulse::High,
                    Pulse::High => Pulse::Low,
                };
                *mem
            }
            (Type::FlipFlop(_), Pulse::High) => {
                return None;
            }
            (Type::Conj(mem), _) => {
                let m = mem.iter_mut().find(|m| m.0 == source).unwrap();
                m.1 = pulse;
                if mem.iter().all(|(_, p)| matches!(p, Pulse::High)) {
                    Pulse::Low
                } else {
                    Pulse::High
                }
            }
            (Type::Broadcaster, _) => Pulse::Low,
        };
        Some(out)
    }
}

fn parse_modules(s: &str) -> Vec<Module> {
    let mut modules = vec![];
    let id_map: HashMap<&str, usize> = s
        .lines()
        .map(|l| l.split_once(" ->").unwrap().0)
        .filter(|name| {
            if *name == "broadcaster" {
                modules.push(Module {
                    type_: Type::Broadcaster,
                    destinations: vec![],
                });
                false
            } else {
                true
            }
        })
        .enumerate()
        .map(|(i, name)| (&name[1..], i + 1))
        .collect();
    for line in s.lines() {
        let (module, destinations) = line.split_once(" -> ").unwrap();
        let destinations = destinations
            .split(", ")
            .map(|name| *id_map.get(name).unwrap_or(&0))
            .inspect(|id| {
                if *id == 0 {
                    println!("untyped!");
                }
            })
            .collect();
        if module == "broadcaster" {
            modules[0].destinations = destinations;
            continue;
        }
        let m = match module.as_bytes()[0] {
            b'%' => Module {
                type_: Type::FlipFlop(Pulse::Low),
                destinations,
            },
            b'&' => Module {
                type_: Type::Conj(vec![]),
                destinations,
            },
            _ => panic!(),
        };
        modules.push(m);
    }
    for id in 0..modules.len() {
        for d in modules[id].destinations.clone() {
            if let Type::Conj(conj) = &mut modules[d].type_ {
                conj.push((id, Pulse::Low));
            }
        }
    }
    modules
}

fn button_press(modules: &mut [Module]) -> (usize, usize) {
    let mut low_cnt = 1;
    let mut high_cnt = 0;
    let mut events: VecDeque<_> = [(0, 0, Pulse::Low)].into();
    while let Some((source, dest, pulse)) = events.pop_front() {
        let module = &mut modules[dest];
        if let Some(out) = module.react(source, pulse) {
            match out {
                Pulse::Low => low_cnt += module.destinations.len(),
                Pulse::High => high_cnt += module.destinations.len(),
            }
            events.extend(
                module
                    .destinations
                    .iter()
                    .filter(|d| **d != 0)
                    .map(|d| (dest, *d, out)),
            );
        }
    }
    (low_cnt, high_cnt)
}

pub fn part_1(input: &str) -> usize {
    let mut modules = parse_modules(input);
    let mut low = 0;
    let mut high = 0;
    for _ in 0..1000 {
        let (l, h) = button_press(&mut modules);
        low += l;
        high += h;
    }
    low * high
}

fn button_press_part2(modules: &mut [Module], press: usize) -> bool {
    let mut events: VecDeque<_> = [(0, 0, Pulse::Low)].into();
    while let Some((source, dest, pulse)) = events.pop_front() {
        let module = &mut modules[dest];
        if let Some(out) = module.react(source, pulse) {
            events.extend(
                module
                    .destinations
                    .iter()
                    .filter(|d| **d != 0)
                    .map(|d| (dest, *d, out)),
            );
        }
        let Type::Conj(mem) = &modules[4].type_ else{
            unreachable!()
        };
        if matches!(mem[0].1, Pulse::High) {
            println!("1:{press}");
        }

        if matches!(mem[1].1, Pulse::High) {
            println!("2:{press}");
        }

        if matches!(mem[2].1, Pulse::High) {
            println!("3:{press}");
        }

        if matches!(mem[3].1, Pulse::High) {
            println!("4:{press}");
        }
    }

    false
}

pub fn part_2(input: &str) -> usize {
    let mut modules = parse_modules(input);
    for i in 1..5_000 {
        if button_press_part2(&mut modules, i) {
            return i;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::day20::*;
    use std::fs;
    #[test]
    fn example20_part1() {
        let input = fs::read_to_string("input/example20").unwrap();
        assert_eq!(part_1(&input), 32000000);
    }
    #[test]
    fn day20_part1() {
        let input = fs::read_to_string("input/day20").unwrap();
        assert_eq!(part_1(&input), 743871576);
    }
    #[test]
    fn example20_part2() {
        let input = fs::read_to_string("input/example20").unwrap();
        assert_eq!(part_2(&input), 0);
    }
    #[test]
    fn day20_part2() {
        let input = fs::read_to_string("input/day20").unwrap();
        assert_eq!(part_2(&input), 0);

        // periods
        // 3797
        // 3907
        // 4021
        // 4093
        use num::integer::lcm;
        let ans = [3797usize, 3907, 4021, 4093]
            .into_iter()
            .reduce(lcm)
            .unwrap();
        assert_eq!(ans, 244151741342687);
    }
}
