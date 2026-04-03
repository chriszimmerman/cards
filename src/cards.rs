use bevy::prelude::*;
use bevy::prelude::{Component, Resource};
use rand::Rng;
use rand::prelude::SliceRandom;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Debug, EnumIter, Clone, Display)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, EnumIter, Clone, Display, Ord, Eq, PartialOrd, PartialEq)]
pub enum Rank {
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Ace = 1,
}

#[derive(Debug, Component, Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
    pub image: String,
}

#[derive(Debug, Component, Resource)]
pub struct Deck {
    pub cards: Vec<Card>,
}

#[derive(Debug, Component, Resource)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn generate_deck() -> Vec<Card> {
        let mut deck = Vec::new();
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                deck.push(Card {
                    suit: suit.clone(),
                    rank: rank.clone(),
                    image: get_path(suit.clone(), rank.clone()),
                })
            }
        }
        deck.into_iter().collect()
    }

    pub fn shuffle(&mut self, rng: &mut impl Rng) {
        self.cards.shuffle(rng);
    }
}

fn get_path(suit: Suit, rank: Rank) -> String {
    let suit: &str = match suit {
        Suit::Diamonds => "diamonds",
        Suit::Clubs => "clubs",
        Suit::Hearts => "hearts",
        Suit::Spades => "spades",
    };

    let rank: &str = match rank {
        Rank::Ace => "1",
        Rank::Two => "2",
        Rank::Three => "3",
        Rank::Four => "4",
        Rank::Five => "5",
        Rank::Six => "6",
        Rank::Seven => "7",
        Rank::Eight => "8",
        Rank::Nine => "9",
        Rank::Ten => "10",
        Rank::Jack => "11",
        Rank::Queen => "12",
        Rank::King => "13",
    };

    format!("cards/{}_{}.png", suit, rank)
}
