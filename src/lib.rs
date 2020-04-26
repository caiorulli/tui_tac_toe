pub fn extract_position(input: String) -> Result<usize, String> {
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
