extern crate tcg;

use tcg::Config;
use tcg::GameManager;

fn main() {
    let config = Config::mock();
    let manager = GameManager::init(&config);
    println!("{:?}", manager);
}
