use std::collections::HashMap;

pub fn list_gifts(letter: &str) -> HashMap<String, u8> {
    let not_underscored = |item: &&str| !item.contains('_');

    count(letter.split(' ').filter(not_underscored).collect())
}

fn count(letter: Vec<&str>) -> HashMap<String, u8> {
    let mut result = HashMap::new();

    for item in letter {
        if let Some(value) = result.get_mut(item) {
            *value += 1;
        } else {
            result.insert(item.to_string(), 1);
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn list_gifts_test() {
        let data = "bici coche balon bici coche peluche";
        let result = list_gifts(data);

        assert_eq!(result.get("bici"), Some(&2));
        assert_eq!(result.get("coche"), Some(&2));
        assert_eq!(result.get("balon"), Some(&1));
        assert_eq!(result.get("peluche"), Some(&1));
    }

    #[test]
    fn list_gifts_with_extra_withespace_test() {
        let data = "bici coche balon  bici coche peluche";
        let result = list_gifts(data);

        assert_eq!(result.get("bici"), Some(&2));
        assert_eq!(result.get("coche"), Some(&2));
        assert_eq!(result.get("balon"), Some(&1));
        assert_eq!(result.get("peluche"), Some(&1));
    }

    #[test]
    fn list_gifts_with_underscore_test() {
        let data = "bici coche balon _playstation bici coche peluche";
        let result = list_gifts(data);

        assert_eq!(result.get("bici"), Some(&2));
        assert_eq!(result.get("coche"), Some(&2));
        assert_eq!(result.get("balon"), Some(&1));
        assert_eq!(result.get("peluche"), Some(&1));
    }
}
