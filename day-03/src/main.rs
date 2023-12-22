fn main() {
    const INPUT: &str = include_str!("input.txt");
    let (parts, symbols) = read_parts_symbols(INPUT);
    let adjacent_parts = adjacent_parts(&parts, &symbols);
    let value: usize = part_sum(adjacent_parts);
    println!("Part 1: {}", value);

    let gears = gears(&parts, &symbols);
    let ratio_sum = ratio_sum(gears);
    println!("Part 2: {}", ratio_sum);
}

fn ratio_sum(gears: Vec<Gear>) -> usize {
    gears.iter().fold(0, |acc, gear| acc + gear.ratio())
}

fn part_sum(adjacent_parts: Vec<Part>) -> usize {
    adjacent_parts.iter().fold(0, |acc, a| acc + a.number)
}

fn adjacent_parts(parts: &[Part], symbols: &[Symbol]) -> Vec<Part> {
    let adjacent_parts: Vec<Part> = parts
        .iter()
        .filter(|part| symbols.iter().any(|symbol| adjacent(part, symbol)))
        .cloned()
        .collect();
    adjacent_parts
}

fn read_parts_symbols(input: &str) -> (Vec<Part>, Vec<Symbol>) {
    let mut parts = Vec::new();
    let mut symbols = Vec::new();

    for (row, line) in input.lines().enumerate() {
        let mut iter = line.chars().enumerate().peekable();
        while let Some((column, character)) = iter.next() {
            // Ignore periods
            if character == '.' {
                continue;
            }

            // Capture part numbers
            if character.is_ascii_digit() {
                let mut digits = vec![character];
                let mut columns = vec![column];
                while let Some((column, character)) = iter.peek() {
                    if character.is_ascii_digit() {
                        digits.push(*character);
                        columns.push(*column);
                        iter.next();
                    } else {
                        break;
                    }
                }
                let number: String = digits.iter().collect::<String>();
                let number: usize = number.parse().unwrap();
                let part = Part {
                    number,
                    row,
                    columns,
                };
                parts.push(part);
                continue;
            }

            // Capture symbols
            let symbol = Symbol {
                value: character,
                row,
                column,
            };
            symbols.push(symbol);
        }
    }

    (parts, symbols)
}

#[derive(Clone, Debug)]
struct Part {
    number: usize,
    row: usize,
    columns: Vec<usize>,
}

#[derive(Debug)]
struct Symbol {
    value: char,
    row: usize,
    column: usize,
}

fn adjacent(part: &Part, symbol: &Symbol) -> bool {
    let row_adjacent = part.row.abs_diff(symbol.row) <= 1;
    let column_adjacent = part
        .columns
        .iter()
        .any(|col| col.abs_diff(symbol.column) <= 1);

    row_adjacent && column_adjacent
}

struct Gear {
    a: usize,
    b: usize,
}

impl Gear {
    fn ratio(&self) -> usize {
        self.a * self.b
    }
}

fn gears(parts: &[Part], symbols: &[Symbol]) -> Vec<Gear> {
    let gear_symbols: Vec<&Symbol> = symbols.iter().filter(|sym| sym.value == '*').collect();
    let mut gears: Vec<Gear> = Vec::new();
    for symbol in gear_symbols {
        let adjacent_numbers: Vec<&Part> = parts.iter().filter(|p| adjacent(p, symbol)).collect();
        if adjacent_numbers.len() == 2 {
            let gear = Gear {
                a: adjacent_numbers[0].number,
                b: adjacent_numbers[1].number,
            };
            gears.push(gear);
        }
    }
    gears
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

    #[test]
    fn part1() {
        let (parts, symbols) = read_parts_symbols(INPUT);
        let adjacent_parts = adjacent_parts(&parts, &symbols);
        let value: usize = part_sum(adjacent_parts);
        assert_eq!(value, 4361);
    }

    #[test]
    fn part2() {
        let (parts, symbols) = read_parts_symbols(INPUT);
        let gears = gears(&parts, &symbols);
        let ratio_sum = gears.iter().fold(0, |acc, gear| acc + gear.ratio());
        assert_eq!(ratio_sum, 467835);
    }
}
