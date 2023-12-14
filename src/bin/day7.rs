use itertools::Itertools;
use std::iter;
use std::slice::Iter;
use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = include_str!("../../inputs/day7.txt");
    let mut hands = input
        .lines()
        .map(|line| {
            let (hand, bet_str) = line.split_once(" ").unwrap();
            let bet = bet_str
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<i32>()
                .unwrap();

            (Hand::parse(hand), bet)
        })
        .collect::<Vec<_>>();

    hands.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let s: i32 = hands
        .into_iter()
        .enumerate()
        .map(|(rank, (_, bet))| (rank + 1) as i32 * bet)
        .sum();

    dbg!(s);
}

type LabelCount = HashMap<Label, i32>;

trait LabelCountExt {
    fn is_high_card(&self) -> bool;
    fn is_full_house(&self) -> bool;
    fn is_one_pair(&self) -> bool;
    fn is_two_pair(&self) -> bool;
    fn is_three_of_kind(&self) -> bool;
    fn is_four_of_kind(&self) -> bool;
    fn is_five_of_kind(&self) -> bool;
    fn highest(&self) -> HandKind;
}

impl LabelCountExt for LabelCount {
    fn is_high_card(&self) -> bool {
        return self.clone().into_iter().all(|(_, count)| count == 1);
    }

    fn is_full_house(&self) -> bool {
        return self
            .clone()
            .into_iter()
            .filter(|(_, count)| *count == 2)
            .count()
            == 1
            && self
                .clone()
                .into_iter()
                .filter(|(_, count)| *count == 3)
                .count()
                == 1;
    }

    fn is_one_pair(&self) -> bool {
        return self
            .clone()
            .into_iter()
            .filter(|(_, count)| *count == 2)
            .count()
            == 1;
    }

    fn is_two_pair(&self) -> bool {
        return self
            .clone()
            .into_iter()
            .filter(|(_, count)| *count == 2)
            .count()
            == 2;
    }

    fn is_three_of_kind(&self) -> bool {
        return self
            .clone()
            .into_iter()
            .filter(|(_, count)| *count == 3)
            .count()
            == 1;
    }

    fn is_four_of_kind(&self) -> bool {
        return self
            .clone()
            .into_iter()
            .filter(|(_, count)| *count == 4)
            .count()
            == 1;
    }

    fn is_five_of_kind(&self) -> bool {
        return self
            .clone()
            .into_iter()
            .filter(|(_, count)| *count == 5)
            .count()
            == 1;
    }

    fn highest(&self) -> HandKind {
        if self.is_five_of_kind() {
            return HandKind::FiveOfKind;
        }

        if self.is_four_of_kind() {
            return HandKind::FourOfKind;
        }

        if self.is_full_house() {
            return HandKind::FullHouse;
        }

        if self.is_three_of_kind() {
            return HandKind::ThreeOfKind;
        }

        if self.is_two_pair() {
            return HandKind::TwoPair;
        }

        if self.is_one_pair() {
            return HandKind::OnePair;
        }

        return HandKind::HighCard;
    }
}

#[derive(Debug, Eq, Hash, Clone, PartialEq, PartialOrd, Ord)]
enum Label {
    J,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    T,
    Q,
    K,
    A,
}

impl Label {
    pub fn iterator() -> Iter<'static, Label> {
        static LABELS: [Label; 12] = [
            Label::TWO,
            Label::THREE,
            Label::FOUR,
            Label::FIVE,
            Label::SIX,
            Label::SEVEN,
            Label::EIGHT,
            Label::NINE,
            Label::T,
            Label::Q,
            Label::K,
            Label::A,
        ];
        LABELS.iter()
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl PartialOrd for HandKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_discriminant = unsafe { *<*const _>::from(self).cast::<u8>() } as i8;
        let other_discriminant = unsafe { *<*const _>::from(other).cast::<u8>() } as i8;

        return self_discriminant.partial_cmp(&other_discriminant);
    }
}

#[derive(Debug)]
struct Hand {
    variant: HandKind,
    value: Vec<Label>,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.variant == other.variant;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.variant != other.variant {
            return self.variant.partial_cmp(&other.variant);
        }

        return self.value.partial_cmp(&other.value);
    }
}

