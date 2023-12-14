use itertools::Itertools;

fn main() {
    let input = include_str!("../../inputs/day11.txt");

    let mut galaxies = Vec::new();
    let mut universe = Vec::new();
    for (x, line) in input.lines().enumerate() {
        let mut column = Vec::new();
        for (y, char) in line.chars().enumerate() {
            if char == '#' {
                galaxies.push((x as i64, y as i64));
            }
            column.push(char);
        }
        universe.push(column);
    }

    let mut empty_rows: Vec<_> = Vec::new();
    for (i, row) in universe.clone().into_iter().enumerate() {
        let mut empty = true;
        for c in row {
            if c == '#' {
                empty = false;
            }
        }
        if empty {
            empty_rows.push(i);
        }
    }

    let mut empty_cols = Vec::new();
    for y in 0..universe[0].len() {
        let mut empty = true;
        for x in 0..universe.len() {
            if universe[x][y] == '#' {
                empty = false;
            }
        }
        if empty {
            empty_cols.push(y);
        }
    }

    let sum = galaxies
        .into_iter()
        .permutations(2)
        .unique()
        .fold(0, |acc: i64, pair| {
            let mut d = acc + (pair[0].0 - pair[1].0).abs() + (pair[0].1 - pair[1].1).abs();

            for er in empty_rows.clone() {
                if std::cmp::min(pair[0].0, pair[1].0) <= (er as i64)
                    && (er as i64) <= std::cmp::max(pair[0].0, pair[1].0)
                {
                    d += 1000000 - 1;
                }
            }

            for ec in empty_cols.clone() {
                if std::cmp::min(pair[0].1, pair[1].1) <= (ec as i64)
                    && (ec as i64) <= std::cmp::max(pair[0].1, pair[1].1)
                {
                    d += 1000000 - 1;
                }
            }

            d
        });

    print!("{}", sum / 2);
}
