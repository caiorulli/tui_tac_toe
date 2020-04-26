use std::fmt::{self, Display, Formatter};

pub fn extract_position(input: String) -> Result<usize, String> {
    let input_chars: Vec<char> = input.chars().collect();

    if input_chars.len() < 3 {
        return Err("Invalid position! You moron!".to_string());
    }

    let i = input_chars[0].to_digit(10);
    let j = input_chars[2].to_digit(10);

    if let Some(x) = i {
        if let Some(y) = j {
            if x < 3 && y < 3 {
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

pub fn check_winner(board: [Player; 9]) -> Player {
    if board[0] == board[1] && board[2] == board[1] {
        return board[0];
    }
    Player::Nobody
}
