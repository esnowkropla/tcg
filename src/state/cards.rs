use std::mem::discriminant;

use super::zones::Zone;
use super::players::Player;

#[derive(Debug, Copy, Clone)]
pub struct Card {
    pub id: u64,
    pub which: u64,
    pub location: Zone,
    pub owner: Player,
}

impl Card {
    pub fn matches(&self, location: Zone) -> bool {
        discriminant(&self.location) == discriminant(&location) &&
            self.location.player() == location.player()
    }
}

#[derive(Debug)]
pub struct Cards(Vec<Card>);

impl Cards {
    pub fn new() -> Cards {
        Cards(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&self, location: Zone, mut card: Card) {
        match location {
            Zone::Stack(_) => {
                card.location = Zone::Stack(Some(self.count(Zone::Stack(None)) as u32))
            }
            Zone::Deck(player, _) => {
                card.location =
                    Zone::Deck(player, Some(self.count(Zone::Deck(player, None)) as u32))
            }
            _ => {}
            /*Zone::Field(player, _),
            Zone::Discard(player, _),
            Zone::Hand(player, _),
            Zone::Removed,*/
        };
    }

    pub fn count(&self, location: Zone) -> usize {
        self.0.iter().filter(|x| x.matches(location)).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        let mut cards = Cards::new();
        cards.0.push(Card {
            id: 0,
            which: 0,
            location: Zone::Deck(Player::A, 0),
            owner: Player::A,
        });
        cards.0.push(Card {
            id: 1,
            which: 0,
            location: Zone::Deck(Player::A, 1),
            owner: Player::A,
        });
        cards.0.push(Card {
            id: 2,
            which: 0,
            location: Zone::Field(Player::A, 0),
            owner: Player::A,
        });
        cards.0.push(Card {
            id: 3,
            which: 0,
            location: Zone::Stack(0),
            owner: Player::A,
        });
        cards.0.push(Card {
            id: 4,
            which: 0,
            location: Zone::Stack(1),
            owner: Player::B,
        });

        assert_eq!(cards.count(Zone::Deck(Player::A, 0)), 2);
        assert_eq!(cards.count(Zone::Deck(Player::B, 0)), 0);
        assert_eq!(cards.count(Zone::Field(Player::A, 0)), 1);
        assert_eq!(cards.count(Zone::Stack(0)), 2);
    }
}
