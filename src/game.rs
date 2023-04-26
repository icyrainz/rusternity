mod board;
mod player;

use crate::game::board::Board;
use crate::game::player::Player;

pub struct Game {
    board: Board,
    players: Vec<Player>,
    current_player: usize,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            players: vec![Player::new("X"), Player::new("O")],
            current_player: 0,
        }
    }

    fn next_player(&mut self) {
        self.current_player = (self.current_player + 1) % self.players.len();
    }

    fn current_player(&self) -> &Player {
        &self.players[self.current_player]
    }

    fn current_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.current_player]
    }

    fn play(&mut self) {
        loop {
            self.next_player();
        }
    }
}
