fn main() {
    let input = include_str!("../../inputs/day6.txt");

    let (times, distances) = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| {
            let (time_line, distance_line) = (
                chunk[0]
                    .split_ascii_whitespace()
                    .filter_map(|part| part.parse::<i64>().ok())
                    .collect::<Vec<i64>>(),
                chunk[1]
                    .split_ascii_whitespace()
                    .filter_map(|part| part.parse::<i64>().ok())
                    .collect::<Vec<i64>>(),
            );

            (time_line, distance_line)
        })
        .next()
        .unwrap();

    let mut res = 1;
    for (index, time) in times.iter().enumerate() {
        let mut count = 0;
        for t in 1..time + 1 {
            if t * (time - t) > distances[index] {
                count += 1;
            }
        }
        res *= count;
    }

    dbg!(times, distances, res);
}
