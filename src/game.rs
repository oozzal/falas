use crate::deck::Deck;
use crate::hand::Hand;
use std::cmp::Ordering;

pub struct Game {
    pub hands: Vec<Hand>,
    total_players: usize,
}

impl Game {
    pub fn new(total_players: usize) -> Self {
        Game {
            total_players,
            hands: Vec::with_capacity(total_players),
        }
    }

    pub fn deal(&mut self) {
        self.hands = Deck::new().deal(self.total_players).unwrap();
    }

    pub fn display(&self) {
        for hand in &self.hands {
            hand.display();
        }
    }

    pub fn show<'a>(&'a self, hand: &'a Hand) -> &'a Hand {
        let mut winner = hand;
        for i in 0..self.total_players {
            let challenger = &self.hands[i];
            if hand.id == challenger.id {
                continue;
            }
            let result = challenger.compare(winner);
            if result == Ordering::Greater || result == Ordering::Equal {
                winner = challenger;
            }
        }
        winner
    }
}
