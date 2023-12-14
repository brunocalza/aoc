fn main() {
    let input = include_str!("../../inputs/day9.txt");

    let part1: i32 = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|history| {
            generate_sequences(history)
                .into_iter()
                .map(|sequence| sequence.last().unwrap().clone())
                .sum::<i32>()
        })
        .sum::<i32>();

    let part2: i32 = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|history| {
            generate_sequences(history)
                .into_iter()
                .rev()
                .map(|sequence| sequence.first().unwrap().clone())
                .fold(0, |acc, n| n - acc)
        })
        .sum::<i32>();

    println!("{} {}", part1, part2);
}

fn generate_sequences(history: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sequences = Vec::new();
    let mut s = history.clone();
    sequences.push(s.clone());
    loop {
        s = generate_next_sequence(s);
        sequences.push(s.clone());
        if s.clone().into_iter().all(|n| n == 0) {
            break;
        }
    }

    sequences
}

fn generate_next_sequence(sequence: Vec<i32>) -> Vec<i32> {
    return sequence
        .clone()
        .into_iter()
        .zip(sequence.into_iter().skip(1))
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>();
}

#[test]
fn generate_next_sequence_works() {
    assert_eq!(
        [3, 3, 3, 3, 3],
        generate_next_sequence([0, 3, 6, 9, 12, 15].to_vec())[..]
    );
    assert_eq!(
        [2, 3, 4, 5, 6],
        generate_next_sequence([1, 3, 6, 10, 15, 21].to_vec())[..]
    );
    assert_eq!(
        [3, 3, 5, 9, 15],
        generate_next_sequence([10, 13, 16, 21, 30, 45].to_vec())[..]
    );

    assert_eq!(
        [
            0, -1, -6, -10, 0, 45, 154, 364, 720, 1275, 2090, 3234, 4784, 6825, 9450, 12760, 16864,
            21879, 27930, 35150
        ],
        generate_next_sequence(
            [
                12, 12, 11, 5, -5, -5, 40, 194, 558, 1278, 2553, 4643, 7877, 12661, 19486, 28936,
                41696, 58560, 80439, 108369, 143519
            ]
            .to_vec()
        )[..],
    );
}

#[test]
fn generate_sequence_works() {
    assert_eq!(
        [
            [0, 3, 6, 9, 12, 15].to_vec(),
            [3, 3, 3, 3, 3].to_vec(),
            [0, 0, 0, 0].to_vec()
        ]
        .to_vec(),
        generate_sequences([0, 3, 6, 9, 12, 15].to_vec())
    );
    assert_eq!(
        [
            [1, 3, 6, 10, 15, 21].to_vec(),
            [2, 3, 4, 5, 6].to_vec(),
            [1, 1, 1, 1].to_vec(),
            [0, 0, 0].to_vec()
        ]
        .to_vec(),
        generate_sequences([1, 3, 6, 10, 15, 21].to_vec())[..]
    );
    assert_eq!(
        [
            [10, 13, 16, 21, 30, 45].to_vec(),
            [3, 3, 5, 9, 15].to_vec(),
            [0, 2, 4, 6, 8].to_vec(),
            [2, 2, 2, 2].to_vec(),
            [0, 0].to_vec()
        ],
        generate_sequences([10, 13, 16, 21, 30, 45].to_vec())[..]
    );
}
