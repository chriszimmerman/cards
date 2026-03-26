use bevy::asset::AssetPath;
use bevy::{math::prelude::*, prelude::*};
use rand::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    let mut deck = generate_deck();
    let mut rng = rand::rng();
    deck.shuffle(&mut rng);
    deck.iter().for_each(|card| println!("{:?}", card));

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Debug, EnumIter, Clone)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, EnumIter, Clone)]
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

fn setup(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            image: asset_server.load("cards/back_1.png"),
            ..default()
        },
        Transform::from_translation(Vec3::new(0., 0., 0.)),
    ));
}
