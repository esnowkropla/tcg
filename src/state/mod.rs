use std::collections::HashMap;

use super::config::Config;
use super::action::Action;

mod cards;
mod zones;
mod players;

pub use self::cards::{Card, Cards};
pub use self::zones::Zone;
pub use self::players::Player;

#[derive(Debug)]
pub struct GameState {
    pub cards: Cards,
}

impl GameState {
    pub fn new(config: &Config) -> GameState {
        let mut state = GameState { cards: Cards::new() };
        state.push_deck(&config.deck_a, Player::A);
        state.push_deck(&config.deck_b, Player::B);

        return state;
    }

    fn push_deck(&mut self, deck: &HashMap<u64, u8>, player: Player) -> () {
        for (card, &num) in deck.iter() {
            for _ in 0..num {
                let len = self.cards.len();
                let card = Card {
                    id: len as u64,
                    which: *card,
                    location: Zone::None,
                    owner: player,
                };
                self.cards.push(Zone::Deck(player, None), card);
            }
        }
    }

    pub fn update(&mut self, other: &GameState, action: &Action) {
        println!("{:?}", other);
        println!("{:?}", action);
    }
}
