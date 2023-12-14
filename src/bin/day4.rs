use std::collections::{BTreeSet, HashMap};

fn main() {}

#[test]
fn part1() {
    let input = include_str!("../../inputs/day4.txt");

    let total_points = input.lines().fold(0, |acc, line| {
        let (_, card) = line.split_once(": ").unwrap();
        let (winning_numbers_str, numbers_str) = card.split_once(" | ").unwrap();

        let winning_numbers = BTreeSet::from_iter(
            winning_numbers_str
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok()),
        );

        let numbers = BTreeSet::from_iter(
            numbers_str
                .split(" ")
                .filter(|c| !c.is_empty())
                .filter_map(|s| s.parse::<i32>().ok()),
        );

        let winning_numbers = numbers.intersection(&winning_numbers).collect::<Vec<_>>();

        let points = if winning_numbers.len() > 0 {
            2_i32.pow(winning_numbers.len() as u32 - 1)
        } else {
            0
        };

        dbg!(winning_numbers);
        dbg!(numbers);
        dbg!(points);

        acc + points
    });

    println!("{}", total_points)
}

#[test]
fn part2() {
    let input = include_str!("../../inputs/day4.txt");
    //     let input = "Card   1: 58 96 35 20 93 34 10 27 37 30 | 99 70 93 11 63 41 37 29  7 28 34 10 40 96 38 35 27 30 20 21  4 51 58 39 56
    // Card   2: 64 84 57 46 53 86 90 99 59 70 | 99 59 30 83 84 70 31 57  6 29 18 82 15 88 86 53 51 64 32 47 44 46 80 39 90
    // Card   3: 55 87 51 18 86  5 66 83 92 95 | 73 68 49 57 29 14 41 42 65 10 84 34 67 44  6 48 61 13 28 38 52 19 78 64 11
    // Card   4: 52 21 59 78 18 42 46 91 31 10 | 48 83 13 68 42 72  4 10  6 36 63 81 21 94  8  3 78 53  2 47 62 77 56 97  7
    // Card   5:  8 79 31  1 26 57 90 62 93 10 | 26 70 73  6 16 15 93 57 34 56 87 31 10 45  1 22 79 77 90 47 42 58 41 62  8
    // Card   6: 90 75 24 69 81 93 39 38 96 33 |  2 78 68 31 99 35 49 66 36 84 54 27 43 80 50  3 22 74 60 98 57 83 13 82 91";

    let copies = input
        .lines()
        .enumerate()
        .fold(HashMap::new(), |mut copies, (game, line)| {
            let n = game as i32;
            match copies.get(&n) {
                Some(c) => {
                    copies.insert(n, c + 1);
                }
                None => {
                    copies.insert(n, 1);
                }
            }

            let (_, card) = line.split_once(": ").unwrap();
            let (winning_numbers_str, numbers_str) = card.split_once(" | ").unwrap();

            let winning_numbers = BTreeSet::from_iter(
                winning_numbers_str
                    .split_whitespace()
                    .filter_map(|s| s.parse::<i32>().ok()),
            );

            let numbers = BTreeSet::from_iter(
                numbers_str
                    .split(" ")
                    .filter(|c| !c.is_empty())
                    .filter_map(|s| s.parse::<i32>().ok()),
            );

            let n_copies = numbers
                .intersection(&winning_numbers)
                .collect::<Vec<_>>()
                .len();

            for i in 1..n_copies + 1 {
                let copy_number = (game + i) as i32;
                match copies.get(&copy_number) {
                    Some(c) => {
                        copies.insert(copy_number, c + copies.get(&(game as i32)).unwrap());
                    }
                    None => {
                        copies.insert(copy_number, 0 + copies.get(&(game as i32)).unwrap());
                    }
                }
            }
            dbg!(copies.clone());

            copies
        });

    //dbg!(copies.clone());

    println!("{}", copies.values().sum::<i32>())
}
