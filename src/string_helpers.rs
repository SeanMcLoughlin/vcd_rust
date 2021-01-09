pub fn append_word(being_appended: &mut String, word: &str) {
    if !being_appended.is_empty() {
        being_appended.push(' ');
    }
    being_appended.push_str(word);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_word() {
        let mut being_appended = "".to_string();
        append_word(&mut being_appended, "Hello");
        assert_eq!(being_appended, "Hello".to_string());
        append_word(&mut being_appended, "World!");
        assert_eq!(being_appended, "Hello World!".to_string());
    }
}
