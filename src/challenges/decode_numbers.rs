use std::collections::HashMap;

pub fn decode_numbers(symbols: &str) -> Option<u32> {
    let symbols: Vec<char> = symbols.chars().collect();
    let meaning = symbols_meaning();
    let mut result: i32 = 0;

    if !symbols.iter().all(|s| meaning.get(s) != None) {
        return None;
    };

    symbols.iter().enumerate().for_each(|(i, symbol)| {
        let n = meaning.get(&symbol).unwrap_or(&0);

        if i + 1 != symbols.len() && n < meaning.get(&symbols[i + 1]).unwrap_or(&0) {
            result -= n;
        } else {
            result += n;
        }
    });

    return Some(result as u32);
}

fn symbols_meaning() -> HashMap<char, i32> {
    let mut meaning = HashMap::new();

    const SYMBOLS: [char; 5] = ['.', ',', ':', ';', '!'];
    const NUMBERS: [i32; 5] = [1, 5, 10, 50, 100];
    SYMBOLS.iter().enumerate().for_each(|(i, &v)| {
        meaning.insert(v, NUMBERS[i]);
    });

    return meaning;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decode_numbers_test() {
        assert_eq!(decode_numbers(".,"), Some(4));
        assert_eq!(decode_numbers(",..."), Some(8));
        assert_eq!(decode_numbers(".;!"), Some(49));
    }

    fn decode_numbers_invalid_test() {
        assert_eq!(decode_numbers(",;W"), None);
    }
}
