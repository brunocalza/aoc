use std::collections::HashMap;

fn main() {
    let input = include_str!("../../inputs/day3.txt");

    //let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

    let mut curr_x: i32 = 0;
    let mut curr_y: i32 = 0;

    let mut special_char_positions = HashMap::new();

    let input_with_coordinate = input.chars().map(|c| {
        let x = curr_x;
        let y = curr_y;

        if c == '\n' {
            curr_x += 1;
            curr_y = 0;
            return ((x, y), '.');
        }

        if !c.is_digit(10) && c != '.' {
            special_char_positions.insert((x, y), c);
        }

        curr_y += 1;
        ((x, y), c)
    });

    let mut map = HashMap::new();

    let numbers_with_coordinates = input_with_coordinate
        .fold(
            (false, String::new(), Vec::new()),
            |(is_prev_digit, mut acc_s, mut numbers), ((x, y), ch)| {
                if ch.is_digit(10) {
                    acc_s.push(ch);
                    return (true, acc_s, numbers);
                }

                if is_prev_digit {
                    let n = acc_s.len() as i32;
                    for i in (y - n)..y {
                        map.insert((x, i), acc_s.clone());
                    }

                    numbers.push(((x, y - n as i32), acc_s));
                }

                return (false, String::new(), numbers);
            },
        )
        .2;

    let r = numbers_with_coordinates
        .iter()
        .filter(|((x, y), n)| {
            return check(((*x, *y), n.to_string()), special_char_positions.clone()) > 0;
        })
        .map(|n| n.1.parse::<i32>().unwrap());

    let res2 = special_char_positions
        .iter()
        .filter(|(_, special_char)| return **special_char == '*')
        .fold(0, |acc, (coordinate, _)| {
            let r = match &check2(*coordinate, map.clone())[..] {
                [s1, s2] => {
                    println!("{} {}", s1, s2);
                    s1.parse::<i32>().unwrap() * s2.parse::<i32>().unwrap()
                }
                _ => 0,
            };
            acc + r
        });

    println!("{:?}", r.sum::<i32>());
    println!("{:?}", res2);
}

fn check(number: ((i32, i32), String), special_char_positions: HashMap<(i32, i32), char>) -> i32 {
    let mut count = 0;
    let mut coordinates = Vec::new();

    let start = number.0 .1 - 1;
    let end = start + number.1.len() as i32 + 1;
    for i in start..end + 1 {
        coordinates.push((number.0 .0 - 1, i));
        coordinates.push((number.0 .0 + 1, i));
    }

    coordinates.push((number.0 .0, start));
    coordinates.push((number.0 .0, end));

    for coordinate in coordinates {
        if special_char_positions.contains_key(&coordinate) {
            count += 1;
        }
    }

    return count;
}

fn check2(number: (i32, i32), m: HashMap<(i32, i32), String>) -> Vec<String> {
    let mut v: HashMap<String, i32> = HashMap::new();
    let mut coordinates = Vec::new();

    let start = number.1 - 1;
    let end = start + 2;
    for i in start..end + 1 {
        coordinates.push((number.0 - 1, i));
        coordinates.push((number.0 + 1, i));
    }

    coordinates.push((number.0, start));
    coordinates.push((number.0, end));

    for coordinate in coordinates {
        if m.contains_key(&coordinate) {
            v.insert(m.get(&coordinate).unwrap().to_owned(), 0);
        }
    }

    return v.into_keys().collect::<Vec<_>>();
}

#[test]
fn check_works() {
    assert_eq!(
        ["467".to_string(), "35".to_string()],
        check2(
            (1, 3),
            HashMap::from([
                ((0, 0), "467".into()),
                ((0, 1), "467".into()),
                ((0, 2), "467".into()),
                ((2, 3), "35".into()),
                ((2, 4), "35".into()),
            ])
        )[..]
    );
}

#[test]
fn input_with_coordinate_works() {
    let input = "1#.\n.11\n.11\n";

    let mut curr_x: i32 = 0;
    let mut curr_y: i32 = 0;

    let mut special_char_positions = HashMap::new();

    let input_with_coordinate = input.chars().map(|c| {
        let x = curr_x;
        let y = curr_y;

        if c == '\n' {
            curr_x += 1;
            curr_y = 0;
            return ((x, y), '.');
        }

        if !c.is_digit(10) && c != '.' {
            special_char_positions.insert((x, y), c);
        }

        curr_y += 1;
        ((x, y), c)
    });

    let numbers_with_coordinates = input_with_coordinate
        .fold(
            (false, String::new(), Vec::new()),
            |(is_prev_digit, mut acc_s, mut numbers), ((x, y), ch)| {
                println!("{:?} {}", (x, y), ch);
                if ch.is_digit(10) {
                    acc_s.push(ch);
                    return (true, acc_s, numbers);
                }

                if is_prev_digit {
                    numbers.push(((x, y - acc_s.len() as i32), acc_s));
                }

                return (false, String::new(), numbers);
            },
        )
        .2;

    println!("{:?}", numbers_with_coordinates);
    println!("{:?}", special_char_positions);
}
