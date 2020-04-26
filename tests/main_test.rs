use tic_tac_toe;

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
