pub fn pangram(letter: &str) -> bool {
    const ABC: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let letter = letter.to_lowercase();
    let mut splitted: Vec<char> = letter.chars().collect();
    splitted.sort_unstable();
    splitted.dedup();
    splitted.retain(|c| ABC.contains(c));

    splitted.len() == ABC.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pangram_test() {
        assert!(pangram("How vexingly quick daft zebras jump!"));
        assert!(pangram("Sphinx of black quartz, judge my vow."));
        assert!(pangram("The quick brown fox jumps over a lazy dog."));
    }
}
