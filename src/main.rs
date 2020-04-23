use rand::Rng;
use std::io;

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

fn extract_position(input: String) -> Result<usize, String> {
    let input_chars: Vec<char> = input.chars().collect();

    if input_chars.len() < 3 {
        return Err("Invalid position! You moron!".to_string());
    }

    let i = input_chars[0].to_digit(10);
    let j = input_chars[2].to_digit(10);

    if let Some(x) = i {
        if let Some(y) = j {
            if x < 3 && y < 3 {
                return Ok((x * 3 + y) as usize);
            } else {
                return Err("Invalid position! You moron!".to_string());
            }
        } else {
            return Err("Invalid position! You moron!".to_string());
        }
    } else {
        return Err("Invalid position! You moron!".to_string());
    }
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

        let result = extract_position(game_move);
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
