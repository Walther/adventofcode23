fn main() {
    const INPUT: &str = include_str!("input.txt");
    let games = parse_games(INPUT);

    // Part 1
    let value = part1(&games);
    println!("Part 1: {}", value);

    // Part 2
    let value = part2(&games);
    println!("Part 2: {}", value);
}

fn part1(games: &[Game]) -> usize {
    const CONSTRAINT: Constraint = Constraint {
        red: 12,
        green: 13,
        blue: 14,
    };
    let valid_games: Vec<&Game> = games
        .iter()
        .filter(|game| game.validate(&CONSTRAINT))
        .collect();
    let value: usize = valid_games.iter().map(|game| game.id).sum();
    value
}

fn part2(games: &[Game]) -> usize {
    let minimums: Vec<Constraint> = games.iter().map(|game| game.minimum()).collect();
    let value: usize = minimums.iter().map(|constraint| constraint.power()).sum();
    value
}

fn parse_games(input: &str) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();
    for line in input.lines() {
        let (game, reveals) = line.split_once(": ").unwrap();
        // "Game ".len() == 5
        let id = &game[5..];
        let id: usize = id.parse().unwrap();
        let mut rounds = Vec::new();
        let reveals = reveals.split("; ");
        for reveal in reveals {
            let (mut red, mut green, mut blue) = (0, 0, 0);
            let colors = reveal.split(", ");
            for color in colors {
                let (num, col) = color.split_once(' ').unwrap();
                let num: usize = num.parse().unwrap();
                match col {
                    "red" => red += num,
                    "green" => green += num,
                    "blue" => blue += num,
                    _ => unreachable!(),
                };
            }
            let reveal = Reveal { red, green, blue };
            rounds.push(reveal);
        }
        games.push(Game { id, rounds });
    }
    games
}

#[derive(Debug)]
pub struct Game {
    pub id: usize,
    pub rounds: Vec<Reveal>,
}

impl Game {
    pub fn validate(&self, constraint: &Constraint) -> bool {
        self.rounds.iter().all(|reveal| reveal.validate(constraint))
    }

    pub fn minimum(&self) -> Constraint {
        Constraint {
            red: self.rounds.iter().max_by_key(|game| game.red).unwrap().red,
            green: self
                .rounds
                .iter()
                .max_by_key(|game| game.green)
                .unwrap()
                .green,
            blue: self
                .rounds
                .iter()
                .max_by_key(|game| game.blue)
                .unwrap()
                .blue,
        }
    }
}

#[derive(Debug)]
pub struct Reveal {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl Reveal {
    pub fn validate(&self, constraint: &Constraint) -> bool {
        self.red <= constraint.red && self.green <= constraint.green && self.blue <= constraint.blue
    }

    pub fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

type Constraint = Reveal;

#[cfg(test)]
mod tests {
    use crate::*;
    const INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

    #[test]
    fn test_part1() {
        let games = parse_games(INPUT);
        let value = part1(&games);
        assert_eq!(value, 8);
    }

    #[test]
    fn test_part2() {
        let games = parse_games(INPUT);
        let value = part2(&games);
        assert_eq!(value, 2286);
    }
}
