use rand::Rng;
use std::io;
use tic_tac_toe;

fn print_board(board: [u32; 9]) {
    for i in 0..3 {
        println!("+---+---+---+");
        for j in 0..3 {
            print!("| {} ", board[i * 3 + j],);
        }
        println!("|");
    }
    println!("+---+---+---+");
}

fn main() {
    let mut board: [u32; 9] = [0; 9];
    let mut finished = false;
    println!("Welcome to old woman's game!\n");
    print_board(board);

    while !finished {
        println!("What's your move?");
        let mut game_move = String::new();

        io::stdin()
            .read_line(&mut game_move)
            .expect("Failed to read line!");

        let result = tic_tac_toe::extract_position(game_move);
        match result {
            Ok(position) => board[position] = 1,
            Err(error) => println!("{}", error),
        };

        print_board(board);

        let computer_move_x = rand::thread_rng().gen_range(0, 3);
        let computer_move_y = rand::thread_rng().gen_range(0, 3);
        let computer_position = (computer_move_x * 3 + computer_move_y) as usize;
        board[computer_position] = 2;

        println!("Ok, my turn.");
        print_board(board);

        finished = true;
    }
}
