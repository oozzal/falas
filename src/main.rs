use rand::prelude::*;
use std::{collections::HashSet, fmt};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    id: usize,
}

#[derive(Debug, Clone)]
struct Card {
    face: Face,
    value: Value,
}

impl Card {
    fn rank(&self) -> u8 {
        self.value.rank()
    }
}

#[derive(EnumIter, Hash, PartialEq, Eq, Debug, Copy, Clone)]
enum Face {
    Vote,
    Chidi,
    Pane,
    Ita,
}

#[derive(EnumIter, Hash, PartialEq, Eq, Debug, Copy, Clone)]
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
    fn rank(&self) -> u8 {
        *self as u8
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

#[derive(Debug, PartialEq)]
enum Output {
    Badi = 1,
    Joot = 2,
    Color = 3,
    Run = 4,
    DoubleRun = 5,
    Trial = 6,
}

impl Output {
    fn rank(self) -> u8 {
        self as u8
    }
}

impl Hand {
    fn display(&self) {
        for card in &self.cards {
            print!("{}, ", card);
        }
        println!("\n#{} {:?}", self.id, self.identify())
    }

    fn sort(&mut self) {
        self.cards.sort_by(|a, b| a.rank().cmp(&b.rank()));
    }

    fn identify(&self) -> Output {
        let mut output = Output::Badi;
        let faces = self
            .cards
            .iter()
            .map(|card| &card.face)
            .collect::<HashSet<&Face>>();
        if faces.len() == 1 {
            output = Output::Color;
        }
        let mut values: Vec<u8> = self
            .cards
            .iter()
            .map(|card| card.rank())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        values.sort();
        if values.len() == 2 {
            output = Output::Joot;
        }
        if values.len() == 1 {
            output = Output::Trial;
        }
        if values.len() == 3 && (values[2] - values[0] == 2 || values[2] - values[1] == 11) {
            if output == Output::Color {
                output = Output::DoubleRun;
            } else {
                output = Output::Run;
            }
        }
        output
    }

    fn rank(&self) -> u8 {
        self.identify().rank()
    }

    fn compare(&self, other: &Hand) -> Compare {
        let self_rank = self.rank();
        let other_rank = other.rank();
        if self_rank > other_rank {
            return Compare::Greater;
        } else if other_rank > self_rank {
            return Compare::Less;
        }
        for i in (0..3).rev() {
            let self_rank = self.cards[i].rank();
            let other_rank = other.cards[i].rank();
            if self_rank > other_rank {
                return Compare::Greater;
            } else if other_rank > self_rank {
                return Compare::Less;
            }
        }
        Compare::Equal
    }
}

#[derive(Debug, PartialEq)]
enum Compare {
    Less,
    Equal,
    Greater,
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
        for i in 0..count {
            let mut hand = Hand {
                cards: Vec::with_capacity(3),
                id: i,
            };
            for _ in 0..3 {
                hand.cards.push(self.cards.pop().unwrap());
            }
            hand.sort();
            hands.push(hand);
        }
        Ok(hands)
    }
}

struct Game {
    hands: Vec<Hand>,
    total_players: usize,
}

impl Game {
    fn new(total_players: usize) -> Self {
        Game {
            total_players,
            hands: Vec::with_capacity(total_players),
        }
    }

    fn deal(&mut self) {
        self.hands = Deck::new().deal(self.total_players).unwrap();
    }

    fn display(&self) {
        for hand in &self.hands {
            hand.display();
        }
    }

    fn show<'a>(&'a self, hand: &'a Hand) -> &'a Hand {
        let mut winner = hand;
        for i in 0..self.total_players {
            let challenger = &self.hands[i];
            if hand.id == challenger.id {
                continue;
            }
            let result = challenger.compare(winner);
            if result == Compare::Greater || result == Compare::Equal {
                winner = challenger;
            }
        }
        winner
    }
}

fn main() {
    let mut game = Game::new(3);
    game.deal();
    let winner: &Hand = game.show(&game.hands[0]);
    game.display();
    println!("\nWINNER IS:");
    winner.display();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_selects_winner() {
        let ekka = Card {
            face: Face::Vote,
            value: Value::Ekka,
        };
        let dukki = Card {
            face: Face::Vote,
            value: Value::Dukki,
        };
        let tikki = Card {
            face: Face::Vote,
            value: Value::Tikki,
        };
        let hand1 = Hand {
            id: 1,
            cards: vec![ekka.clone(), dukki.clone(), tikki.clone()],
        };
        let hand2 = Hand {
            id: 2,
            cards: vec![ekka, dukki, tikki],
        };
        let game = Game {
            total_players: 2,
            hands: vec![hand1, hand2],
        };
        let winner = game.show(&game.hands[0]);
        assert_eq!(winner.id, game.hands[1].id);
        let winner = game.show(&game.hands[1]);
        assert_eq!(winner.id, game.hands[0].id);
    }

    #[test]
    fn it_identifies_run() {
        let ekka = Card {
            face: Face::Vote,
            value: Value::Ekka,
        };
        let dukki = Card {
            face: Face::Vote,
            value: Value::Dukki,
        };
        let tikki = Card {
            face: Face::Pane,
            value: Value::Tikki,
        };
        let chauka = Card {
            face: Face::Pane,
            value: Value::Chauka,
        };
        let hand_cards = vec![ekka.clone(), dukki.clone(), tikki];
        let hand = Hand {
            cards: hand_cards,
            id: 1,
        };
        assert_eq!(hand.identify() == Output::Run, true);
        let hand_cards = vec![ekka, dukki, chauka];
        let hand = Hand {
            cards: hand_cards,
            id: 2,
        };
        assert_eq!(hand.identify() == Output::Run, false);
    }
}
