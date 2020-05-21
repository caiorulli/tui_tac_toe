use rand::Rng;
use std::io;
use std::io::Write;
use termion::clear;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

use tic_tac_toe;
use tic_tac_toe::Player;

fn write_board(
    stdout: &mut RawTerminal<std::io::Stdout>,
    board: [Player; 9],
    position: (u16, u16),
    winner_status: &str,
) {
    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    write!(stdout, "Welcome to old woman's game!\n\rHere you will try to save humankind from being enslaved by a\n\rmighty invincible artificial intelligence.\n\rGodspeed!\n\r\n\r\n\r")
        .unwrap();
    for i in 0..3 {
        write!(stdout, "+---+---+---+\n\r").unwrap();
        for j in 0..3 {
            write!(stdout, "| {} ", board[i * 3 + j]).unwrap();
        }
        write!(stdout, "|\n\r").unwrap();
    }
    write!(stdout, "+---+---+---+\n\r\n\r\n\r{}", winner_status).unwrap();
    write!(stdout, "{}", cursor::Goto(position.0, position.1)).unwrap();

    stdout.flush().unwrap();
}

fn make_computer_move(board: [Player; 9]) -> usize {
    loop {
        let computer_move_x = rand::thread_rng().gen_range(0, 3);
        let computer_move_y = rand::thread_rng().gen_range(0, 3);
        let result = computer_move_x * 3 + computer_move_y;
        if board[result] == Player::Nobody {
            break result;
        }
    }
}

fn main() -> Result<(), io::Error> {
    let mut board = tic_tac_toe::empty_board();

    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    let board_position: (u16, u16) = (3, 8);
    let mut position: (u16, u16) = (board_position.0, board_position.1);
    let mut winner_status = "";

    write_board(&mut stdout, board, position, winner_status);

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => {
                write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
                break;
            }
            Key::Char('l') => {
                if (position.0 - board_position.0) / 4 < 2 {
                    position.0 += 4;
                }
            }
            Key::Char('h') => {
                if (position.0 - board_position.0) / 4 > 0 {
                    position.0 -= 4;
                }
            }
            Key::Char('j') => {
                if (position.1 - board_position.1) / 2 < 2 {
                    position.1 += 2;
                }
            }
            Key::Char('k') => {
                if (position.1 - board_position.1) / 2 > 0 {
                    position.1 -= 2;
                }
            }
            Key::Char(' ') => {
                let position_x = (position.0 - board_position.0) / 4;
                let position_y = (position.1 - board_position.1) / 2;
                let result: usize = (position_y * 3 + position_x) as usize;

                if board[result] == Player::Nobody {
                    board[result] = Player::Human;
                    board[make_computer_move(board)] = Player::Computer;

                    match tic_tac_toe::check_winner(board) {
                        Player::Human => {
                            winner_status = "Congratulations! You have saved us from extinction!";
                            write_board(&mut stdout, board, position, winner_status);
                            write!(stdout, "{}", cursor::Goto(1, 17)).unwrap();
                            break;
                        }
                        Player::Computer => {
                            winner_status = "You lost! Humanity is doomed now...";
                            write_board(&mut stdout, board, position, winner_status);
                            write!(stdout, "{}", cursor::Goto(1, 17)).unwrap();
                            break;
                        }
                        Player::Nobody => {
                            winner_status = "Nobody won just yet. The battle rages on!";
                        }
                    }
                }
            }
            _ => println!("General Kenobi!"),
        }

        write_board(&mut stdout, board, position, winner_status);
    }

    Ok(())
}
