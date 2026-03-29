use bevy::log::tracing_subscriber::fmt;
use bevy::{math::prelude::*, prelude::*};
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};
use rand::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .insert_resource({
            let mut rng = rand::rng();
            let mut deck = Deck {
                cards: Deck::generate_deck(),
            };
            deck.shuffle(&mut rng);
            let state = GameState {
                deck: deck,
                score: 0,
                hand: Hand { cards: Vec::new() },
            };
            state
        })
        .add_systems(Update, mouse_button_input)
        .add_systems(EguiPrimaryContextPass, display_score)
        .add_systems(Startup, setup)
        .run();
}

fn display_score(game_state: Res<GameState>, mut contexts: EguiContexts) -> Result {
    egui::Window::new("Blackjack").show(contexts.ctx_mut()?, |ui| {
        ui.label(format!("Score: {}", game_state.score));
    });
    Ok(())
}

#[derive(Debug, EnumIter, Clone, Display)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, EnumIter, Clone, Display, Ord, Eq, PartialOrd, PartialEq)]
enum Rank {
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
struct Card {
    suit: Suit,
    rank: Rank,
    image: String,
}

#[derive(Debug, Component, Resource)]
struct Deck {
    cards: Vec<Card>,
}

#[derive(Debug, Component, Resource)]
struct Hand {
    cards: Vec<Card>,
}
#[derive(Debug, Component, Resource)]
struct GameState {
    deck: Deck,
    score: u32,
    hand: Hand,
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
        },
         Transform::from_xyz(-580., 200., 0.),
        ),
    );
}

fn clear_hand(hand_query: &Query<(Entity, &Sprite), With<Card>>, commands: &mut Commands) {
    hand_query
        .iter()
        .for_each(|(entity, _)| commands.entity(entity).despawn());
}

fn mouse_button_input(
    hand_query: Query<(Entity, &Sprite), With<Card>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<GameState>,
) {
    if buttons.just_released(MouseButton::Left) {
        let card = state.deck.cards.pop().unwrap();
        println!("{} of {}", card.rank.to_string(), card.suit.to_string());
        state.hand.cards.push(card.clone());
        state.score = hand_score(state.hand.cards.clone());

        if state.score > 21 {
            println!("Bust!");
            state.hand.cards.clear();
            state.score = 0;
            state.deck = Deck {
                cards: Deck::generate_deck(),
            };
            let mut rng = rand::rng();
            state.deck.shuffle(&mut rng);
            clear_hand(&hand_query, &mut commands)
        } else {
            commands.spawn((
                Sprite {
                    image: asset_server.load(card.image.clone()),
                    custom_size: Some(Vec2::new(64., 96.)),
                    ..default()
                },
                Transform::from_xyz(70. * (52. - state.deck.cards.len() as f32) - 580., 200., 0.),
                card.clone(),
            ));
        }
        println!("Current score is {}", state.score);
    }
}

fn hand_score(mut hand: Vec<Card>) -> u32 {
    let mut score = 0;
    hand.sort_by(|a, b| a.rank.cmp(&b.rank).reverse());
    for card in hand.iter() {
        let card_score = match card.rank {
            Rank::Jack => 10,
            Rank::Queen => 10,
            Rank::King => 10,
            Rank::Ten => 10,
            Rank::Nine => 9,
            Rank::Eight => 8,
            Rank::Seven => 7,
            Rank::Six => 6,
            Rank::Five => 5,
            Rank::Four => 4,
            Rank::Three => 3,
            Rank::Two => 2,
            Rank::Ace => calculate_ace(score),
        };
        score += card_score;
    }

    score
}

fn calculate_ace(score: u32) -> u32 {
    if score > 10 { 1 } else { 11 }
}
