const STARTING_HEALTH: i32 = 25;
const MAX_HEALTH: i32 = 999;
const MAX_HAND_SIZE: u8 = 12;

pub struct Player {
    name: String,
    health: i32,
    hand: Vec<Card>,
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            health: STARTING_HEALTH,
        }
    }

    fn say_hello(&self) {
        println!("Hello, I am {}", self.name);
    }
}
