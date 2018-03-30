use std::ops::{Index, IndexMut};

use super::cards::Card;
use super::players::Player;

pub struct Buffer([Option<Card>; 64]);

impl Buffer {
    pub fn new() -> Buffer {
        Buffer([None; 64])
    }

    pub fn len(&self) -> Result<usize, &'static str> {
        for i in 0..64 {
            if let None = self.0[i] {
                return Ok(i);
            }
        }
        return Err("Card buffer ran out of room");
    }
}

#[derive(Debug)]
pub enum Zone<'a> {
    Owned([Vec<&'a Card>; 2]),
    Neutral(Vec<&'a Card>),
}

impl<'a> Zone<'a> {
    pub fn new_owned() -> Zone<'a> {
        Zone::Owned([Vec::new(), Vec::new()])
    }

    pub fn new_neutral() -> Zone<'a> {
        Zone::Neutral(Vec::new())
    }
}

impl<'a> Index<Player> for Zone<'a> {
    type Output = Vec<&'a Card>;

    fn index(&self, player: Player) -> &Vec<&'a Card> {
        match self {
            &Zone::Neutral(ref vec) => vec,
            &Zone::Owned(ref slice) => {
                match player {
                    Player::A => &slice[0],
                    Player::B => &slice[1],
                }
            }
        }
    }
}

impl<'a> IndexMut<Player> for Zone<'a> {
    fn index_mut(&mut self, player: Player) -> &mut Vec<&'a Card> {
        match self {
            &mut Zone::Neutral(ref mut vec) => vec,
            &mut Zone::Owned(ref mut slice) => {
                match player {
                    Player::A => &mut slice[0],
                    Player::B => &mut slice[1],
                }
            }
        }
    }
}
