pub mod score;

use bevy::{math::prelude::*, prelude::*};
use bevy_egui::{EguiContexts, EguiPlugin, UiRenderOrder, egui};
use rand::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: false,
            ui_render_order: UiRenderOrder::EguiAboveBevyUi,
            bindless_mode_array_size: None,
        })
        .add_systems(Startup, setup)
        .init_state::<GamePhase>()
        .insert_resource({
            let mut rng = rand::rng();
            let mut deck = Deck {
                cards: Deck::generate_deck(),
            };
            deck.shuffle(&mut rng);
            let state = GameState {
                deck: deck,
                player_score: 0,
                cpu_score: 0,
                player_hand: Hand { cards: Vec::new() },
                cpu_hand: Hand { cards: Vec::new() },
            };
            state
        })
        .insert_resource(HandTimer(Timer::from_seconds(1., TimerMode::Repeating)))
        .add_systems(Update, player.run_if(in_state(GamePhase::Player)))
        .add_systems(Update, cpu.run_if(in_state(GamePhase::CPU)))
        .add_systems(
            Update,
            display_game_over.run_if(in_state(GamePhase::GameOver)),
        )
        .run();
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, States, Default)]
enum GamePhase {
    #[default]
    Player,
    CPU,
    GameOver,
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
pub struct Card {
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
    player_score: u32,
    cpu_score: u32,
    player_hand: Hand,
    cpu_hand: Hand,
}

#[derive(Resource)]
struct HandTimer(Timer);

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

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn clear_hand(hand_query: &Query<(Entity, &Sprite), With<Card>>, commands: &mut Commands) {
    hand_query
        .iter()
        .for_each(|(entity, _)| commands.entity(entity).despawn());
}

fn player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<GameState>,
    mut contexts: EguiContexts,
    mut game_phase: ResMut<NextState<GamePhase>>,
) -> Result {
    egui::Window::new("Play Options").show(contexts.ctx_mut()?, |ui| {
        ui.label(format!("Player Score: {}", state.player_score));
        ui.label(format!("CPU Score: {}", state.cpu_score));
        if ui.button("Hit").clicked() {
            let card = state.deck.cards.pop().unwrap();
            println!("{} of {}", card.rank.to_string(), card.suit.to_string());
            state.player_hand.cards.push(card.clone());
            state.player_score = score::hand_score(state.player_hand.cards.clone());

            commands.spawn((
                Sprite {
                    image: asset_server.load(card.image.clone()),
                    custom_size: Some(Vec2::new(64., 96.)),
                    ..default()
                },
                Transform::from_xyz(
                    70. * state.player_hand.cards.len() as f32 - 660.,
                    100.,
                    0.,
                ),
                card.clone(),
            ));

            if state.player_score > 21 {
                game_phase.set(GamePhase::GameOver);
            }
        }

        if ui.button("Stay").clicked() {
            game_phase.set(GamePhase::CPU);
        }

    });
    Ok(())
}

fn cpu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<GameState>,
    mut game_phase: ResMut<NextState<GamePhase>>,
    mut timer: ResMut<HandTimer>,
    mut contexts: EguiContexts,
    time: Res<Time>,
) -> Result {
    egui::Window::new("Play Options").show(contexts.ctx_mut()?, |ui| {
        ui.label(format!("Player Score: {}", state.player_score));
        ui.label(format!("CPU Score: {}", state.cpu_score));

        timer.0.tick(time.delta()); //<callout id="first_library_create.pig.timer_delta" />
        if timer.0.just_finished() {
            if state.cpu_score < 16 {
                let card = state.deck.cards.pop().unwrap();
                println!("{} of {}", card.rank.to_string(), card.suit.to_string());
                state.cpu_hand.cards.push(card.clone());
                state.cpu_score = score::hand_score(state.cpu_hand.cards.clone());

                commands.spawn((
                    Sprite {
                        image: asset_server.load(card.image.clone()),
                        custom_size: Some(Vec2::new(64., 96.)),
                        ..default()
                    },
                    Transform::from_xyz(
                        70. * state.cpu_hand.cards.len() as f32 - 660.,
                        0.,
                        0.,
                    ),
                    card.clone(),
                ));

                if state.cpu_score > 21 {
                    game_phase.set(GamePhase::GameOver);
                }
            } else {
                game_phase.set(GamePhase::GameOver);
            }
        }
    });
    Ok(())
}

fn display_game_over(
    mut contexts: EguiContexts,
    mut state: ResMut<GameState>,
    hand_query: Query<(Entity, &Sprite), With<Card>>,
    mut commands: Commands,
    mut game_phase: ResMut<NextState<GamePhase>>,
) -> Result {
    let game_over_text;
    if state.player_score > 21 || state.cpu_score > state.player_score {
        game_over_text = "You lose. :( Play again?".to_string()
    } else if state.cpu_score > 21 || state.player_score > state.cpu_score {
        game_over_text = "You win! Play again?".to_string()
    } else {
        game_over_text = "TIE GAME!? WHAAAAAA? Play again?".to_string()
    }

    egui::Window::new(game_over_text).anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0 ,0.0)).show(contexts.ctx_mut()?, |ui| {
        if ui.button("Play Again").clicked() {
            state.player_hand.cards.clear();
            state.cpu_hand.cards.clear();
            state.player_score = 0;
            state.cpu_score = 0;
            state.deck = Deck {
                cards: Deck::generate_deck(),
            };
            let mut rng = rand::rng();
            state.deck.shuffle(&mut rng);

            clear_hand(&hand_query, &mut commands);
            game_phase.set(GamePhase::Player);
        }
    });
    Ok(())
}
