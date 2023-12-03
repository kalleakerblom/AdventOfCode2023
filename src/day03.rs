struct EngineNumber {
    x_start: usize,
    x_end: usize,
    y: usize,
    n: u32,
}
impl EngineNumber {
    fn symbol_is_adjacent(&self, symbol: &Symbol) -> bool {
        let (sx, sy) = (symbol.x, symbol.y);
        let x_start = self.x_start.saturating_sub(1);
        let y_start = self.y.saturating_sub(1);
        x_start <= sx && sx <= self.x_end + 1 && y_start <= sy && sy <= self.y + 1
    }

    fn is_part_number(&self, symbols: &[Symbol]) -> bool {
        symbols.iter().any(|s| self.symbol_is_adjacent(s))
    }
}

struct Symbol {
    x: usize,
    y: usize,
    ch: char,
}

fn parse_line(
    line: &str,
    line_nr: usize,
    engine_numbers: &mut Vec<EngineNumber>,
    symbols: &mut Vec<Symbol>,
) {
    let mut digits = String::new();
    let mut digits_start = 0;
    let mut digits_end = 0;
    for (x, ch) in line.chars().enumerate() {
        if ch.is_ascii_digit() {
            if digits.is_empty() {
                digits_start = x;
                digits_end = x;
            } else {
                digits_end += 1;
            }
            digits.push(ch);
            continue;
        }

        if ch != '.' {
            symbols.push(Symbol { x, y: line_nr, ch });
        }
        if !digits.is_empty() {
            let n: u32 = digits.parse().unwrap();
            engine_numbers.push(EngineNumber {
                x_start: digits_start,
                x_end: digits_end,
                y: line_nr,
                n,
            });
            digits.clear();
        }
    }
    if !digits.is_empty() {
        let n: u32 = digits.parse().unwrap();
        engine_numbers.push(EngineNumber {
            x_start: digits_start,
            x_end: digits_end,
            y: line_nr,
            n,
        });
    }
}

fn parse(input: &str) -> (Vec<EngineNumber>, Vec<Symbol>) {
    let mut engine_numbers = Vec::new();
    let mut symbols = Vec::new();
    for (line_nr, line) in input.lines().enumerate() {
        parse_line(line, line_nr, &mut engine_numbers, &mut symbols);
    }
    (engine_numbers, symbols)
}

fn sum_part_numbers(engine_numbers: &[EngineNumber], sym_map: &[Symbol]) -> u32 {
    engine_numbers
        .iter()
        .filter(|en| en.is_part_number(sym_map))
        .map(|en| en.n)
        .sum()
}

pub fn part_1(input: &str) -> u32 {
    let (engine_numbers, symbols) = parse(input);
    sum_part_numbers(&engine_numbers, &symbols)
}

fn sum_gear_ratios(engine_numbers: &[EngineNumber], symbols: &[Symbol]) -> u32 {
    let mut sum = 0;
    for symbol in symbols.iter() {
        if symbol.ch != '*' {
            continue;
        }
        let mut adjacent_numbers = Vec::new();
        for en in engine_numbers {
            if en.symbol_is_adjacent(symbol) {
                if adjacent_numbers.len() == 2 {
                    // too many numbers
                    adjacent_numbers.clear();
                    break;
                }
                adjacent_numbers.push(en.n);
            }
        }
        if adjacent_numbers.len() == 2 {
            sum += adjacent_numbers[0] * adjacent_numbers[1];
        }
    }
    sum
}

pub fn part_2(input: &str) -> u32 {
    let mut engine_numbers = Vec::new();
    let mut symbols = Vec::new();
    for (line_nr, line) in input.lines().enumerate() {
        parse_line(line, line_nr, &mut engine_numbers, &mut symbols);
    }
    sum_gear_ratios(&engine_numbers, &symbols)
}

#[cfg(test)]
mod tests {
    use crate::day03::*;
    use std::fs;
    #[test]
    fn example03_part1() {
        let input = fs::read_to_string("input/example03").unwrap();
        assert_eq!(part_1(&input), 4361);
    }
    #[test]
    fn day03_part1() {
        let input = fs::read_to_string("input/day03").unwrap();
        assert_eq!(part_1(&input), 525911);
    }
    #[test]
    fn example03_part2() {
        let input = fs::read_to_string("input/example03").unwrap();
        assert_eq!(part_2(&input), 467835);
    }
    #[test]
    fn day03_part2() {
        let input = fs::read_to_string("input/day03").unwrap();
        assert_eq!(part_2(&input), 75805607);
    }
}
