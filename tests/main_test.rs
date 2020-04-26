use tic_tac_toe;
use tic_tac_toe::Player;

#[test]
fn extract_position_success() {
    let result = tic_tac_toe::extract_position("1 1".to_string());
    match result {
        Ok(x) => assert_eq!(x, 4),
        Err(error) => panic!(error),
    }
}

#[test]
fn extract_position_failure_boundary() {
    let result = tic_tac_toe::extract_position("9 8".to_string());
    match result {
        Ok(x) => panic!(x),
        Err(error) => assert_eq!(error, "Invalid position! You moron!"),
    }
}

#[test]
fn extract_position_failure_lack_of_value() {
    let result = tic_tac_toe::extract_position("0".to_string());
    match result {
        Ok(x) => panic!(x),
        Err(error) => assert_eq!(error, "Invalid position! You moron!"),
    }
}

#[test]
fn extract_position_failure_not_digit() {
    let result = tic_tac_toe::extract_position("e".to_string());
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
// #[test]
// fn check_winner_human_wins_second_row() {
//     let board = [0, 0, 0, 1, 1, 1, 0, 0, 0];
//     match tic_tac_toe::check_winner(board) {
//         Player::Human => assert!(true),
//         _ => assert!(false),
//     }
// }

// #[test]
// fn check_winner_human_wins_third_row() {
//     let board = [0, 0, 0, 0, 0, 0, 1, 1, 1];
//     match tic_tac_toe::check_winner(board) {
//         Player::Human => assert!(true),
//         _ => assert!(false),
//     }
// }

// #[test]
// fn check_winner_human_wins_first_column() {
//     let board = [1, 0, 0, 1, 0, 0, 1, 0, 0];
//     match tic_tac_toe::check_winner(board) {
//         Player::Human => assert!(true),
//         _ => assert!(false),
//     }
// }

// #[test]
// fn check_winner_human_wins_second_column() {
//     let board = [0, 1, 0, 0, 1, 0, 0, 1, 0];
//     match tic_tac_toe::check_winner(board) {
//         Player::Human => assert!(true),
//         _ => assert!(false),
//     }
// }

// #[test]
// fn check_winner_human_wins_third_column() {
//     let board = [0, 0, 1, 0, 0, 1, 0, 0, 1];
//     match tic_tac_toe::check_winner(board) {
//         Player::Human => assert!(true),
//         _ => assert!(false),
//     }
// }
