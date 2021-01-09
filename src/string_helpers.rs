pub fn append_word(being_appended: &mut String, word: &str) {
    if !being_appended.is_empty() {
        being_appended.push(' ');
    }
    being_appended.push_str(word);
}
