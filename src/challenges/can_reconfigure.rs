use std::collections::HashMap;

pub fn can_reconfigure(from: &str, to: &str) -> bool {
    let from: Vec<char> = from.chars().collect();
    let to: Vec<char> = to.chars().collect();
    if from.len() != to.len() {
        return false;
    }

    {
        let mut sorted_f: Vec<char> = from.clone();
        let mut sorted_t: Vec<char> = to.clone();
        sorted_f.sort_unstable();
        sorted_t.sort_unstable();
        sorted_f.dedup();
        sorted_t.dedup();

        if sorted_f.len() != sorted_t.len() {
            return false;
        }
    }

    let mut chars: HashMap<char, Vec<char>> = HashMap::new();

    for (i, &value) in from.iter().enumerate() {
        if let Some(c) = chars.get_mut(&value) {
            if c[0] != to[i] {
                return false;
            }
            c.push(to[i]);
        } else {
            chars.insert(value, vec![to[i]]);
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_reconfigure_test() {
        assert!(can_reconfigure("BAL", "LIB"));
        assert!(can_reconfigure("XBOX", "XOBX"));
        assert!(!can_reconfigure("CON", "JUU"));
        assert!(!can_reconfigure("XBOX", "XXBO"));
        assert!(!can_reconfigure("MMM", "MID"));
        assert!(!can_reconfigure("AA", "MID"));
    }
}
