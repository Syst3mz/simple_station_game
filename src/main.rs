mod game_system;

use crate::game_system::Station;


fn main() {
    let station = Station::default();
    println!("{}", station.to_string());
}
