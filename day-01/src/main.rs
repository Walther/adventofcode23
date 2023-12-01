use regex::Regex;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let lines: Vec<String> = INPUT.lines().map(|line| line.to_owned()).collect();

    // Part 1
    let value = calibration(&lines);
    println!("Part 1: {}", value);

    // Part 2
    let human_digits = parse_human_digits(&lines);
    let value = calibration(&human_digits);
    println!("Part 2: {}", value);
}

fn parse_human_digits(lines: &[String]) -> Vec<String> {
    let digit_words =
        Regex::new("^(1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let human_digits: Vec<String> = lines
        .iter()
        .map(|line| {
            let mut parsed: Vec<String> = Vec::new();
            let len = line.len();
            for i in (0..(len)).rev() {
                let suffix = &line[i..];
                if let Some(mat) = digit_words.find(suffix) {
                    let text = mat.as_str();
                    let digit = digitizer(text);
                    parsed.push(digit.to_owned());
                };
            }
            parsed.reverse();
            parsed.join("")
        })
        .collect();
    human_digits
}

fn digitizer(text: &str) -> &str {
    match text {
        "one" | "1" => "1",
        "two" | "2" => "2",
        "three" | "3" => "3",
        "four" | "4" => "4",
        "five" | "5" => "5",
        "six" | "6" => "6",
        "seven" | "7" => "7",
        "eight" | "8" => "8",
        "nine" | "9" => "9",
        _ => unreachable!(),
    }
}

fn calibration(lines: &[String]) -> u32 {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .filter(|char| char.is_ascii_digit())
                .map(|digit| digit.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|digits| digits.first().unwrap() * 10 + digits.last().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part1() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        let lines: Vec<String> = input.lines().map(|line| line.to_owned()).collect();
        let value = calibration(&lines);
        assert_eq!(value, 142);
    }

    #[test]
    fn part2() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        let lines: Vec<String> = input.lines().map(|line| line.to_owned()).collect();
        let human_digits = parse_human_digits(&lines);
        let value = calibration(&human_digits);
        assert_eq!(value, 281);
    }

    #[test]
    fn overlap() {
        let input = "eighthree";
        let lines: Vec<String> = input.lines().map(|line| line.to_owned()).collect();
        let human_digits = parse_human_digits(&lines);
        let value = calibration(&human_digits);
        assert_eq!(value, 83);
    }
}
