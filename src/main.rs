mod game_system;
mod game_interface;

use crate::game_system::Station;
use rand::thread_rng;

fn main() {
    game_interface::begin_game_loop()
}
