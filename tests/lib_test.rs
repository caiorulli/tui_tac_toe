use tui_tac_toe;
use tui_tac_toe::{Game, Winner};

#[test]
fn check_winner_empty_board() {
    let game = Game::new();
    assert_eq!(game.check_winner(), Winner::Nobody);
}

#[test]
fn check_winner_human_wins() {
    let mut game = Game::new();
    game.apply_move((0, 0));
    game.apply_move((1, 0));
    game.apply_move((0, 1));
    game.apply_move((1, 1));
    game.apply_move((0, 2));
    assert_eq!(game.check_winner(), Winner::Human);
}

#[test]
fn check_winner_computer_wins() {
    let mut game = Game::new();
    game.apply_move((0, 0));
    game.apply_move((2, 2));
    game.apply_move((1, 1));
    game.apply_move((1, 2));
    game.apply_move((0, 1));
    game.apply_move((0, 2));
    assert_eq!(game.check_winner(), Winner::Computer);
}

#[test]
fn check_winner_draw() {
    let mut game = Game::new();
    game.apply_move((0, 0));
    game.apply_move((0, 1));
    game.apply_move((0, 2));
    game.apply_move((1, 1));
    game.apply_move((1, 0));
    game.apply_move((2, 0));
    game.apply_move((1, 2));
    game.apply_move((2, 2));
    game.apply_move((2, 1));
    assert_eq!(game.check_winner(), Winner::Draw);
}