impl Hand {
    fn parse(hand: &str) -> Hand {
        if hand.len() != 5 {
            panic!("invalid length")
        }

        let count: LabelCount = hand.chars().fold(HashMap::new(), |mut count, c| {
            let label = match c {
                'A' => Label::A,
                'K' => Label::K,
                'Q' => Label::Q,
                'J' => Label::J,
                'T' => Label::T,
                '9' => Label::NINE,
                '8' => Label::EIGHT,
                '7' => Label::SEVEN,
                '6' => Label::SIX,
                '5' => Label::FIVE,
                '4' => Label::FOUR,
                '3' => Label::THREE,
                '2' => Label::TWO,
                _ => unreachable!(),
            };
            *count.entry(label).or_insert(0) += 1;
            count
        });

        let h = hand
            .chars()
            .map(|c| match c {
                'A' => Label::A,
                'K' => Label::K,
                'Q' => Label::Q,
                'J' => Label::J,
                'T' => Label::T,
                '9' => Label::NINE,
                '8' => Label::EIGHT,
                '7' => Label::SEVEN,
                '6' => Label::SIX,
                '5' => Label::FIVE,
                '4' => Label::FOUR,
                '3' => Label::THREE,
                '2' => Label::TWO,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();

        if hand.contains('J') {
            let n_j = hand.chars().filter(|c| *c == 'J').count();

            let asjdl = Label::iterator()
                .flat_map(|l| iter::repeat(l).take(n_j))
                .collect::<Vec<&Label>>()
                .chunks(n_j)
                .map(|chunk| chunk[..].to_vec())
                .collect::<Vec<_>>();
            let mut possible_values = Label::iterator()
                .permutations(n_j)
                .unique()
                .chain(asjdl)
                .map(|permutation| {
                    let mut count_copy = count.clone();
                    for label in permutation.clone() {
                        *count_copy.entry(label.clone()).or_insert(0) += 1;
                        count_copy.remove_entry(&Label::J);
                    }

                    (permutation, count_copy.highest())
                });

            let (curr_permutation, curr_kind) = possible_values.next().unwrap();
            let pick = possible_values.fold(
                (curr_permutation, curr_kind),
                |(prev_p, prev_k), (curr_p, curr_k)| {
                    if curr_k > prev_k {
                        return (curr_p, curr_k);
                    }
                    (prev_p, prev_k)
                },
            );

            return Hand {
                variant: pick.1,
                value: h,
            };
        }

        if count.is_high_card() {
            return Hand {
                variant: HandKind::HighCard,
                value: h,
            };
        }

        if count.is_full_house() {
            return Hand {
                variant: HandKind::FullHouse,
                value: h,
            };
        }

        if count.is_one_pair() {
            return Hand {
                variant: HandKind::OnePair,
                value: h,
            };
        }

        if count.is_two_pair() {
            return Hand {
                variant: HandKind::TwoPair,
                value: h,
            };
        }

        if count.is_three_of_kind() {
            return Hand {
                variant: HandKind::ThreeOfKind,
                value: h,
            };
        }

        if count.is_four_of_kind() {
            return Hand {
                variant: HandKind::FourOfKind,
                value: h,
            };
        }

        if count.is_five_of_kind() {
            return Hand {
                variant: HandKind::FiveOfKind,
                value: h,
            };
        }

        unreachable!()
    }
}

#[test]
fn hand_cmp_works() {
    assert_eq!(HandKind::HighCard.eq(&HandKind::HighCard), true);
    assert_eq!(HandKind::HighCard.eq(&HandKind::FiveOfKind), false);
    assert_eq!(HandKind::HighCard < HandKind::FiveOfKind, true);
}

#[test]
fn hand_parse_works() {
    assert_eq!(
        Hand::parse("72345"),
        Hand {
            variant: HandKind::HighCard,
            value: vec![
                Label::SEVEN,
                Label::TWO,
                Label::THREE,
                Label::FOUR,
                Label::FIVE
            ]
        }
    );
    assert_eq!(
        Hand::parse("72375"),
        Hand {
            variant: HandKind::OnePair,
            value: vec![
                Label::SEVEN,
                Label::TWO,
                Label::THREE,
                Label::SEVEN,
                Label::FIVE
            ]
        }
    );
    assert_eq!(
        Hand::parse("72275"),
        Hand {
            variant: HandKind::TwoPair,
            value: vec![
                Label::SEVEN,
                Label::TWO,
                Label::TWO,
                Label::SEVEN,
                Label::FIVE
            ]
        }
    );
    assert_eq!(
        Hand::parse("72555"),
        Hand {
            variant: HandKind::ThreeOfKind,
            value: vec![
                Label::SEVEN,
                Label::TWO,
                Label::FIVE,
                Label::FIVE,
                Label::FIVE
            ]
        }
    );
    assert_eq!(
        Hand::parse("22225"),
        Hand {
            variant: HandKind::FourOfKind,
            value: vec![Label::TWO, Label::TWO, Label::TWO, Label::TWO, Label::FIVE]
        }
    );
    assert_eq!(
        Hand::parse("KKKKK"),
        Hand {
            variant: HandKind::FiveOfKind,
            value: vec![Label::K, Label::K, Label::K, Label::K, Label::K]
        }
    );
    assert_eq!(
        Hand::parse("22233"),
        Hand {
            variant: HandKind::FullHouse,
            value: vec![
                Label::TWO,
                Label::TWO,
                Label::TWO,
                Label::THREE,
                Label::THREE
            ]
        }
    );

    assert_eq!(Hand::parse("T55J5").variant, HandKind::FourOfKind);
    assert_eq!(Hand::parse("JJJJJ").variant, HandKind::FiveOfKind);
    assert_eq!(Hand::parse("7JJJ7").variant, HandKind::FiveOfKind);

    assert_eq!(Hand::parse("JJ3J3").variant, HandKind::FiveOfKind);
}

#[test]
fn hand_cmp() {
    assert_eq!(
        Hand::parse("KK677")
            .partial_cmp(&Hand::parse("KTJJT"))
            .unwrap(),
        Ordering::Greater
    );
    assert_eq!(
        Hand::parse("T55J5")
            .partial_cmp(&Hand::parse("QQQJA"))
            .unwrap(),
        Ordering::Less
    );
    assert_eq!(
        Hand::parse("77888")
            .partial_cmp(&Hand::parse("77788"))
            .unwrap(),
        Ordering::Greater
    );
}

#[test]
fn permutations() {
    //dbg!(Label::iterator().permutations(2).collect::<Vec<_>>());
    dbg!(Label::iterator()
        .flat_map(|l| iter::repeat(l).take(2))
        .collect::<Vec<_>>()
        .chunks(2)
        .collect::<Vec<_>>());
}
