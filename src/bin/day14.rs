use itertools::Itertools;

fn main() {
    let input = include_str!("../../inputs/day14.txt");

    let part1 = input
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>()
        .transpose()
        .tilt(false)
        .transpose()
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc: i32, (i, row)| {
            let rounded_rocks_count = row
                .chars()
                .fold(0, |acc, char| if char == 'O' { acc + 1 } else { acc });
            acc + (i as i32 + 1) * rounded_rocks_count
        });

    println!("{}", part1);

    // 100 96097
    // 101 96095
    // 102 96093
    // 103 96096
    // 104 96112
    // 105 96132
    // 106 96141
    // 107 96141
    // 108 96124
    // 109 96105
    // 110 96094

    let part2 = input
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>()
        .cycle(110 + 1000000000 % 11) // it repeats every 11 cyles
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc: i32, (i, row)| {
            let rounded_rocks_count = row
                .chars()
                .fold(0, |acc, char| if char == 'O' { acc + 1 } else { acc });
            acc + (i as i32 + 1) * rounded_rocks_count
        });

    println!("{}", part2);
}

trait Tilter {
    fn transpose(self) -> Vec<String>;
    fn tilt(self, reverse: bool) -> Vec<String>;
    fn cycle(self, n: i32) -> Vec<String>;
}

impl Tilter for Vec<String> {
    fn transpose(self) -> Vec<String> {
        let mut output = Vec::new();
        for y in 0..self[0].len() {
            let mut l = Vec::new();
            for line in self.clone() {
                l.push(line.chars().nth(y).unwrap());
            }
            let s = l.iter().collect::<String>();
            output.push(s);
        }

        output
    }

    fn tilt(self, reverse: bool) -> Vec<String> {
        let mut output = Vec::new();
        for line in self {
            let new_line = line
                .split("#")
                .map(|part| {
                    let mut chars: Vec<char> = part.chars().collect();
                    chars.sort_by(|a, b| if reverse { a.cmp(b) } else { b.cmp(a) });
                    chars.iter().collect::<String>()
                })
                .join("#");
            output.push(new_line);
        }

        output
    }

    fn cycle(self, n: i32) -> Vec<String> {
        let mut _self = self;

        for __ in 1..n + 1 {
            _self = _self
                .transpose()
                .tilt(false)
                .transpose()
                .tilt(false)
                .transpose()
                .tilt(true)
                .transpose()
                .tilt(true);
        }

        _self
    }
}
