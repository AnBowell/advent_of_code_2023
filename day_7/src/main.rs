use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

const FILE_LOC: &'static str = "data/input.txt";

fn main() {
    problem_one();
    problem_two()
}
fn problem_two() {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut hand_types = Vec::with_capacity(1000);

    let mut replace_hashmap = HashMap::new();

    replace_hashmap.insert('J', 1);
    replace_hashmap.insert('T', 10);
    replace_hashmap.insert('Q', 12);
    replace_hashmap.insert('K', 13);
    replace_hashmap.insert('A', 14);

    for line in lines {
        let mut split_line = line.as_ref().unwrap().split_whitespace();

        let hand = split_line.next().unwrap();
        let bid = split_line.next().unwrap().parse::<u64>().unwrap();

        hand_types.push(HandStats::from_hand_with_joker(hand, bid, &replace_hashmap));
    }

    hand_types.sort_by_key(|item| (item.hand_type, item.card_value_vec.clone()));
    // hand_types.reverse();

    let total_winnings = hand_types
        .iter()
        .enumerate()
        .fold(0, |acc, (counter, hand_stats)| {
            acc + ((counter + 1) as u64 * hand_stats.bid)
        });

    println!("Problem two total winnings: {}", total_winnings);
}

fn problem_one() {
    let file = File::open(FILE_LOC).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut hand_types = Vec::with_capacity(1000);

    let mut replace_hashmap = HashMap::new();

    replace_hashmap.insert('T', 10);
    replace_hashmap.insert('J', 11);
    replace_hashmap.insert('Q', 12);
    replace_hashmap.insert('K', 13);
    replace_hashmap.insert('A', 14);

    for line in lines {
        let mut split_line = line.as_ref().unwrap().split_whitespace();

        let hand = split_line.next().unwrap();
        let bid = split_line.next().unwrap().parse::<u64>().unwrap();

        hand_types.push(HandStats::from_hand(hand, bid, &replace_hashmap));
    }

    hand_types.sort_by_key(|item| (item.hand_type, item.card_value_vec.clone()));

    let total_winnings = hand_types
        .iter()
        .enumerate()
        .fold(0, |acc, (counter, hand_stats)| {
            acc + ((counter + 1) as u64 * hand_stats.bid)
        });

    println!("Problem one total winnings: {}", total_winnings);
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct HandStats {
    hand_type: HandType,
    card_value_vec: Vec<u64>,
    bid: u64,
}

impl HandStats {
    pub fn from_hand(hand: &str, bid: u64, replace_hashmap: &HashMap<char, u64>) -> Self {
        let unique_cards = hand.chars().collect::<HashSet<char>>();

        let hand_type = hand_type_from_cards(&unique_cards, hand);

        let mut card_value_vec = Vec::with_capacity(hand.len());

        for card in hand.chars() {
            if card.is_numeric() {
                card_value_vec.push(card.to_string().parse::<u64>().unwrap());
            } else {
                card_value_vec.push(*replace_hashmap.get(&card).unwrap())
            }
        }

        return Self {
            hand_type,
            card_value_vec,
            bid,
        };
    }

    pub fn from_hand_with_joker(
        hand: &str,
        bid: u64,
        replace_hashmap: &HashMap<char, u64>,
    ) -> Self {
        let joker_count = hand.chars().filter(|x| x == &'J').count();

        let hand_no_joker = hand.replace("J", "");

        let mut unique_cards = hand.chars().collect::<HashSet<char>>();

        let hand_type = if joker_count > 0 {
            unique_cards.remove(&'J');
            hand_type_from_cards_joker_filtered(&unique_cards, &hand_no_joker)
        } else {
            hand_type_from_cards(&unique_cards, hand)
        };

        let mut card_value_vec = Vec::with_capacity(hand.len());

        for card in hand.chars() {
            if card.is_numeric() {
                card_value_vec.push(card.to_string().parse::<u64>().unwrap());
            } else {
                card_value_vec.push(*replace_hashmap.get(&card).unwrap())
            }
        }

        return Self {
            hand_type,
            card_value_vec,
            bid,
        };
    }
}

fn hand_type_from_cards(unique_cards: &HashSet<char>, hand: &str) -> HandType {
    match unique_cards.len() {
        1 => HandType::FiveOfAKind,
        2 => {
            let mut hand_type = HandType::FullHouse;

            for card in unique_cards.iter() {
                let card_count = hand.chars().filter(|x| x == card).count();
                if card_count == 1 || card_count == 4 {
                    hand_type = HandType::FourOfAKind;
                }
            }
            hand_type
        }
        3 => {
            let mut hand_type = HandType::TwoPair;

            for card in unique_cards.iter() {
                let card_count = hand.chars().filter(|x| x == card).count();
                if card_count == 3 {
                    hand_type = HandType::ThreeOfAKind;
                }
            }
            hand_type
        }
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => panic!("Not possible!"),
    }
}

/// 1
/// 1JJJJ 5ok
/// 2
/// 23JJJ 4ok
/// 323JJ 4ok
/// 3331J 4ok
/// 3311J Fullhouse
/// 3
/// 123JJ 3ok
/// 1233J 3ok
/// 4
/// 1234J pair
/// 5
/// 12345 HighCard
///
fn hand_type_from_cards_joker_filtered(unique_cards: &HashSet<char>, hand: &str) -> HandType {
    match unique_cards.len() {
        0 => HandType::FiveOfAKind,
        1 => HandType::FiveOfAKind,
        2 => {
            let mut hand_type = HandType::FullHouse;

            for card in unique_cards.iter() {
                let card_count = hand.chars().filter(|x| x == card).count();
                if card_count == 1 {
                    hand_type = HandType::FourOfAKind;
                }
            }
            hand_type
        }
        3 => HandType::ThreeOfAKind,
        4 => HandType::OnePair,
        _ => panic!("Not possible!"),
    }
}
