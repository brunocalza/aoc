use std::{
    collections::{HashMap, HashSet},
    iter,
};

fn main() {
    let input = include_str!("../../inputs/day10.txt");
    let grid = parse_grid(input);

    // part 1
    println!(
        "{:?}",
        farthest_from_s_in_loop(grid.clone(), Direction::Down)
    );

    let mut set: HashSet<(i32, i32)> = HashSet::new();
    for pos in find_loop(grid, Direction::Down).unwrap() {
        set.insert(pos);
    }

    let mut count = 0;
    for (x, line) in input.lines().enumerate() {
        for (y, ch) in line.chars().enumerate() {
            if !set.contains(&(x as i32, y as i32)) {
                let c = line
                    .chars()
                    .take(y)
                    .filter(|&ch| ch == '|' || ch == 'L' || ch == 'J')
                    .count();
                if c % 2 == 1 {
                    print!("{}", "I");
                    count += 1
                } else {
                    print!("{}", ".");
                }
            } else {
                print!("{}", ch);
            }
        }
        print!("\n");
    }

    dbg!(count);
}

fn parse_grid(input: &str) -> Grid {
    let tiles = input
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(move |(y, ch)| {
                    let kind = match ch {
                        'S' => TileKind::START,
                        '-' => TileKind::HORIZONTAL,
                        '|' => TileKind::VERTICAL,
                        'L' => TileKind::L,
                        'J' => TileKind::J,
                        '7' => TileKind::SEVEN,
                        'F' => TileKind::F,
                        '.' => TileKind::GROUND,
                        _ => {
                            unreachable!();
                        }
                    };

                    Tile {
                        coord: (x as i32, y as i32),
                        kind: kind,
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for lines in tiles.clone() {
        for tile in lines {
            if tile.kind == TileKind::START {
                return Grid {
                    start: tile.clone(),
                    standing_at: tile,
                    tiles: tiles.clone(),
                    dimentions: (tiles.len() as i32, tiles[0].len() as i32),
                };
            }
        }
    }

    unreachable!();
}

fn farthest_from_s_in_loop(grid: Grid, direction: Direction) -> Option<i32> {
    return find_loop(grid, direction).and_then(|l| Some(l.len() as i32 / 2));
}

fn find_loop(grid: Grid, direction: Direction) -> Option<Vec<(i32, i32)>> {
    let mut direction = direction;
    let mut l = Vec::new();
    let mut grid = grid;

    l.push(grid.standing_at.coord);

    loop {
        let next_tile = grid.next(direction);
        if next_tile.is_none() {
            return None;
        }

        let tile = next_tile.unwrap();
        l.push(tile.0.coord);
        if tile.0.kind == TileKind::START {
            return Some(l);
        }
        direction = tile.1;
    }
}

#[test]
fn parse_grid_works() {
    let input = ".....
.S-7.
.|.|.
.L-J.
.....";

    let mut grid = parse_grid(input);
    assert_eq!(
        (
            Tile {
                coord: (1, 2),
                kind: TileKind::HORIZONTAL
            },
            Direction::Right
        ),
        grid.next(Direction::Right).unwrap()
    );
    assert_eq!(
        (
            Tile {
                coord: (1, 3),
                kind: TileKind::SEVEN
            },
            Direction::Right
        ),
        grid.next(Direction::Right).unwrap()
    );
    assert_eq!(
        (
            Tile {
                coord: (2, 3),
                kind: TileKind::VERTICAL
            },
            Direction::Down
        ),
        grid.next(Direction::Right).unwrap()
    );
    assert_eq!(
        (
            Tile {
                coord: (3, 3),
                kind: TileKind::J
            },
            Direction::Down
        ),
        grid.next(Direction::Down).unwrap()
    );
    assert_eq!(
        (
            Tile {
                coord: (3, 2),
                kind: TileKind::HORIZONTAL
            },
            Direction::Left
        ),
        grid.next(Direction::Down).unwrap()
    );
    assert_eq!(
        (
            Tile {
                coord: (3, 1),
                kind: TileKind::L
            },
            Direction::Left
        ),
        grid.next(Direction::Left).unwrap()
    );
    assert_eq!(
        (
            Tile {
                coord: (2, 1),
                kind: TileKind::VERTICAL
            },
            Direction::Up
        ),
        grid.next(Direction::Left).unwrap()
    );
    assert_eq!(
        (
            Tile {
                coord: (1, 1),
                kind: TileKind::START
            },
            Direction::Up
        ),
        grid.next(Direction::Up).unwrap()
    );
}

#[derive(PartialEq, Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Debug, Clone)]
enum TileKind {
    START,
    HORIZONTAL,
    VERTICAL,
    L,
    J,
    SEVEN,
    F,
    GROUND,
}

#[derive(Debug, Clone, PartialEq)]
struct Tile {
    coord: (i32, i32),
    kind: TileKind,
}

impl Tile {
    fn next(&self, direction: Direction) -> ((i32, i32), Direction) {
        match self.kind {
            TileKind::VERTICAL => {
                if direction == Direction::Up {
                    ((self.coord.0 - 1, self.coord.1), Direction::Up)
                } else {
                    ((self.coord.0 + 1, self.coord.1), Direction::Down)
                }
            }
            TileKind::HORIZONTAL => {
                if direction == Direction::Right {
                    ((self.coord.0, self.coord.1 + 1), Direction::Right)
                } else {
                    ((self.coord.0, self.coord.1 - 1), Direction::Left)
                }
            }
            TileKind::L => {
                if direction == Direction::Down {
                    ((self.coord.0, self.coord.1 + 1), Direction::Right)
                } else {
                    ((self.coord.0 - 1, self.coord.1), Direction::Up)
                }
            }
            TileKind::J => {
                if direction == Direction::Down {
                    ((self.coord.0, self.coord.1 - 1), Direction::Left)
                } else {
                    ((self.coord.0 - 1, self.coord.1), Direction::Up)
                }
            }
            TileKind::SEVEN => {
                if direction == Direction::Right {
                    ((self.coord.0 + 1, self.coord.1), Direction::Down)
                } else {
                    ((self.coord.0, self.coord.1 - 1), Direction::Left)
                }
            }
            TileKind::F => {
                if direction == Direction::Up {
                    ((self.coord.0, self.coord.1 + 1), Direction::Right)
                } else {
                    ((self.coord.0 + 1, self.coord.1), Direction::Down)
                }
            }
            TileKind::START => match direction {
                Direction::Up => ((self.coord.0 - 1, self.coord.1), direction),
                Direction::Down => ((self.coord.0 + 1, self.coord.1), direction),
                Direction::Left => ((self.coord.0, self.coord.1 - 1), direction),
                Direction::Right => ((self.coord.0, self.coord.1 + 1), direction),
            },
            _ => {
                unreachable!();
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    start: Tile,
    tiles: Vec<Vec<Tile>>,
    dimentions: (i32, i32),
    standing_at: Tile,
}

impl Grid {
    fn next(&mut self, direction: Direction) -> Option<(Tile, Direction)> {
        let (next_coordinate, direction) = self.standing_at.next(direction);
        if next_coordinate.0 < 0
            || next_coordinate.0 >= self.dimentions.0
            || next_coordinate.1 < 0
            || next_coordinate.1 >= self.dimentions.1
        {
            return None;
        }

        let next_tile = &self.tiles[next_coordinate.0 as usize][next_coordinate.1 as usize];
        if next_tile.kind == TileKind::GROUND {
            return None;
        }

        self.standing_at = next_tile.clone();
        return Some((next_tile.clone(), direction));
    }
}
