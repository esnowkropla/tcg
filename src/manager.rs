use super::state::GameState;
use super::action::Action;
use super::config::Config;

pub struct GameManager<'a> {
    a: GameState<'a>,
    b: GameState<'a>,
    current: *mut GameState<'a>,
}

impl<'a> GameManager<'a> {
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
