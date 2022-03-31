use rand::Rng;
use std::io;
use std::{io::Write, sync::mpsc, thread};
use termion::clear;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

use tui_tac_toe;
use tui_tac_toe::{Game, Move, Player, Winner};

const WELCOME_MESSAGE: &str = "Welcome to old woman's game!\n\rHere you will try to save humankind from being enslaved by a\n\rmighty invincible artificial intelligence.\n\rGodspeed!\n\r\n\r\n\r";
const PLAYER_WIN_MESSAGE: &str = "Congratulations! You have saved us from extinction!";
const AI_WIN_MESSAGE: &str = "You lost! Humanity is doomed now...";
const DRAW_MESSAGE: &str = "It's a draw! You live to fight another day.";
const NO_WIN_MESSAGE: &str = "Nobody won just yet. The battle rages on!";

fn write_board(
    stdout: &mut RawTerminal<std::io::Stdout>,
    board: [Player; 9],
    position: (u16, u16),
    winner_status: &str,
) {
    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    write!(stdout, "{}", WELCOME_MESSAGE).unwrap();
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

fn make_computer_move(game: &Game) -> Move {
    match tui_tac_toe::minimax(game) {
        (_, Some(best_move)) => best_move,
        _ => loop {
            let computer_move_x = rand::thread_rng().gen_range(0, 3);
            let computer_move_y = rand::thread_rng().gen_range(0, 3);
            let result = (computer_move_x as usize, computer_move_y as usize);
            if game.is_move_valid(&result) {
                break result;
            }
        },
    }
}

fn main() -> Result<(), io::Error> {
    let mut game = Game::new(Player::Human);

    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode()?;

    let board_position: (u16, u16) = (3, 8);
    let mut position: (u16, u16) = (board_position.0, board_position.1);
    let mut winner_status = "";

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for c in stdin.keys() {
            tx.send(c.unwrap()).unwrap();
        }
    });

    while game.check_winner() == Winner::Nobody {
        write_board(&mut stdout, game.build_board(), position, winner_status);

        if game.turn == Player::Human {
            let input_char = rx.recv().unwrap();

            match input_char {
                Key::Char('q') => {
                    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1))?;
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
                    let result_move = (position_x as usize, position_y as usize);

                    if game.is_move_valid(&result_move) {
                        game.apply_move(result_move);
                    }
                }
                _ => {}
            }
        } else {
            game.apply_move(make_computer_move(&game));
        }

        match game.check_winner() {
            Winner::Human => {
                winner_status = PLAYER_WIN_MESSAGE;
                write_board(&mut stdout, game.build_board(), position, winner_status);
                write!(stdout, "{}", cursor::Goto(1, 17))?
            }
            Winner::Computer => {
                winner_status = AI_WIN_MESSAGE;
                write_board(&mut stdout, game.build_board(), position, winner_status);
                write!(stdout, "{}", cursor::Goto(1, 17))?
            }
            Winner::Draw => {
                winner_status = DRAW_MESSAGE;
                write_board(&mut stdout, game.build_board(), position, winner_status);
                write!(stdout, "{}", cursor::Goto(1, 17))?
            }
            Winner::Nobody => {
                winner_status = NO_WIN_MESSAGE;
            }
        }
    }

    Ok(())
}
