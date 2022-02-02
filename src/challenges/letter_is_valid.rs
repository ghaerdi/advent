pub fn letter_is_valid(letter: &str) -> bool {
    const INVALID_BRACKETS: [&str; 5] = ["[", "]", "{", "}", "()"];
    let has_not_invalid_brackets = |b: &&str| !letter.contains(b);

    INVALID_BRACKETS.iter().all(has_not_invalid_brackets)
        && letter.find('(') < letter.find(')')
        && letter.matches('(').count() == letter.matches(')').count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn letter_is_valid_test() {
        assert!(letter_is_valid("bici coche (balón) bici coche peluche"));
        assert!(letter_is_valid("(muñeca) consola bici"));
    }

    #[test]
    fn letter_is_not_valid_brackets() {
        assert!(!letter_is_valid("peluche (bici [coche) bici coche balón"));
        assert!(!letter_is_valid("(peluche {) bici"));
    }

    #[test]
    fn letter_is_not_valid_unclosed_parenthreses() {
        assert!(!letter_is_valid("bici coche (balón bici coche"));
    }

    #[test]
    fn letter_is_not_valid_void_parentheses() {
        assert!(!letter_is_valid("() bici"));
        assert!(!letter_is_valid("(()) bici"));
    }

    #[test]
    fn letter_is_not_valid_inverted_parentheses() {
        assert!(!letter_is_valid("bici coche )balón( bici coche peluche"));
    }
}
