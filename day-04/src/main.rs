use std::collections::HashSet;
use std::iter::repeat;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let cards = read_cards(INPUT);
    let values = cards.iter().map(|card| card.points());
    let value: usize = values.sum();
    println!("Part 1: {}", value);

    let sum = copies(cards);
    println!("Part 2: {}", sum);
}

struct Card {
    winning_numbers: HashSet<usize>,
    scratch_numbers: HashSet<usize>,
}

impl Card {
    fn count(&self) -> usize {
        self.winning_numbers
            .intersection(&self.scratch_numbers)
            .count()
    }

    fn points(&self) -> usize {
        let count = self.count();
        if count == 0 {
            0
        } else {
            2_usize.pow(count as u32 - 1)
        }
    }
}

fn read_cards(input: &str) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();
    for line in input.lines() {
        // Remove card id
        let (_, line) = line.split_once(':').unwrap();
        let (winning, scratch) = line.split_once('|').unwrap();
        let winning_numbers = winning
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let scratch_numbers = scratch
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let card = Card {
            winning_numbers,
            scratch_numbers,
        };
        cards.push(card)
    }
    cards
}

fn copies(cards: Vec<Card>) -> usize {
    // Start with one copy of each card
    let mut copies: Vec<usize> = repeat(1).take(cards.len()).collect();
    // Iterate through cards and count the copies
    for index in 0..copies.len() {
        // N next cards will get copied
        let copy_n = cards[index].count();
        // Each will be copied M times
        let copy_m = copies[index];
        for copy in 1..=copy_n {
            copies[index + copy] += copy_m;
        }
    }
    // Count total number of cards
    let sum: usize = copies.iter().sum();
    sum
}

#[cfg(test)]
mod tests {

    use crate::*;

    const INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;

    #[test]
    fn part1() {
        let cards = read_cards(INPUT);
        let values = cards.iter().map(|card| card.points());
        let value: usize = values.sum();
        println!("Part 1: {}", value);
        assert_eq!(value, 13);
    }

    #[test]
    fn part2() {
        let cards = read_cards(INPUT);
        let sum = copies(cards);
        println!("Part 2: {}", sum);
        assert_eq!(sum, 30)
    }
}
