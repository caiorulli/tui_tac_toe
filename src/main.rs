use rand::Rng;
use std::io;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;

use tic_tac_toe;
use tic_tac_toe::Player;

fn print_board(board: [Player; 9]) {
    for i in 0..3 {
        println!("+---+---+---+");
        for j in 0..3 {
            print!("| {} ", board[i * 3 + j]);
        }
        println!("|");
    }
    println!("+---+---+---+");
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
    print_board(board);

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
                print_board(board);

                board[make_computer_move(board)] = Player::Computer;

                println!("Ok, my turn.");
                print_board(board);

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
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|mut f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(f.size());
        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(block, chunks[0]);
        let block = Block::default().title("Block 2").borders(Borders::ALL);
        f.render_widget(block, chunks[1]);
    })
}
