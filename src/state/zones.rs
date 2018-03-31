use super::players::Player;

#[derive(Debug, Copy, Clone)]
pub enum Zone {
    Stack(Option<u32>),
    Deck(Player, Option<u32>),
    Field(Player, Option<u32>),
    Discard(Player, Option<u32>),
    Hand(Player, Option<u32>),
    Removed,
    None,
}

impl Zone {
    pub fn player(&self) -> Player {
        match self {
            &Zone::Stack(_) => Player::None,
            &Zone::Deck(player, _) => player,
            &Zone::Field(player, _) => player,
            &Zone::Discard(player, _) => player,
            &Zone::Hand(player, _) => player,
            &Zone::Removed => Player::None,
            &Zone::None => Player::None,
        }
    }
}
