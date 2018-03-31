use super::state::GameState;
use super::action::Action;
use super::config::Config;

#[derive(Debug)]
pub struct GameManager {
    a: GameState,
    b: GameState,
    current: *mut GameState,
}

impl GameManager {
    pub fn init(config: &Config) -> GameManager {
        let mut g = GameManager {
            a: GameState::new(config),
            b: GameState::new(config),
            current: 0 as *mut GameState,
        };
        g.current = &mut g.a as *mut GameState;
        return g;
    }

    pub fn get_state(&self) -> &GameState {
        unsafe { &*self.current }
    }

    fn swap(&mut self) {
        let a = &self.a as *const GameState as usize;
        let b = &self.b as *const GameState as usize;
        self.current = (a ^ b ^ self.current as usize) as *mut GameState;
    }

    pub fn next_state(
        &mut self,
        old_state: &GameState,
        new_state: &mut GameState,
        action: &Action,
    ) -> () {
        new_state.update(old_state, action);
        self.swap();
    }
}
