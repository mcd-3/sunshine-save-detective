/// Adds whitespace padding to the left and/or the right of a string
/// 
/// Returns the padded string
pub fn padded_string(str: &str, padding_left: u16, padding_right: u16) -> String {
    let mut left = String::from("");
    let mut right: String = String::from("");

    for i in 0..padding_left {
        left.insert(i as usize, ' ');
    }

    for i in 0..padding_right {
        right.insert(i as usize, ' ');
    }

    format!{"{}{}{}", left, str, right}
}