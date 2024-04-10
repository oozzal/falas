use rand::prelude::*;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
}

#[derive(Debug)]
struct Card {
    face: Face,
    value: Value,
}

#[derive(EnumIter, Debug, Copy, Clone)]
enum Face {
    Vote,
    Chidi,
    Pane,
    Ita,
}

#[derive(EnumIter, Debug, Copy, Clone)]
enum Value {
    Dukki = 2,
    Tikki = 3,
    Chauka = 4,
    Panja = 5,
    Chakka = 6,
    Satta = 7,
    Athha = 8,
    Nahal = 9,
    Dahal = 10,
    Gulam = 11,
    Missi = 12,
    Badshah = 13,
    Ekka = 14,
}

impl Value {
    fn rank(self) -> u8 {
        self as u8
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.value, self.face)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Ekka => write!(f, "A"),
            Value::Badshah => write!(f, "K"),
            Value::Missi => write!(f, "Q"),
            Value::Gulam => write!(f, "J"),
            _ => write!(f, "{}", self.rank()),
        }
    }
}

impl fmt::Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Face::Vote => write!(f, "♠"),
            Face::Chidi => write!(f, "♣"),
            Face::Pane => write!(f, "♥"),
            Face::Ita => write!(f, "♦"),
        }
    }
}

impl Hand {
    fn display(mut self) {
        self.cards
            .sort_by(|a, b| a.value.rank().cmp(&b.value.rank()));
        for card in self.cards {
            print!("{}, ", card)
        }
        println!()
    }
}

impl Deck {
    fn new() -> Self {
        let mut deck = Deck {
            cards: Vec::with_capacity(52),
        };
        for face in Face::iter() {
            for value in Value::iter() {
                deck.cards.push(Card { face, value });
            }
        }
        deck.shuffle();
        deck
    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, count: usize) -> Result<Vec<Hand>, &str> {
        if count > 17 {
            return Err("Maximum players reached");
        }
        let mut hands: Vec<Hand> = Vec::with_capacity(count);
        for _ in 0..count {
            let mut hand = Hand {
                cards: Vec::with_capacity(3),
            };
            for _ in 0..3 {
                hand.cards.push(self.cards.pop().unwrap());
            }
            hands.push(hand);
        }
        Ok(hands)
    }
}

fn main() {
    let mut deck = Deck::new();
    let hands = deck.deal(3).unwrap();
    for hand in hands {
        hand.display()
    }
}
