#![no_std]

use gmeta::{InOut, In, Out, Metadata};
use gstd::prelude::*;

pub struct PebblesMetadata;

impl Metadata for PebblesMetadata {
    type Init = In<PebblesInit>;
    type Handle = InOut<PebblesAction, PebblesEvent>;
    type State = Out<GameState>;
    type Reply = ();
    type Others = ();
    type Signal = ();
}


#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct PebblesInit {
    pub difficulty: DifficultyLevel,
    pub pebbles_count: u8,
    pub max_pebbles_per_turn: u8,
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub enum DifficultyLevel {
    #[default]
    Easy,
    Hard,
}
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum PebblesAction {
    Turn(u8),
    GiveUp,
    Restart {
        difficulty: DifficultyLevel,
        pebbles_count: u8,
        max_pebbles_per_turn: u8,
    },
}
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum PebblesEvent {
    CounterTurn(u8),
    Won(Player),
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub enum Player {
    #[default]
    User,
    Program,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum StateQuery {
    All,
    Winner,
    CounterTurn,
    PebblesRemaining,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum StateReply {
    All(GameState),
    Winner(Option<Player>),
    CounterTurn(u8),
    PebblesRemaining(u8),
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct GameState {
    pub pebbles_count: u8,
    pub max_pebbles_per_turn: u8,
    pub pebbles_remaining: u8,
    pub difficulty: DifficultyLevel,
    pub first_player: Player,
    pub winner: Option<Player>,
}
