pub fn distance_to_word_start(text: &str, cursor_pos: usize) -> usize {
    let mut distance = 0;

    for i in (0..cursor_pos).rev() {
        distance += 1;
        if text[i..=i].chars().all(char::is_whitespace) {
            break
        }
    }

    distance
}
