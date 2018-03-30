use super::players::Player;

#[derive(Debug, Copy, Clone)]
pub struct Card {
    pub id: u64,
    pub which: u64,
    pub owner: Player,
}
