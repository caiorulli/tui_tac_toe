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

pub fn empty_board() -> [Player; 9] {
    [Player::Nobody; 9]
}

pub fn extract_position(board: [Player; 9], input: String) -> Result<usize, String> {
    let input_chars: Vec<char> = input.chars().collect();

    if input_chars.len() < 3 {
        return Err("Invalid position! You moron!".to_string());
    }

    let i = input_chars[0].to_digit(10);
    let j = input_chars[2].to_digit(10);

    if let Some(x) = i {
        if let Some(y) = j {
            if x < 3 && y < 3 && board[(x * 3 + y) as usize] == Player::Nobody {
                Ok((x * 3 + y) as usize)
            } else {
                Err("Invalid position! You moron!".to_string())
            }
        } else {
            Err("Invalid position! You moron!".to_string())
        }
    } else {
        Err("Invalid position! You moron!".to_string())
    }
}

pub fn check_winner(board: [Player; 9]) -> Player {
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
