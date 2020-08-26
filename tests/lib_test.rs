use tui_tac_toe;
use tui_tac_toe::Game;
use tui_tac_toe::Move;
use tui_tac_toe::Player;

#[test]
fn extract_position_success() {
    let result = Move::new(1, 1);
    match result {
        Ok(x) => assert_eq!(x, Move { x: 1, y: 1 }),
        Err(error) => panic!(error),
    }
}

#[test]
fn extract_position_failure_boundary() {
    let result = Move::new(9, 8);
    match result {
        Ok(x) => panic!(x),
        Err(error) => assert_eq!(error, "Invalid position! You moron!"),
    }
}

#[test]
fn extract_position_failure_already_filled() {
    let mut game = Game::new();
    game.board[8] = Player::Computer;
    let result = Move::new(2, 2).unwrap();
    assert_eq!(game.validate_move(&result), false);
}

#[test]
fn check_winner_empty_board() {
    let game = Game::new();
    match game.check_winner() {
        Player::Nobody => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn check_winner_human_wins_first_row() {
    let mut game = Game::new();
    game.make_move(Move { x: 0, y: 0 }, Player::Human);
    game.make_move(Move { x: 0, y: 1 }, Player::Human);
    game.make_move(Move { x: 0, y: 2 }, Player::Human);
    match game.check_winner() {
        Player::Human => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn check_winner_computer_wins_first_row() {
    let mut game = Game::new();
    game.make_move(Move { x: 0, y: 0 }, Player::Computer);
    game.make_move(Move { x: 0, y: 1 }, Player::Computer);
    game.make_move(Move { x: 0, y: 2 }, Player::Computer);
    match game.check_winner() {
        Player::Computer => assert!(true),
        _ => assert!(false),
    }
}
