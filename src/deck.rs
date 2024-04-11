use crate::card::{Card, Face, Value};
use crate::hand::Hand;
use rand::prelude::*;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
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

    pub fn deal(&mut self, count: usize) -> Result<Vec<Hand>, &str> {
        if count > 17 {
            return Err("Maximum players reached");
        }
        let mut hands: Vec<Hand> = Vec::with_capacity(count);
        for i in 0..count {
            let mut hand = Hand::new(i, 3);
            for _ in 0..3 {
                hand.cards.push(self.cards.pop().unwrap());
            }
            hand.sort();
            hand.identify();
            hands.push(hand);
        }
        Ok(hands)
    }
}
