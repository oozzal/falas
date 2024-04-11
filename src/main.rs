mod card;
mod deck;
mod game;
mod hand;

use game::Game;
use hand::Hand;

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
    use crate::card::{Card, Face, Value};
    use crate::game::Game;

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
        let mut hand1 = Hand::new(1, 3);
        hand1.cards = vec![ekka.clone(), dukki.clone(), tikki.clone()];
        hand1.identify();
        let mut hand2 = Hand::new(2, 3);
        hand2.cards = vec![ekka, dukki, tikki];
        hand2.identify();
        let mut game = Game::new(2);
        game.hands = vec![hand1, hand2];
        let winner = game.show(&game.hands[0]);
        assert_eq!(winner.id, game.hands[1].id);
        let winner = game.show(&game.hands[1]);
        assert_eq!(winner.id, game.hands[0].id);
    }

    #[test]
    fn it_identifies_run() {
        use hand::Output;

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
        let mut hand = Hand::new(1, 3);
        hand.cards = vec![ekka.clone(), dukki.clone(), tikki];
        hand.identify();
        assert_eq!(hand.identity.unwrap() == Output::Run, true);
        let mut hand = Hand::new(2, 3);
        hand.cards = vec![ekka, dukki, chauka];
        hand.identify();
        assert_eq!(hand.identity.unwrap() == Output::Run, false);
    }
}
