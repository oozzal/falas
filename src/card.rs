use std::fmt;
use strum_macros::EnumIter;

#[derive(Debug, Clone)]
pub struct Card {
    pub face: Face,
    pub value: Value,
}

impl Card {
    pub fn rank(&self) -> u8 {
        self.value.rank()
    }
}

#[derive(EnumIter, Hash, PartialEq, Eq, Debug, Copy, Clone)]
pub enum Face {
    Vote,
    Chidi,
    Pane,
    Ita,
}

#[derive(EnumIter, Hash, PartialEq, Eq, Debug, Copy, Clone)]
pub enum Value {
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
            Face::Vote => write!(f, "♠️"),
            Face::Chidi => write!(f, "♣️"),
            Face::Pane => write!(f, "❤️️"),
            Face::Ita => write!(f, "♦️"),
        }
    }
}
