use rand::Rng;
use termion::clear;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io;
use std::io::Write;

use tic_tac_toe;
use tic_tac_toe::Player;

fn write_board(stdout: &mut termion::raw::RawTerminal<std::io::Stdout>, board: [Player; 9], position: (u16, u16)) {
    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    write!(stdout, "Welcome to old woman's game!\n\rHere you will try to save humankind from being enslaved by a mighty invincible artificial intelligence.\n\rGodspeed!\n\r\n\r\n\r")
        .unwrap();
    for i in 0..3 {
        write!(stdout, "+---+---+---+\n\r").unwrap();
        for j in 0..3 {
            write!(stdout, "| {} ", board[i * 3 + j]).unwrap();
        }
        write!(stdout, "|\n\r").unwrap();
    }
    write!(stdout, "+---+---+---+\n\r").unwrap();
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

fn old_main() {
    let mut board: [Player; 9] = tic_tac_toe::empty_board();
    println!("Welcome to old woman's game!\nHere you will try to save humankind from being slaved by a mighty invincible artificial intelligence.\nGodspeed!\n");
    // print_board(board);

    loop {
        println!("What's your move?");
        let mut game_move = String::new();

        io::stdin()
            .read_line(&mut game_move)
            .expect("Failed to read line!");

        let result = tic_tac_toe::extract_position(board, game_move);
        match result {
            Ok(position) => {
                board[position] = Player::Human;
                // print_board(board);

                board[make_computer_move(board)] = Player::Computer;

                println!("Ok, my turn.");
                // print_board(board);

                match tic_tac_toe::check_winner(board) {
                    Player::Human => {
                        println!("Congratulations! You have saved us from extinction!");
                        break;
                    }
                    Player::Computer => {
                        println!("You lost! Humanity is doomed now...");
                        break;
                    }
                    Player::Nobody => {
                        println!("Nobody won just yet. The battle rages on!");
                    }
                }
            }
            Err(error) => println!("{}", error),
        };
    }
}

fn main() -> Result<(), io::Error> {
    let mut board = tic_tac_toe::empty_board();

    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    let board_position: (u16, u16) = (1, 6);
    let mut position: (u16, u16) = (3, 7);

    write_board(&mut stdout, board, position);

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => {
                write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
                break;
            },
            Key::Char('l') => position.0 += 4,
            Key::Char('h') => position.0 -= 4,
            Key::Char('j') => position.1 += 2,
            Key::Char('k') => position.1 -= 2,
            Key::Char(' ') => {
                let position_x = position.0 - board_position.0;
                let position_y = position.1 - board_position.1;
                let result: usize = (position_x * 3 + position_y) as usize;
                board[result] = Player::Human;

            },
            _ => println!("General Kenobi!"),
        }

        write_board(&mut stdout, board, position);
    }

    Ok(())
}
