extern crate tcg;
extern crate uuid;

use uuid::Uuid;

use tcg::state::Card;
use tcg::state::Player;
use tcg::state::Zone;

fn main() {
    let card = Card {
        id: 0,
        which: Uuid::new_v4(),
        owner: Player::A,
    };
    println!("{:?}", card);
}
