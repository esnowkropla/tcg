use std::collections::HashMap;

use super::config::Config;
use super::action::Action;

mod cards;
mod zones;
mod players;

pub use self::cards::Card;
pub use self::zones::{Zone, Buffer};
pub use self::players::Player;

pub struct GameState<'a> {
    pub cards: Buffer,

    //Zones
    pub stack: Zone<'a>,
    pub deck: Zone<'a>,
    /* pub field: Zone<'a>,
    pub discard: Zone<'a>,
    pub hand: Zone<'a>,
    pub removed: Zone<'a>,*/
}

impl<'a> GameState<'a> {
    pub fn new(config: &Config) -> GameState {
        let mut state = GameState {
            cards: Buffer::new(),
            stack: Zone::new_neutral(),
            deck: Zone::new_owned(),
        };
        state.push_deck(&config.deck_a, Player::A);
        state.push_deck(&config.deck_b, Player::B);

        return state;
    }

    fn push_deck(
        &mut self,
        deck: &HashMap<u64, u8>,
        player: Player,
    ) -> Result<usize, &'static str> {
        let mut i = 0;
        for (card, &num) in deck.iter() {
            for _ in 0..num {
                let len = self.cards.len()?;
                let card = Card {
                    id: len,
                    which: *card,
                    owner: player,
                };
                self.cards.push(card);
                //self.deck[player].push(self.cards.get(self.cards.len()));
                i += 1;
            }
        }
        return Ok(i);
    }

    pub fn update(&mut self, other: &GameState, action: &Action) {}
}
