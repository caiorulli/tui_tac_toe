use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Player {
    Human,
    Computer,
    Nobody,
}

impl Player {
    fn next(&self) -> Player {
        match self {
            Player::Human => Player::Computer,
            Player::Computer => Player::Human,
            Player::Nobody => Player::Nobody,
        }
    }


    fn to_winner(&self) -> Winner {
        match self {
            Player::Human => Winner::Human,
            Player::Computer => Winner::Computer,
            Player::Nobody => Winner::Nobody,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Winner {
    Nobody,
    Human,
    Computer,
    Draw,
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

pub type Move = (usize, usize);

pub struct Game {
    moves: Vec<Move>,
}

impl Game {
    pub fn new() -> Game {
        Game { moves: Vec::with_capacity(9) }
    }

    pub fn is_move_valid(&self, input: &Move) -> bool {
        !self.moves.contains(input)
    }

    pub fn apply_move(&mut self, input: Move) {
        self.moves.push(input)
    }

    pub fn build_board(&self) -> [Player; 9] {
        let mut board = [Player::Nobody; 9];
        let mut player = Player::Human;
        for (x, y) in self.moves.iter() {
            board[y * 3 + x] = player;
            player = player.next();
        }
        board
    }

    pub fn has_ended(&self) -> bool {
        self.moves.len() == 9
    }

    pub fn check_winner(&self) -> Winner {
        let board = self.build_board();
        for base in 0..3 {
            let root = base * 3;
            if board[root] == board[root + 1]
                && board[root] == board[root + 2]
                && board[root] != Player::Nobody
            {
                return board[root].to_winner();
            }
            if board[base] == board[base + 3]
                && board[base] == board[base + 6]
                && board[base] != Player::Nobody
            {
                return board[base].to_winner();
            }
        }
        if board[0] == board[4] && board[0] == board[8] && board[0] != Player::Nobody {
            return board[0].to_winner();
        }
        if board[2] == board[4] && board[2] == board[6] && board[2] != Player::Nobody {
            return board[6].to_winner();
        }
        if self.has_ended() {
            return Winner::Draw
        }
        Winner::Nobody
    }
}
