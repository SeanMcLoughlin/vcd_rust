pub fn append_word(being_appended: &mut String, word: &str) {
    if !being_appended.is_empty() {
        being_appended.push(' ');
    }
    being_appended.push_str(word);
}

pub fn is_cmd(word: &str) -> bool {
    word.starts_with('$')
}

pub fn vector_type_being_dumped(word: &str) -> bool {
    word.starts_with('b') || word.starts_with('r')
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

    #[test]
    fn test_is_cmd() {
        assert_eq!(true, is_cmd("$var"));
        assert_eq!(false, is_cmd("garbage"));
        assert_eq!(false, is_cmd(""));
    }

    #[test]
    fn test_vector_type_being_dumped() {
        assert_eq!(true, vector_type_being_dumped("b1010 &"));
        assert_eq!(true, vector_type_being_dumped("r42 {"));
        assert_eq!(false, vector_type_being_dumped("13 ("));
        assert_eq!(false, vector_type_being_dumped("1 %"));
    }
}
