use std::collections::HashMap;

fn main() {
    let input = include_str!("../../inputs/day8.txt");

    let mut lines = input.lines();
    let directions = lines.next().unwrap();

    lines.next();

    let mut network: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in lines {
        let (lhs, rhs) = line.split_once(" = ").unwrap();
        network.insert(
            lhs,
            rhs.trim_start_matches("(")
                .trim_end_matches(")")
                .split_once(", ")
                .unwrap(),
        );
    }

    let positions = network
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(|key| network[key])
        .collect::<Vec<_>>();
    for position in positions {
        let mut p = position.clone();
        let mut direction_index = 0;
        let mut steps = 1;

        loop {
            let direction = directions.as_bytes()[direction_index] as char;
            direction_index = (direction_index + 1) % directions.len();
            steps += 1;

            let next_position: (&str, &str);
            if direction == 'L' {
                next_position = network[p.0]
            } else {
                next_position = network[p.1]
            }

            let ends_with_z = next_position.0.ends_with("Z") || next_position.1.ends_with("Z");
            if ends_with_z && direction_index == directions.len() - 1 {
                println!("{}", steps);
                break;
            }
            p = next_position;
        }
    }
}
