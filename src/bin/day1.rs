use std::collections::HashMap;

fn main() {
    let file = include_str!("../../inputs/day1.txt");

    let sum = file
        .lines()
        .map(replace)
        .map(extract_digits)
        .map(|digits| digits.parse::<i64>().unwrap())
        .sum::<i64>();

    println!("{}", sum);
}

fn replace(line: &str) -> String {
    let substitutions = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let mut s = String::new();
    for index in 0..line.len() {
        let l = &line[index..];
        
        for (pattern, ch) in &substitutions {
            if l.starts_with(pattern) {
                s.push(*ch);
            }
        }

        s.push(l.as_bytes()[0] as char);
    }

    s
}

fn extract_digits(line: String) -> String {
    let only_digits: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();

    only_digits
        .iter()
        .take(1)
        .chain(only_digits.last())
        .collect()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::extract_digits;
    use crate::replace;

    #[test_case("two1nine", "29")]
    #[test_case("eightwothree", "83")]
    #[test_case("abcone2threexyz", "13")]
    #[test_case("xtwone3four", "24")]
    #[test_case("4nineeightseven2", "42")]
    #[test_case("zoneight234", "14")]
    #[test_case("7pqrstsixteen", "76")]
    #[test_case("oneight", "18")]
    fn convert_works(line: &str, res: &str) {
        assert_eq!(res, extract_digits(replace(line)).as_str());
    }
}
