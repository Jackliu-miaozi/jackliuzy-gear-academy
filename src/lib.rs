#![no_std]
use gstd::msg;
use pebbles_game_io::*;

static mut PEBBLE_GAME: Option<GameState> = None;

#[no_mangle]
unsafe extern "C" fn init() {
    
}

#[no_mangle]
unsafe extern "C" fn handle() {
}

#[no_mangle]
unsafe extern "C" fn state() {
    
}
