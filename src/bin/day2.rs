use std::collections::HashMap;

fn main() {
    let file = include_str!("../../inputs/day2.txt");

    let constraints: HashMap<&str, i32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    // part 1
    let part1_result: i32 = file
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(game, sets)| {
            (
                game.chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap(),
                sets.split("; ").collect::<Vec<&str>>(),
            )
        })
        .filter(|(_, sets)| {
            for set in sets {
                for subset in set.split(", ").collect::<Vec<&str>>() {
                    let (n, color) = subset.split_once(' ').unwrap();
                    if n.parse::<i32>().unwrap() > *constraints.get(color).unwrap() {
                        return false;
                    }
                }
            }

            true
        })
        .map(|(game, _)| game)
        .sum();

    println!("{}", part1_result);

    // part 2
    let part2_result: i32 = file
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(game, sets)| {
            (
                game.chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap(),
                sets.split("; ").collect::<Vec<&str>>(),
            )
        })
        .map(|(_, sets)| {
            let mut minimum_required = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

            for set in sets {
                for subset in set.split(", ").collect::<Vec<&str>>() {
                    let (n, color) = subset.split_once(' ').unwrap();
                    let n = n.parse::<i32>().unwrap();

                    if n > *minimum_required.get(color).unwrap() {
                        minimum_required.insert(color, n);
                    }
                }
            }

            minimum_required.into_values().product::<i32>()
        })
        .sum();

    println!("{}", part2_result);
}
