use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy, PartialEq)]
pub enum Player {
    Human,
    Computer,
    Nobody,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Player::Human => write!(f, "X"),
            Player::Computer => write!(f, "O"),
            Player::Nobody => write!(f, " "),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Move {
    pub x: usize,
    pub y: usize,
}

impl Move {
    pub fn new(x: usize, y: usize) -> Result<Move, String> {
        let result_move = Move { x, y };
        if result_move.value() > 8 {
            return Err("Invalid position! You moron!".to_string());
        }

        Ok(result_move)
    }

    pub fn value(&self) -> usize {
        self.y * 3 + self.x
    }
}

pub struct Game {
    pub board: [Player; 9],
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: [Player::Nobody; 9],
        }
    }

    pub fn validate_move(&self, input: &Move) -> bool {
        self.board[input.value()] == Player::Nobody
    }

    pub fn make_move(&mut self, input: Move, player: Player) {
        self.board[input.value()] = player;
    }

    pub fn check_winner(&self) -> Player {
        let board = self.board;
        for base in 0..3 {
            let root = base * 3;
            if board[root] == board[root + 1]
                && board[root] == board[root + 2]
                && board[root] != Player::Nobody
            {
                return board[root];
            }
            if board[base] == board[base + 3]
                && board[base] == board[base + 6]
                && board[base] != Player::Nobody
            {
                return board[base];
            }
        }
        if board[0] == board[4] && board[0] == board[8] && board[0] != Player::Nobody {
            return board[0];
        }
        if board[2] == board[4] && board[2] == board[6] && board[2] != Player::Nobody {
            return board[6];
        }
        Player::Nobody
    }
}
