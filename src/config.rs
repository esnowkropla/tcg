use std::collections::HashMap;

pub struct Config {
    pub deck_a: HashMap<u64, u8>,
    pub deck_b: HashMap<u64, u8>,
}

impl Config {
    pub fn mock() -> Config {
        let mut config = Config {
            deck_a: HashMap::new(),
            deck_b: HashMap::new(),
        };
        config.deck_a.insert(0, 1);
        config.deck_a.insert(1, 1);
        config.deck_a.insert(2, 4);
        config.deck_a.insert(3, 4);

        config.deck_b.insert(0, 4);
        config.deck_b.insert(1, 4);
        config.deck_b.insert(2, 2);
        config.deck_b.insert(3, 1);

        return config;
    }
}
