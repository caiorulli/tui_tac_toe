use tic_tac_toe;
use tic_tac_toe::Player;

#[test]
fn extract_position_success() {
    let board = tic_tac_toe::empty_board();
    let result = tic_tac_toe::extract_position(board, "1 1".to_string());
    match result {
        Ok(x) => assert_eq!(x, 4),
        Err(error) => panic!(error),
    }
}

#[test]
fn extract_position_failure_boundary() {
    let board = tic_tac_toe::empty_board();
    let result = tic_tac_toe::extract_position(board, "9 8".to_string());
    match result {
        Ok(x) => panic!(x),
        Err(error) => assert_eq!(error, "Invalid position! You moron!"),
    }
}

#[test]
fn extract_position_failure_lack_of_value() {
    let board = tic_tac_toe::empty_board();
    let result = tic_tac_toe::extract_position(board, "0".to_string());
    match result {
        Ok(x) => panic!(x),
        Err(error) => assert_eq!(error, "Invalid position! You moron!"),
    }
}

#[test]
fn extract_position_failure_already_filled() {
    let mut board = tic_tac_toe::empty_board();
    board[8] = Player::Computer;
    let result = tic_tac_toe::extract_position(board, "2 2".to_string());
    match result {
        Ok(x) => panic!(x),
        Err(error) => assert_eq!(error, "Invalid position! You moron!"),
    }
}
#[test]
fn extract_position_failure_not_digit() {
    let board = tic_tac_toe::empty_board();
    let result = tic_tac_toe::extract_position(board, "e".to_string());
    match result {
        Ok(x) => panic!(x),
        Err(error) => assert_eq!(error, "Invalid position! You moron!"),
    }
}

#[test]
fn check_winner_empty_board() {
    let board = tic_tac_toe::empty_board();
    match tic_tac_toe::check_winner(board) {
        Player::Nobody => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn check_winner_human_wins_first_row() {
    let mut board = tic_tac_toe::empty_board();
    board[0] = Player::Human;
    board[1] = Player::Human;
    board[2] = Player::Human;
    match tic_tac_toe::check_winner(board) {
        Player::Human => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn check_winner_computer_wins_first_row() {
    let mut board = tic_tac_toe::empty_board();
    board[0] = Player::Computer;
    board[1] = Player::Computer;
    board[2] = Player::Computer;
    match tic_tac_toe::check_winner(board) {
        Player::Computer => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn check_winner_human_wins_second_row() {
    let mut board = tic_tac_toe::empty_board();
    board[3] = Player::Human;
    board[4] = Player::Human;
    board[5] = Player::Human;
    match tic_tac_toe::check_winner(board) {
        Player::Human => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn check_winner_computer_wins_second_row() {
    let mut board = tic_tac_toe::empty_board();
    board[3] = Player::Computer;
    board[4] = Player::Computer;
    board[5] = Player::Computer;
    match tic_tac_toe::check_winner(board) {
        Player::Computer => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn check_winner_human_wins_third_row() {
    let mut board = tic_tac_toe::empty_board();
    board[6] = Player::Human;
    board[7] = Player::Human;
    board[8] = Player::Human;
    match tic_tac_toe::check_winner(board) {
        Player::Human => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn check_winner_computer_wins_third_row() {
    let mut board = tic_tac_toe::empty_board();
    board[6] = Player::Computer;
    board[7] = Player::Computer;
    board[8] = Player::Computer;
    match tic_tac_toe::check_winner(board) {
        Player::Computer => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn check_winner_human_wins_first_column() {
    let mut board = tic_tac_toe::empty_board();
    board[0] = Player::Human;
    board[3] = Player::Human;
    board[6] = Player::Human;
    match tic_tac_toe::check_winner(board) {
        Player::Human => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn check_winner_computer_wins_first_column() {
    let mut board = tic_tac_toe::empty_board();
    board[0] = Player::Computer;
    board[3] = Player::Computer;
    board[6] = Player::Computer;
    match tic_tac_toe::check_winner(board) {
        Player::Computer => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn check_winner_human_wins_second_column() {
    let mut board = tic_tac_toe::empty_board();
    board[1] = Player::Human;
    board[4] = Player::Human;
    board[7] = Player::Human;
    match tic_tac_toe::check_winner(board) {
        Player::Human => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn check_winner_computer_wins_second_column() {
    let mut board = tic_tac_toe::empty_board();
    board[1] = Player::Computer;
    board[4] = Player::Computer;
    board[7] = Player::Computer;
    match tic_tac_toe::check_winner(board) {
        Player::Computer => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn check_winner_human_wins_third_column() {
    let mut board = tic_tac_toe::empty_board();
    board[2] = Player::Human;
    board[5] = Player::Human;
    board[8] = Player::Human;
    match tic_tac_toe::check_winner(board) {
        Player::Human => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn check_winner_computer_wins_third_column() {
    let mut board = tic_tac_toe::empty_board();
    board[2] = Player::Computer;
    board[5] = Player::Computer;
    board[8] = Player::Computer;
    match tic_tac_toe::check_winner(board) {
        Player::Computer => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn check_winner_human_wins_first_diagonal() {
    let mut board = tic_tac_toe::empty_board();
    board[0] = Player::Human;
    board[4] = Player::Human;
    board[8] = Player::Human;
    match tic_tac_toe::check_winner(board) {
        Player::Human => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn check_winner_computer_wins_first_diagonal() {
    let mut board = tic_tac_toe::empty_board();
    board[0] = Player::Computer;
    board[4] = Player::Computer;
    board[8] = Player::Computer;
    match tic_tac_toe::check_winner(board) {
        Player::Computer => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn check_winner_human_wins_second_diagonal() {
    let mut board = tic_tac_toe::empty_board();
    board[2] = Player::Human;
    board[4] = Player::Human;
    board[6] = Player::Human;
    match tic_tac_toe::check_winner(board) {
        Player::Human => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn check_winner_computer_wins_second_diagonal() {
    let mut board = tic_tac_toe::empty_board();
    board[2] = Player::Computer;
    board[4] = Player::Computer;
    board[6] = Player::Computer;
    match tic_tac_toe::check_winner(board) {
        Player::Computer => assert!(true),
        _ => assert!(false),
    }
}
