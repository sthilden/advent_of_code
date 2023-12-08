use std::{fs, path::Path};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum Value {
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    T,
    J,
    Q,
    K,
    A,
}

impl Value {
    fn from_char(c: char) -> Option<Value> {
        match c {
            '2' => Some(Value::V2),
            '3' => Some(Value::V3),
            '4' => Some(Value::V4),
            '5' => Some(Value::V5),
            '6' => Some(Value::V6),
            '7' => Some(Value::V7),
            '8' => Some(Value::V8),
            '9' => Some(Value::V9),
            'T' => Some(Value::T),
            'J' => Some(Value::J),
            'Q' => Some(Value::Q),
            'K' => Some(Value::K),
            'A' => Some(Value::A),
            _ => None,
        }
    }

    fn joker_value(&self) -> u8 {
        match self {
            Value::V2 => 2,
            Value::V3 => 3,
            Value::V4 => 4,
            Value::V5 => 5,
            Value::V6 => 6,
            Value::V7 => 7,
            Value::V8 => 8,
            Value::V9 => 9,
            Value::T => 10,
            Value::J => 1, // weakest
            Value::Q => 11,
            Value::K => 12,
            Value::A => 13,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Type {
    fn from_cards(cards: &[Value; 5]) -> Type {
        let mut cards_copy = cards.clone();
        cards_copy.sort();
        let [a, b, c, d, e] = cards_copy;
        if a == b && b == c && c == d && d == e {
            return Type::FiveOfAKind;
        }
        if (a == b && b == c && c == d) || (b == c && c == d && d == e) {
            return Type::FourOfAKind;
        }
        if ((a == b && b == c) && (d == e)) || ((a == b) && (c == d && d == e)) {
            return Type::FullHouse;
        }
        if (a == b && b == c) || (b == c && c == d) || (c == d && d == e) {
            return Type::ThreeOfAKind;
        }
        if (a == b && c == d) || (a == b && d == e) || (b == c && d == e) {
            return Type::TwoPair;
        }
        if a == b || b == c || c == d || d == e {
            return Type::OnePair;
        }
        Type::HighCard
    }

    fn from_cards_with_joker(cards: &[Value; 5]) -> Type {
        // Find all Jokers
        let mut jokers: Vec<usize> = Vec::new();
        cards.into_iter().enumerate().for_each(|(inx, card)| {
            if *card == Value::J {
                jokers.push(inx);
            }
        });

        // All cards are jokers, or there are no jokers
        if jokers.len() == 5 || jokers.is_empty() {
            return Type::from_cards(cards);
        }

        // Loop over all non-Joker characters, and replace all Jokers with that character.
        // Then compute the type from each hand, and find the maximum type.
        let mut max_type: Option<Type> = None;
        cards.into_iter().for_each(|card| {
            if *card != Value::J {
                let mut cards_clone = cards.clone();
                for inx in &jokers {
                    cards_clone[*inx] = *card;
                }
                let t = Type::from_cards(&cards_clone);
                if max_type.is_none() || t > *max_type.as_ref().unwrap() {
                    max_type = Some(t);
                }
            }
        });
        max_type.unwrap()
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Hand {
    hand_type: Type,
    cards: [Value; 5],
}

impl Hand {
    fn get_cards(s: &str) -> Option<[Value; 5]> {
        if s.len() != 5 {
            return None;
        }
        Some([
            Value::from_char(s.chars().nth(0)?)?,
            Value::from_char(s.chars().nth(1)?)?,
            Value::from_char(s.chars().nth(2)?)?,
            Value::from_char(s.chars().nth(3)?)?,
            Value::from_char(s.chars().nth(4)?)?,
        ])
    }

    fn from_str(s: &str) -> Option<Hand> {
        let cards = Hand::get_cards(s)?;
        Some(Hand {
            cards,
            hand_type: Type::from_cards(&cards),
        })
    }

    fn from_str_with_joker(s: &str) -> Option<Hand> {
        let cards = Hand::get_cards(s)?;
        Some(Hand {
            cards,
            hand_type: Type::from_cards_with_joker(&cards),
        })
    }
}

pub fn problem1() {
    println!("problem 1");

    let path = Path::new("resources/day7_input");
    let contents = fs::read_to_string(path).unwrap();

    let mut hands = contents
        .lines()
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (hand_str, bid) = line.split_once(' ').unwrap();
            let hand = Hand::from_str(hand_str).unwrap();
            (hand, bid.parse::<usize>().unwrap())
        })
        .collect::<Vec<(Hand, usize)>>();

    // Sort in order by hand
    hands.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0));

    // Compute winnings
    let mut total_winnings = 0;
    hands.into_iter().enumerate().for_each(|(inx, hand)| {
        let rank = inx + 1;
        let bid = hand.1;
        total_winnings += rank * bid;
    });

    println!("Answer: {}", total_winnings);
}

pub fn problem2() {
    println!("problem 2");

    let path = Path::new("resources/day7_input");
    let contents = fs::read_to_string(path).unwrap();

    let mut hands = contents
        .lines()
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (hand_str, bid) = line.split_once(' ').unwrap();
            let hand = Hand::from_str_with_joker(hand_str).unwrap();
            (hand, bid.parse::<usize>().unwrap())
        })
        .collect::<Vec<(Hand, usize)>>();

    // Sort in order by hand (sort first cards using J as the weakest card)
    hands.sort_by(|lhs, rhs| {
        lhs.0
            .cards
            .into_iter()
            .map(|v| v.joker_value())
            .collect::<Vec<u8>>()
            .cmp(
                &rhs.0
                    .cards
                    .into_iter()
                    .map(|v| v.joker_value())
                    .collect::<Vec<u8>>(),
            )
    });
    hands.sort_by(|lhs, rhs| lhs.0.hand_type.cmp(&rhs.0.hand_type));

    // Compute winnings
    let mut total_winnings = 0;
    hands.into_iter().enumerate().for_each(|(inx, hand)| {
        let rank = inx + 1;
        let bid = hand.1;
        total_winnings += rank * bid;
    });

    println!("Answer: {}", total_winnings);
}
