#![no_std]
use core::array;
use gcore::exec;
use gstd::msg;
use pebbles_game_io::*;

static mut PEBBLE_GAME: Option<GameState> = None;

#[no_mangle]
unsafe extern "C" fn init() {
    // Load the initialization message
    let init: PebblesInit = msg::load().expect("Unable to decode InitPeble");

    // Create a new game state with the provided initialization values
    let pebbles_game = GameState {
        pebbles_count: init.pebbles_count,
        max_pebbles_per_turn: init.max_pebbles_per_turn,
        difficulty: init.difficulty,
        pebbles_remaining: init.pebbles_count,
        ..Default::default()
    };

    // Store the game state
    PEBBLE_GAME = Some(pebbles_game);
}

#[no_mangle]
unsafe extern "C" fn handle() {
    // Load the action message
    let action: PebblesAction = msg::load().expect("Unable to decode PebblesAction");

    // Initialize seed variables
    let mut a_seed = [0u8; 32];
    let mut b_seed = [0u8; 32];

    // Get a mutable reference to the game state
    let pebble_game = PEBBLE_GAME
        .as_mut()
        .expect("Unexpected error in taking state");

    // Generate a valid seed for player A
    while !(1 <= a_seed[1] && a_seed[1] < pebble_game.max_pebbles_per_turn) {
        let subject: [u8; 32] = array::from_fn(|i| i as u8 + 1);
        let (new_a_seed, _block_number) = exec::random(subject).expect("Error in random");
        a_seed = new_a_seed;
    }

    // Generate a valid seed for player B
    while !(1 <= b_seed[1] && b_seed[1] < pebble_game.pebbles_remaining) {
        let subject: [u8; 32] = array::from_fn(|i| i as u8 + 1);
        let (new_b_seed, _block_number) = exec::random(subject).expect("Error in random");
        b_seed = new_b_seed;
    }

    // Handle the action based on its type
    match action {
        PebblesAction::Turn(pebbles) => {
            match pebble_game.difficulty {
                DifficultyLevel::Easy => {
                    // Update the game state for an easy difficulty turn
                    pebble_game.pebbles_remaining -= pebbles;
                    if pebble_game.pebbles_remaining == 0 {
                        // If there are no pebbles remaining, the user wins
                        msg::reply(PebblesEvent::Won(Player::User), 0).expect("Error in a reply");
                        pebble_game.winner = Some(Player::User);
                    } else {
                        // Increment the counter turn and send a reply
                        pebble_game.counter_turn += 1;
                        msg::reply(PebblesEvent::CounterTurn(pebble_game.counter_turn), 0)
                            .expect("Error in a reply");

                        // Subtract pebbles based on the seed value
                        if pebble_game.pebbles_remaining >= pebble_game.max_pebbles_per_turn {
                            pebble_game.pebbles_remaining -= a_seed[1];
                        } else {
                            pebble_game.pebbles_remaining -= b_seed[1];
                        }

                        // Check if there are no pebbles remaining after the turn
                        if pebble_game.pebbles_remaining == 0 {
                            // Increment the counter turn and send a reply
                            pebble_game.counter_turn += 1;
                            msg::reply(PebblesEvent::CounterTurn(pebble_game.counter_turn), 0)
                                .expect("Error in a reply");

                            // If there are no pebbles remaining, the program wins
                            msg::reply(PebblesEvent::Won(Player::Program), 0)
                                .expect("Error in a reply");
                        } else {
                            // Increment the counter turn and send a reply
                            pebble_game.counter_turn += 1;
                            msg::reply(PebblesEvent::CounterTurn(pebble_game.counter_turn), 0)
                                .expect("Error in a reply");
                        }
                    }
                }
                _ => {
                    // Update the game state for a normal difficulty turn
                    pebble_game.pebbles_remaining -= pebbles;
                    if pebble_game.pebbles_remaining == 0 {
                        // If there are no pebbles remaining, the user wins
                        msg::reply(PebblesEvent::Won(Player::User), 0).expect("Error in a reply");
                        pebble_game.winner = Some(Player::User);
                    } else {
                        // Subtract pebbles based on the game rules
                        if pebble_game.pebbles_remaining >= pebble_game.max_pebbles_per_turn
                            && (pebble_game.pebbles_remaining % pebble_game.max_pebbles_per_turn
                                != 0 & 1)
                        {
                            pebble_game.pebbles_remaining -= pebble_game.pebbles_remaining
                                % pebble_game.max_pebbles_per_turn
                                - 1;
                            pebble_game.counter_turn += 1;
                            msg::reply(PebblesEvent::CounterTurn(pebble_game.counter_turn), 0)
                                .expect("Error in a reply");
                        }
                        if pebble_game.pebbles_remaining >= pebble_game.max_pebbles_per_turn
                            && (pebble_game.pebbles_remaining % pebble_game.max_pebbles_per_turn
                                == 1)
                        {
                            pebble_game.pebbles_remaining -= a_seed[1];
                            pebble_game.counter_turn += 1;
                            msg::reply(PebblesEvent::CounterTurn(pebble_game.counter_turn), 0)
                                .expect("Error in a reply");
                        }
                        if 0 < pebble_game.pebbles_remaining
                            && pebble_game.pebbles_remaining <= pebble_game.max_pebbles_per_turn
                        {
                            // If there are no pebbles remaining, the program wins
                            pebble_game.counter_turn += 1;
                            msg::reply(PebblesEvent::CounterTurn(pebble_game.counter_turn), 0)
                                .expect("Error in a reply");
                            msg::reply(PebblesEvent::Won(Player::Program), 0)
                                .expect("Error in a reply");
                            pebble_game.winner = Some(Player::Program);
                        } else {
                            panic!("Unexpected error");
                        }
                    }
                }
            }
        }
        PebblesAction::GiveUp => {
            // If the user gives up, the program wins
            msg::reply(PebblesEvent::Won(Player::Program), 0).expect("Error in a reply");
            pebble_game.winner = Some(Player::Program);
        }
        PebblesAction::Restart {
            difficulty,
            pebbles_count,
            max_pebbles_per_turn,
        } => {
            // Restart the game with the provided values
            let _pebble_game = GameState {
                pebbles_count,
                max_pebbles_per_turn,
                difficulty,
                pebbles_remaining: pebbles_count,
                ..Default::default()
            };
        }
    }
}

#[no_mangle]
unsafe extern "C" fn state() {
    // Take ownership of the game state
    let pebble_game = PEBBLE_GAME
        .take()
        .expect("Unexpected error in taking state");

    // Load the state query
    let query: StateQuery = msg::load().expect("Unable to load the state query");

    // Handle the state query based on its type
    match query {
        StateQuery::All => {
            // Reply with the entire game state
            msg::reply(StateReply::All(pebble_game), 0).expect("Error in a reply");
        }
        StateQuery::Winner => {
            // Reply with the winner of the game
            msg::reply(StateReply::Winner(pebble_game.winner), 0).expect("Error in a reply");
        }
        StateQuery::CounterTurn => {
            // Reply with the counter turn value
            msg::reply(StateReply::CounterTurn(pebble_game.counter_turn), 0)
                .expect("Error in a reply");
        }
        StateQuery::PebblesRemaining => {
            // Reply with the number of pebbles remaining
            msg::reply(
                StateReply::PebblesRemaining(pebble_game.pebbles_remaining),
                0,
            )
            .expect("Error in a reply");
        }
    }
}
