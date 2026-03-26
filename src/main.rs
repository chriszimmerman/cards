use bevy::{math::prelude::*, prelude::*};
use rand::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource({
            let mut rng = rand::rng();
            let mut deck = Deck {
                cards: Deck::generate_deck(),
            };
            deck.shuffle(&mut rng);
            deck
        })
        .add_systems(Update, mouse_button_input)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Debug, EnumIter, Clone, Display)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, EnumIter, Clone, Display)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Component)]
struct Card {
    suit: Suit,
    rank: Rank,
    image: String,
}

#[derive(Debug, Component, Resource)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn generate_deck() -> Vec<Card> {
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

    fn shuffle(&mut self, rng: &mut impl Rng) {
        self.cards.shuffle(rng);
    }
}

fn get_path(suit: Suit, rank: Rank) -> String {
    let suit: &str = match suit {
        Suit::Diamonds => "diamonds",
        Suit::Clubs => "clubs",
        Suit::Hearts => "hearts",
        Suit::Spades => "spades",
        _ => panic!("Invalid suit"),
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn(
        (Sprite {
            image: asset_server.load("cards/back_1.png"),
            custom_size: Some(Vec2::new(64., 96.)),
            ..default()
        }),
    );
}

fn mouse_button_input(
    buttons: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut deck: ResMut<Deck>,
) {
    if buttons.just_released(MouseButton::Left) {
        let card = deck.cards.pop().unwrap();
        commands.spawn((
            Sprite {
                image: asset_server.load(card.image.clone()),
                custom_size: Some(Vec2::new(64., 96.)),
                ..default()
            },
            Transform::from_xyz(70. * (52. - deck.cards.len() as f32), 0., 0.),
        ));
        println!("{} of {}", card.rank.to_string(), card.suit.to_string());
    }
}
