use crate::card::{Card, Face};
use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Output {
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

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub id: usize,
    pub identity: Option<Output>,
}

impl Hand {
    pub fn new(id: usize, capacity: usize) -> Self {
        Hand {
            id,
            cards: Vec::with_capacity(capacity),
            identity: None,
        }
    }

    pub fn display(&self) {
        print!("#{} {:?}  ", self.id, self.identity.unwrap());
        for card in &self.cards {
            print!("{}  ", card);
        }
        println!()
    }

    pub fn sort(&mut self) {
        self.cards.sort_by_key(|card| card.rank());
    }

    pub fn identify(&mut self) {
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
        self.identity = Some(output);
    }

    fn rank(&self) -> u8 {
        self.identity.unwrap().rank()
    }

    pub fn compare(&self, other: &Hand) -> Ordering {
        let other_rank = other.rank();
        let self_rank = self.rank();
        match self_rank.cmp(&other_rank) {
            Ordering::Equal => {
                for i in (0..3).rev() {
                    let self_rank = self.cards[i].rank();
                    let other_rank = other.cards[i].rank();
                    match self_rank.cmp(&other_rank) {
                        Ordering::Equal => continue,
                        ordering => return ordering,
                    }
                }
            }
            // greater and less
            ordering => return ordering,
        }
        Ordering::Equal
    }
}
