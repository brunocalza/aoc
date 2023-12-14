use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../inputs/day13.txt");

    let maps = input.split("\n\n").collect::<Vec<_>>();

    let mut s = 0;
    for map in maps {
        let rows = map.lines().map(|line| line.to_string()).collect::<Vec<_>>();

        let mut cols = Vec::new();
        for y in 0..rows[0].len() {
            let mut col = Vec::new();
            for line in map.lines() {
                col.push(line.as_bytes()[y] as char);
            }
            let s = col.iter().collect::<String>();
            cols.push(s);
        }

        let n_cols = reflections(cols);
        let n_rows = reflections(rows);
        println!("{} {}", n_cols, n_rows);
        s += n_cols + 100 * n_rows;
    }

    println!("{}", s)
}

fn reflections(input: Vec<String>) -> i32 {
    for i in 0..input.len() - 1 {
        let mut j = i as i32;
        let mut k = i as i32 + 1;
        let mut different_count = 0;

        while j >= 0 && k < input.len() as i32 {
            if has_one_difference(input[j as usize].as_str(), input[k as usize].as_str()) {
                different_count += 1;
                j -= 1;
                k += 1;
                continue;
            }

            if input[j as usize] != input[k as usize] {
                break;
            }

            j -= 1;
            k += 1;
        }

        if (j == -1 || k == input.len() as i32) && different_count == 1 {
            return i as i32 + 1;
        }
    }

    0
}

fn has_one_difference(s1: &str, s2: &str) -> bool {
    let mut iter = s1.chars().zip(s2.chars()).filter(|(c1, c2)| c1 != c2);

    iter.next().is_some() && iter.next().is_none()
}

#[test]
fn reflections_works() {
    let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

    let rows = input
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>();

    let n = reflections(rows);

    assert_eq!(1, n);
}
