use itertools::Itertools;

fn main() {
    let input = include_str!("../../inputs/day12.txt");
    let s: i64 = input
        .lines()
        .map(|line| {
            let (arrangement, spec) = line.split_once(" ").unwrap();
            count_combinations_that_matches_spec(
                arrangement,
                0,
                spec.split(",")
                    .map(|c| c.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .sum();

    println!("{}", s);
}

// #[test]
// fn count_combinations_that_matches_spec_works() {
//     assert_eq!(1, count_combinations_that_matches_spec("???.###", "1,1,3"));
//     assert_eq!(4, count_combinations_that_matches_spec(".??..??...?##.", "1,1,3"));
//     assert_eq!(4, count_combinations_that_matches_spec("????.######..#####.", "1,6,5"));
//     assert_eq!(10, count_combinations_that_matches_spec("?###????????", "3,2,1"));
// }

fn count_combinations_that_matches_spec(
    input: &str,
    index: usize,
    arrangement_spec: Vec<i32>,
) -> i64 {
    if arrangement_spec.len() == 0 || (arrangement_spec.len() == 1 && arrangement_spec[0] == 0) {
        if !input[index..].contains("#") {
            println!("{} {:?}", &input[index..], arrangement_spec);
            return 1;
        }
        return 0;
    }

    if index >= input.len() {
        return 0;
    }

    println!("{} {:?}", &input[index..], arrangement_spec);

    let (new_spec, is_zero) = if arrangement_spec[0] == 0 {
        (arrangement_spec[1..].to_vec(), true)
    } else {
        let mut v = [arrangement_spec[0] - 1].to_vec();
        v.extend_from_slice(&arrangement_spec[1..]);
        (v, false)
    };

    return match &input[index..index + 1] {
        "?" => {
            if is_zero {
                return count_combinations_that_matches_spec(input, index + 1, new_spec);
            } else {
                return count_combinations_that_matches_spec(input, index + 1, arrangement_spec)
                    + count_combinations_that_matches_spec(input, index + 1, new_spec);
            }
        }
        "." => {
            if is_zero {
                count_combinations_that_matches_spec(input, index + 1, new_spec)
            } else {
                count_combinations_that_matches_spec(input, index + 1, arrangement_spec)
            }
        }
        "#" => {
            if !is_zero {
                return count_combinations_that_matches_spec(input, index + 1, new_spec);
            }

            return 0;
        }
        _ => unreachable!(),
    };
}

// #[test]
// fn contigous_groups_works() {
//     assert_eq!("1,1,3", contigous_groups("#.#.###"));
//     assert_eq!("1,1,3", contigous_groups(".#...#....###."));
//     assert_eq!("1,3,1,6", contigous_groups(".#.###.#.######"));
//     assert_eq!("4,1,1", contigous_groups("####.#...#..."));
//     assert_eq!("1,6,5", contigous_groups("#....######..#####."));
//     assert_eq!("3,2,1", contigous_groups(".###.##....#"));
// }

fn contigous_groups(input: &str) -> String {
    let (mut v, count) = input
        .chars()
        .fold((Vec::new(), 0), |(mut v, mut count), ch| {
            if ch == '#' {
                count += 1;
            } else if count > 0 {
                v.push(count);
                count = 0;
            }

            (v, count)
        });

    if count > 0 {
        v.push(count);
    }

    v.into_iter().map(|v| v.to_string()).join(",")
}
