use std::collections::btree_map::Range;

use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let input = include_str!("../../inputs/day5.txt");

    let input = input.split("\n\n").collect::<Vec<_>>();

    let (&[seeds], maps) = input.split_at(1) else {
        unreachable!()
    };
    let (_, seeds) = seeds.split_once(": ").unwrap();
    let seeds = seeds
        .split(" ")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| {
            let (seed_range_start, seed_range_end) = (chunk[0], chunk[0] + chunk[1]);
            seed_range_start..seed_range_end
        })
        .collect::<Vec<_>>();

    let maps = maps.iter().map(|map_str| {
        let (_, values) = map_str.split_once(":\n").unwrap();
        let ranges: Vec<&str> = values.split("\n").collect::<Vec<&str>>();

        let ranges = ranges
            .iter()
            .map(|range| {
                let [destination_str, source_str, length_str] =
                    range.splitn(3, " ").collect::<Vec<&str>>()[..]
                else {
                    unreachable!()
                };
                let destination = destination_str.parse::<i64>().unwrap();
                let source = source_str.parse::<i64>().unwrap();
                let length = length_str.parse::<i64>().unwrap();

                (source, destination, length)
            })
            .collect::<Vec<_>>();

        ranges
    });

    let location = maps
        .collect::<Vec<_>>()
        .into_par_iter()
        .progress()
        .fold(
            || vec![seeds[0].clone()],
            |ranges, map_ranges| {
                dbg!(ranges.clone());
                let mut new_ranges: Vec<std::ops::Range<_>> = Vec::new();
                for range in ranges {
                    for r in map((range.start, range.end), map_ranges.clone()) {
                        new_ranges.push(std::ops::Range {
                            start: r.0,
                            end: r.1,
                        })
                    }
                }

                dbg!(new_ranges.clone());

                new_ranges
            },
        )
        .collect::<Vec<_>>()
        .iter()
        .flat_map(|r| r.iter().map(|r| Some(r.start)))
        .min();

    dbg!(location);
}

// 5 - 10 ; 3 - 7 - 12; 5 - 9 - 10

// 5 - 9
// 6 - 10
// 7 11
// 8 12
// 9 13
// 10 14

fn map(range: (i64, i64), mut ranges: Vec<(i64, i64, i64)>) -> Vec<(i64, i64)> {
    let mut s = range.0;
    let e = range.1;

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut new_ranges: Vec<(i64, i64)> = Vec::new();
    for (source, destination, length) in ranges {
        if source + length - 1 < s {
            continue;
        }

        if source > s {
            new_ranges.push((s, source - 1));
        }

        let shift = s - source;
        new_ranges.push((
            std::cmp::max(shift + destination, destination),
            std::cmp::min(destination + length - 1, e - s + destination + shift),
        ));
        s = s + length - shift;
    }

    if s <= e {
        new_ranges.push((s, e));
    }

    new_ranges
}

#[test]
fn map_works() {
    assert_eq!(vec![(9, 9), (6, 10)], map((5, 10), vec![(3, 7, 3)]));
    assert_eq!(vec![(9, 10), (7, 10)], map((5, 10), vec![(3, 7, 4)]));
    assert_eq!(vec![(9, 11), (8, 10)], map((5, 10), vec![(3, 7, 5)]));
    assert_eq!(vec![(9, 12), (9, 10)], map((5, 10), vec![(3, 7, 6)]));
    assert_eq!(vec![(9, 13), (10, 10)], map((5, 10), vec![(3, 7, 7)]));
    assert_eq!(vec![(9, 14)], map((5, 10), vec![(3, 7, 12)]));

    assert_eq!(vec![(7, 9), (8, 10)], map((5, 10), vec![(5, 7, 3)]));
    assert_eq!(vec![(5, 5), (7, 9), (9, 10)], map((5, 10), vec![(6, 7, 3)]));
    assert_eq!(
        vec![(5, 6), (8, 10), (10, 10)],
        map((5, 10), vec![(7, 8, 3)])
    );
}
